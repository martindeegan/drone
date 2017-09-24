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
const DEGREE_TO_RADIAN: f32 = 1.0 / RADIAN_TO_DEGREES;

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
    pub north_reading: MultiSensorData,

    pub start_altitude: f32,
    pub sea_level_pressure: f32,
    pub yaw_offset: f32
}



impl IMU {
    pub fn new(sensor_stream: Receiver<SensorInput>) -> IMU {
        let mut imu = IMU {
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
            start_altitude: 0.0,
            sea_level_pressure: 101.325,
            north_reading: MultiSensorData::zeros(),
            yaw_offset: 0.0
        };
        #[cfg(not(test))]
        for i in 0..10 {
            imu.read_data();
            imu.compute_intertial_reference(1.0);
        }

        imu.north_reading = imu.last_magnetic_reading;
        imu
    }

    pub fn read_data(&mut self) {
        match self.input_rx.recv() {
            Ok(input) => {
                self.last_angular_rate = input.angular_rate;
                self.last_acceleration = input.acceleration;
                if input.magnetic_reading.is_some() {
                    self.last_magnetic_reading = input.magnetic_reading.unwrap();
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
        self.compute_position(dt);
    }

    fn compute_attitude(&mut self, dt: f32) {
        let alpha = 0.02;
        // Trig calculations
        // let s_roll = self.last_attitude.x.sin(); let c_roll = self.last_attitude.x.cos();
        // let s_pitch = self.last_attitude.y.sin(); let c_pitch = self.last_attitude.y.cos();

        // Estimate
        self.last_attitude = self.last_attitude + self.last_angular_rate * dt;

        // Compute true values
        let A_x = self.last_acceleration.x;
        let A_y = self.last_acceleration.y;
        let A_z = self.last_acceleration.z;

        // Rotation about the x axis
        let mut roll = A_y.atan2(A_z) * RADIAN_TO_DEGREES;
        self.last_attitude.x = roll * alpha + self.last_attitude.x * (1.0 - alpha);
        roll = self.last_attitude.x * DEGREE_TO_RADIAN;

        // Rotation about the y axis
        let gz_2 = A_y * (roll).sin() + A_z * (roll).cos();
        let mut pitch = A_x.atan2(gz_2) * -1.0 * RADIAN_TO_DEGREES;
        self.last_attitude.y = pitch * alpha + self.last_attitude.y * (1.0 - alpha);
        pitch = self.last_attitude.y * DEGREE_TO_RADIAN;

        roll *= -1.0;

        let yaw_alpha = 0.5;
        let mag_x_comp = self.last_magnetic_reading.x * pitch.cos() + self.last_magnetic_reading.z * pitch.sin();
        let mag_y_comp = self.last_magnetic_reading.x * roll.sin() * pitch.sin() +
                         self.last_magnetic_reading.y * roll.cos() -
                         self.last_magnetic_reading.z * roll.sin() * pitch.cos();

        let yaw = mag_y_comp.atan2(mag_x_comp) * RADIAN_TO_DEGREES - self.yaw_offset;
        self.last_attitude.z = yaw * yaw_alpha + self.last_attitude.z * (1.0 - yaw_alpha);
    }

    fn compute_altitude(&mut self) {
        let pressure = self.last_pressure * 1000.0;
        let sea_level_pa = 101.325 * 1000.0;
        self.last_altitude = 44330. * (1. - (pressure / sea_level_pa).powf(0.1903)) - self.start_altitude;
    }

    fn compute_position(&mut self, dt: f32) {

        // println!("{},{},{}", self.relative_location.x, self.relative_location.y, self.relative_location.z);
    }
}

fn linearalgebraize(vec3: MultiSensorData) -> Vector3<f32> {
    Vector3::new(vec3.x, vec3.y, vec3.z)
}

fn delinearalgebraize(vec3: Vector3<f32>) -> MultiSensorData {
    MultiSensorData { x: vec3.data[0], y: vec3.data[1], z: vec3.data[2] }
}

// fn compute_true_yaw(mut M: MultiSensorData, mut start_magnetic_reading: MultiSensorData, attitude: Attitude) -> f32 {
//     M = M * DEGREE_TO_RADIAN;
//     start_magnetic_reading = start_magnetic_reading * DEGREE_TO_RADIAN;
//     let sr = attitude.x.sin(); let cr = attitude.x.cos();
//     let sp = attitude.y.sin(); let cp = attitude.y.cos();
//
//     let roll_pitch_rotation_matrix = Matrix3::new(cp,  sr*sp,  cr*sp,
//                                                   0.0,    cr,   -1.0*sr,
//                                                   -sp, sr*cp,  cr*cp);
//     let M_n = linearalgebraize(start_magnetic_reading);
//     let M_ = roll_pitch_rotation_matrix * M_n;
//
//     let A = (M.y*M_.x - M.x*M_.y);
//     let B = (M.y*M_.y + M.x*M_.y);
//     let yaw = A.atan2(B) * RADIAN_TO_DEGREES;
//     // R_3 * M = M_
//     yaw
// }

fn world_space_to_drone_space(vector: MultiSensorData, attitude: Attitude) -> MultiSensorData {
    let navec3 = linearalgebraize(vector);
    // Compute Trig values for efficiency
    let sr = attitude.x.sin(); let cr = attitude.x.cos();
    let sp = attitude.y.sin(); let cp = attitude.y.cos();
    let sy = attitude.z.sin(); let cy = attitude.z.cos();

    let rotation_matrix = Matrix3::new(cp*cy,   -1.0*cr*sy + sr*sp*cy,    sr*sy + cr*sp*cy,
                                       cp*sy,      cr*cy + sr*sp*sy,   -1.0*sr *cy + cr*sp*sy,
                                       -1.0*sp,         sr*cp,                 cr*cp);

    delinearalgebraize(rotation_matrix * navec3)
}

fn drone_space_to_world_space(vector: MultiSensorData, attitude: Attitude) -> MultiSensorData {
    let navec3 = linearalgebraize(vector);

    let sr = attitude.x.sin(); let cr = attitude.x.cos();
    let sp = attitude.y.sin(); let cp = attitude.y.cos();
    let sy = attitude.z.sin(); let cy = attitude.z.cos();

    let rotation_matrix = Matrix3::new(       cp*cy,                   cp*sy,          -1.0*sp,
                                       -1.0*cr*sy + sr*sp*cy,    cr*cy + sr*sp*sy,     sr*cp,
                                         sr*sy + cr*sp*cy,   -1.0*sr *cy + cr*sp*sy,   cr*cp);

    delinearalgebraize(rotation_matrix * navec3)
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
    fn testFlushAllSensorData() {
        let (tx, rx) : (Sender<SensorInput>, Receiver<SensorInput>) = channel();
        let mut imu = IMU::new(rx);
        let data1 = createTestData(1.0, 1.0, 1.0, 1.0, 1.0, 1.0);
        let data2 = createTestData(2.0, 2.0, 2.0, 2.0, 2.0, 2.0);
        tx.send(data1);
        tx.send(data2);
        imu.read_data();

        assert_eq!(imu.last_angular_rate.x, 2.0);
        assert_eq!(imu.last_angular_rate.y, 2.0);
        assert_eq!(imu.last_angular_rate.z, 2.0);
    }

    #[test]
    fn testMovingGyroData () {
        let (tx, rx) : (Sender<SensorInput>, Receiver<SensorInput>) = channel();
        let mut imu = IMU::new(rx);

        let alpha = 0.02;
        let first_tilt = 20.0;
        let data1 = createTestData(first_tilt, first_tilt, first_tilt, 0.0, 0.0, 1.0);

        tx.send(data1);
        imu.read_data();
        imu.compute_intertial_reference(1.0);
        assert_eq!(imu.last_attitude.x, first_tilt * (1.0 - alpha));
        assert_eq!(imu.last_attitude.y, first_tilt * (1.0 - alpha));
        assert_eq!(imu.last_attitude.z, first_tilt * (1.0 - alpha));

        // Next data point. Even though we have tilted first_tilt deg, we are still pointing down according to the accelerometer readings. This isn't realistic.  Go figure!
        let data2 = createTestData(0.5, 0.5, 0.5, 0.0, 0.0, 1.0);
        tx.send(data2);
        imu.read_data();
        imu.compute_intertial_reference(0.5);
        let new_angle = (first_tilt * (1.0 - alpha) + (0.5 * 0.5)) * (1.0 - alpha);

        assert_eq!(imu.last_attitude.x, new_angle);
        assert_eq!(imu.last_attitude.y, new_angle);
        // TODO: Test this value.
        // assert_eq!(imu.last_attitude.z, -2.3946);
    }

    #[test]
    fn testMovingAccelerometerData () {
        let (tx, rx) : (Sender<SensorInput>, Receiver<SensorInput>) = channel();
        let mut imu = IMU::new(rx);

        let alpha = 0.02;
        let tilt = 0.0;
        let data1 = createTestData(tilt, tilt, tilt, 0.0, 0.0, 1.0);

        tx.send(data1);
        imu.read_data();
        imu.compute_intertial_reference(1.0);
        assert_eq!(imu.last_attitude.x, 0.0);
        assert_eq!(imu.last_attitude.y, 0.0);
        assert_eq!(imu.last_attitude.z, 0.0);

        // All of a suden, we are flipped 90 degrees!
        let data2 = createTestData(tilt, tilt, tilt, -1.0, 0.0, 0.0);
        tx.send(data2);
        imu.read_data();
        imu.compute_intertial_reference(0.5);

        assert_eq!(imu.last_attitude.x, 0.0);
        assert_eq!(imu.last_attitude.y, 90.0 * alpha);
        assert_eq!(imu.last_attitude.z, 0.0);

        imu.compute_intertial_reference(0.5);

        assert_eq!(imu.last_attitude.x, 0.0);
        assert_eq!(imu.last_attitude.y, (90.0 * alpha) * (1.0 - alpha) + 90.0 * alpha);
        assert_eq!(imu.last_attitude.z, 0.0);

    }

    #[test]
    fn testYawComputationSimple () {
        let original_magnetic_reading = MultiSensorData {
            x: 1.0,
            y: 0.0,
            z: 0.0
        };

        let new_magnetic_reading = MultiSensorData {
            x: 1.0,
            y: 0.0,
            z: 0.0
        };

        let current_rotation = Attitude {
            x: 0.0,
            y: 0.0,
            z: 0.0 // unused
        };

        assert_eq!(compute_yaw(current_rotation, new_magnetic_reading, original_magnetic_reading), 0.0);
    }

    #[test]
    fn testYawComputationRotation () {
        let original_magnetic_reading = MultiSensorData {
            x: 1.0,
            y: 0.0,
            z: 0.0
        };

        let current_rotation = Attitude {
            x: 0.0,
            y: 0.0,
            z: 0.0 // unused
        };

        let new_magnetic_reading1= MultiSensorData {
            x: 0.0,
            y: 1.0,
            z: 0.0
        };

        let new_magnetic_reading2 = MultiSensorData {
            x: 0.0,
            y: -1.0,
            z: 0.0
        };

        let new_magnetic_reading3 = MultiSensorData {
            x: -1.0,
            y: 0.0,
            z: 0.0
        };

        assert_eq!(compute_yaw(current_rotation, new_magnetic_reading1, original_magnetic_reading), 90.0);
        assert_eq!(compute_yaw(current_rotation, new_magnetic_reading2, original_magnetic_reading), -90.0);
        assert_eq!(compute_yaw(current_rotation, new_magnetic_reading3, original_magnetic_reading).abs(), 180.0);
    }

    #[test]
    fn testYawComputationRotationWithZ () {
        let original_magnetic_reading = MultiSensorData {
            x: 1.0,
            y: 0.0,
            z: 0.1
        };

        let current_rotation = Attitude {
            x: 0.0,
            y: 0.0,
            z: 0.0 // unused
        };

        let new_magnetic_reading1= MultiSensorData {
            x: 0.0,
            y: 1.0,
            z: 0.1
        };

        let new_magnetic_reading2 = MultiSensorData {
            x: 0.0,
            y: -1.0,
            z: 0.1
        };

        let new_magnetic_reading3 = MultiSensorData {
            x: -1.0,
            y: 0.0,
            z: 0.1
        };

        let new_magnetic_reading4 = MultiSensorData {
            x: 1.0,
            y: 1.0,
            z: 0.1
        };

        assert_eq!(compute_yaw(current_rotation, new_magnetic_reading1, original_magnetic_reading), 90.0);
        assert_eq!(compute_yaw(current_rotation, new_magnetic_reading2, original_magnetic_reading), -90.0);
        assert_eq!(compute_yaw(current_rotation, new_magnetic_reading3, original_magnetic_reading), 180.0);
        assert_eq!(compute_yaw(current_rotation, new_magnetic_reading4, original_magnetic_reading), 45.0);
    }

    #[test]
    fn testYawComputationRotation2 () {
        let original_magnetic_reading = MultiSensorData {
            x: 1.0,
            y: 0.0,
            z: 0.1
        };

        let current_rotation = Attitude {
            x: 45.0,
            y: 0.0,
            z: 0.0 // unused
        };

        let new_magnetic_reading1 = MultiSensorData {
            x: 0.0,
            y: 1.0,
            z: 0.1
        };

        assert_eq!(compute_yaw(current_rotation, new_magnetic_reading1, original_magnetic_reading), 90.0);
    }

    // TODO: Ideally, have an integration test that uses real world values.
}
