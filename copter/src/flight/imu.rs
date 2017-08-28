use hardware::sensors::{MultiSensorData,start_sensors,SensorInput};

use std::sync::mpsc::{Sender, Receiver, channel};
use std::f32::consts::PI;
use na::core::{Matrix3,Vector3,Matrix2};
// x: roll
// y: pitch
// z: yaw
pub type Attitude = MultiSensorData;

// Position in space relative to the tracking start
// x: North/South
// y: East/West
// z: Altitude
pub type Position = MultiSensorData;

pub fn magnitude(vector: MultiSensorData) -> f32 {
    (vector.x * vector.x + vector.y * vector.y + vector.z * vector.z).sqrt()
}

const RADIAN_TO_DEGREES: f32 = 180.0 / PI;

pub struct AttitudeOutput {
    current_attitude: Option<MultiSensorData>,
    current_angular_rate: Option<MultiSensorData>,
    current_altitude: Option<f32>
}

pub struct IMU {
    // Input
    input_rx: Receiver<SensorInput>,

    // Previous data
    pub last_angular_rate: MultiSensorData,
    pub last_acceleration: MultiSensorData,
    pub last_magnetic_reading: MultiSensorData,
    pub last_temperature: f32,
    pub last_pressure: f32,

    // Previous states
    pub last_attitude: Attitude,
    pub last_position: Position,
    pub last_altitude: f32,

    // Dead reckoning
    tracking: bool,
    pub relative_location: MultiSensorData,

    north_reading: Option<MultiSensorData>,
}



impl IMU {
    pub fn new(sensor_stream: Receiver<SensorInput>) -> IMU {
        IMU {
            input_rx: sensor_stream,
            last_angular_rate: MultiSensorData::zeros(),
            last_acceleration: MultiSensorData::zeros(),
            last_magnetic_reading: MultiSensorData::zeros(),
            last_temperature: 0.0,
            last_pressure: 0.0,
            last_attitude: Attitude::zeros(),
            last_position: Position::zeros(),
            last_altitude: 0.0,
            tracking: false,
            relative_location: MultiSensorData::zeros(),
            north_reading: None,
        }
    }

    pub fn read_data(&mut self) {
        match self.input_rx.recv() {
            Ok(input) => {
                self.last_angular_rate = input.angular_rate;
                self.last_acceleration = input.acceleration;
                if input.magnetic_reading.is_some() {
                    self.last_magnetic_reading = input.magnetic_reading.unwrap();
                    if self.north_reading.is_none() {
                        self.north_reading = Some(self.last_magnetic_reading);
                    }
                }
                self.last_pressure = input.pressure;
            },
            Err(e) => {}
        }

        loop {
            match self.input_rx.try_recv() {
                Ok(input) => {
                    self.last_angular_rate = input.angular_rate;
                    self.last_acceleration = input.acceleration;
                    if input.magnetic_reading.is_some() {
                        self.last_magnetic_reading = input.magnetic_reading.unwrap();
                    }
                    self.last_pressure = input.pressure;
                },
                Err(e) => { break; }
            }
        }
    }

    pub fn compute_intertial_reference(&mut self, dt: f32) {
        self.compute_attitude(dt);
        self.compute_altitude();
        self.compute_position();
    }

    fn compute_attitude(&mut self, dt: f32) {

        // Trig calculations
        let s_roll = self.last_attitude.x.sin(); let c_roll = self.last_attitude.x.cos();
        let s_pitch = self.last_attitude.y.sin(); let c_pitch = self.last_attitude.y.cos();

        // Estimate
        self.last_attitude = self.last_attitude + self.last_angular_rate * dt;

        // Compute true values
        let A_x = self.last_acceleration.x;
        let A_y = self.last_acceleration.y;
        let A_z = self.last_acceleration.z;
        let roll = A_y.atan2(A_z) * RADIAN_TO_DEGREES;
        let pitch = A_x.atan2((A_y * A_y + A_z * A_z).sqrt()) * RADIAN_TO_DEGREES * -1.0;

        // let M_x = self.last_magnetic_reading.x; let M_y = self.last_magnetic_reading.y; let M_z = self.last_magnetic_reading.z;
        // let mx_ = self.north_reading.x / (self.north_reading.x * self.north_reading.x + self.north_reading.y * self.north_reading.y);
        // let my_ = self.north_reading.y / (self.north_reading.x * self.north_reading.x + self.north_reading.y * self.north_reading.y);
        //
        // let yaw_num = M_x * my_ * c_pitch + M_y * (my_ * s_pitch * s_roll - mx_ * c_roll) + M_z * (my_ * c_roll * s_pitch + mx_ * s_roll);
        // let yaw_den = M_x * mx_ * c_pitch + M_y * (mx_ * s_roll * s_pitch + my_ * c_roll) + M_z * (mx_ * c_roll * s_pitch + my_ * s_roll);
        //
        // let yaw = yaw_num.atan2(yaw_den) * RADIAN_TO_DEGREES * -1.0;

        if self.north_reading.is_some() {
            compute_true_yaw(self.last_magnetic_reading, self.north_reading.unwrap(), self.last_attitude);
        }
        // println!("Last angular rate: {:?}, {:?}, {:?}", self.last_angular_rate.x, self.last_angular_rate.y, self.last_angular_rate.z);

        // let magnitude = magnitude(self.last_acceleration);
        // let pitch = (self.last_acceleration.x / magnitude).asin() * RADIAN_TO_DEGREES;
        // let roll = self.last_acceleration.y.atan2(self.last_acceleration.z) * RADIAN_TO_DEGREES;

        // let A_yaw = self.last_attitude.y.cos() * self.last_magnetic_reading.x -
        //           self.last_attitude.x.sin() * self.last_attitude.y.sin() * self.last_magnetic_reading.y -
        //           self.last_attitude.x.cos() * self.last_attitude.y.sin() * self.last_magnetic_reading.z;
        // let B_yaw = self.last_attitude.x.cos() * self.last_magnetic_reading.y - self.last_attitude.x.sin() * self.last_magnetic_reading.z;
        // let yaw = (self.north_reading.x * (A_yaw + B_yaw)).atan2(self.north_reading.y * (A_yaw - B_yaw)) * RADIAN_TO_DEGREES;

        let alpha = 0.02;
        self.last_attitude = self.last_attitude * (1.0 - alpha) + Attitude { x: roll, y: pitch, z: yaw } * alpha;
    }

    fn compute_altitude(&mut self) {
        let pressure = self.last_pressure * 1000.0;
        let sea_level_pa = 101.325 * 1000.0;
        self.last_altitude = 44330. * (1. - (pressure / sea_level_pa).powf(0.1903));
    }

    fn compute_position(&mut self) {

    }
}

fn linearalgebraize(vec3: MultiSensorData) -> Vector3<f32> {
    Vector3::new(vec3.x, vec3.y, vec3.z)
}

fn compute_true_yaw(M: MultiSensorData, start_magnetic_reading: MultiSensorData, attitude: Attitude) -> f32 {
    let sr = attitude.x.sin(); let cr = attitude.x.cos();
    let sp = attitude.y.sin(); let cp = attitude.y.cos();

    let roll_pitch_rotation_matrix = Matrix3::new(cp,  sr*sp,  cr*sp,
                                                  0.0,    cr,   -1.0*sr,
                                                  -sp, sr*cp,  cr*cp);
    let M_n = linearalgebraize(start_magnetic_reading);
    let M_ = roll_pitch_rotation_matrix * M_n;

    let A = (M.y*M_.x - M.x*M_.y);
    let B = (M.y*M_.y + M.x*M_.y);
    let yaw = A.atan2(B) * RADIAN_TO_DEGREES;
    // R_3 * M = M_
    println!("{:?}", yaw);
    yaw
}

fn world_space_to_drone_space(vector: Vector3<f32>, attitude: Attitude) -> Vector3<f32> {
    // Compute Trig values for efficiency
    let sr = attitude.x.sin(); let cr = attitude.x.cos();
    let sp = attitude.y.sin(); let cp = attitude.y.cos();
    let sy = attitude.z.sin(); let cy = attitude.z.cos();

    let rotation_matrix = Matrix3::new(cp*cy,   -1.0*cr*sy + sr*sp*cy,    sr*sy + cr*sp*cy,
                                       cp*sy,      cr*cy + sr*sp*sy,   -1.0*sr *cy + cr*sp*sy,
                                       -1.0*sp,         sr*cp,                 cr*cp);

    rotation_matrix * vector
}

fn drone_space_to_world_space(vector: Vector3<f32>, attitude: Attitude) -> Vector3<f32> {
    let sr = attitude.x.sin(); let cr = attitude.x.cos();
    let sp = attitude.y.sin(); let cp = attitude.y.cos();
    let sy = attitude.z.sin(); let cy = attitude.z.cos();

    let rotation_matrix = Matrix3::new(       cp*cy,                   cp*sy,          -1.0*sp,
                                       -1.0*cr*sy + sr*sp*cy,    cr*cy + sr*sp*sy,     sr*cp,
                                         sr*sy + cr*sp*cy,   -1.0*sr *cy + cr*sp*sy,   cr*cp);

    rotation_matrix * vector
}

#[cfg(test)]
mod tests {
    use super::*;

    fn createTestData(gx: f32, gy: f32, gz: f32, ax: f32, ay: f32, az: f32) -> SensorInput {
        SensorInput {
            angular_rate: MultiSensorData {
                x: gx,
                y: gy,
                z: gz
            },
            acceleration: MultiSensorData {
                x: ax,
                y: ay,
                z: az
            },
            magnetic_reading: None,
            temperature: 0.0,
            pressure: 0.0
        }
    }

    #[test]
    fn testSimpleData () {
        let (tx, rx) : (Sender<SensorInput>, Receiver<SensorInput>) = channel();
        let mut imu = IMU::new(rx);
        let data = SensorInput {
            angular_rate: MultiSensorData::zeros(),
            acceleration: MultiSensorData {
                x: 0.0,
                y: 0.0,
                z: 1.0
            },
            magnetic_reading: None,
            temperature: 0.0,
            pressure: 0.0
        };
        tx.send(data);
        imu.read_data();
        imu.compute_intertial_reference(1.0);
        assert_eq!(imu.last_attitude.x, 0.0);
        assert_eq!(imu.last_attitude.y, 0.0);
        assert_eq!(imu.last_attitude.z, 0.0);
    }

    #[test]
    fn testMovingData () {
        let (tx, rx) : (Sender<SensorInput>, Receiver<SensorInput>) = channel();
        let mut imu = IMU::new(rx);
        //
        let data1 = createTestData(-1.09, -0.25, 4.64, 0.19, -0.01, 0.78);
        let data2 = createTestData(-0.17,  0.26, 4.78, 0.2, 0.0, 0.78);
        let data3 = createTestData(-0.67, 0.12, 4.86, 0.2, -0.01, 0.74);
        let data4 = createTestData(-0.8, 0.29, 4.78,  0.21, 0, 0.76);
        tx.send(data);
        imu.read_data();
        imu.compute_intertial_reference(3.39);
        tx.send(data2);
        imu.read_data();
        imu.compute_intertial_reference(0.3);
        tx.send(data3);
        imu.read_data();
        imu.compute_intertial_reference(0.1);
        tx.send(data4);
        imu.read_data();
        imu.compute_intertial_reference(0.1);

        assert_eq!(imu.last_attitude.x, 0.0);
        assert_eq!(imu.last_attitude.y, 0.0);
        assert_eq!(imu.last_attitude.z, 0.0);
    }
}
