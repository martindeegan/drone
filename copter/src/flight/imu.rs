use hardware::sensors::{MultiSensorData,start_sensors,SensorInput};

use std::sync::mpsc::{Sender, Receiver, channel};
use std::f32::consts::PI;
use na::{Vector3,Rotation};

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

    north_reading: MultiSensorData,
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
            north_reading: MultiSensorData::zeros(),
        };
        #[cfg(not(test))]
        for i in 0..10 {
            imu.read_data();
        }
        imu.north_reading = imu.last_magnetic_reading;
        imu
    }

    pub fn read_data(&mut self) {
        match self.input_rx.recv() {
            Ok(input) => {
                println!("Received data!");
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
        self.compute_position();
    }

    fn compute_attitude(&mut self, dt: f32) {
        let alpha = 1.0;
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
        let roll = A_y.atan2(A_z) * RADIAN_TO_DEGREES;
        // Rotation about the y axis
        let pitch = A_x.atan2((A_y * A_y + A_z * A_z).sqrt()) * RADIAN_TO_DEGREES * -1.0;

        let estimated_attitude = Attitude {
            x: roll * alpha + self.last_attitude.x * (1.0 - alpha),
            y: pitch * alpha + self.last_attitude.y * (1.0 - alpha),
            z: 0.0,
        };

        let yaw = compute_yaw(estimated_attitude, self.north_reading, self.last_magnetic_reading);
        println!("yaw: {:?}", yaw);
        
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

fn linearalgebrize(vec3: MultiSensorData) -> Vector3<f32> {
    Vector3::new(vec3.x, vec3.y, vec3.z)
}

fn compute_yaw(attitude: Attitude, current_magnetic_reading: MultiSensorData, original_magnetic_reading: MultiSensorData) -> f32 {
    let attitude_rad = attitude * DEGREE_TO_RADIAN;
    let m_prime_x = original_magnetic_reading.x * attitude_rad.y.cos() + original_magnetic_reading.y * attitude_rad.x.sin() * attitude_rad.y.sin() + original_magnetic_reading.z * attitude_rad.x.cos() * attitude_rad.y.sin();
    let m_prime_y = original_magnetic_reading.y * attitude_rad.x.cos() - original_magnetic_reading.z * attitude_rad.x.sin();
    (-current_magnetic_reading.x * m_prime_y + current_magnetic_reading.y * m_prime_x).atan2(current_magnetic_reading.y * m_prime_y + current_magnetic_reading.x * m_prime_x) * RADIAN_TO_DEGREES
}

fn compute_yaw_robert(attitude: Attitude, m: MultiSensorData, original_magnetic_reading: MultiSensorData) -> f32 {
    let attitude_rad = attitude * DEGREE_TO_RADIAN;
    let x_y_magnitude = (original_magnetic_reading.x * original_magnetic_reading.x + original_magnetic_reading.y * original_magnetic_reading.y);
    let m_prime_x = original_magnetic_reading.x / x_y_magnitude;
    let m_prime_y = original_magnetic_reading.y / x_y_magnitude;

    let sin_theta = attitude.y.sin();
    let cos_theta = attitude.y.cos();
    let sin_phi = attitude.y.sin();
    let cos_phi = attitude.y.sin();

    let top = m.x * m_prime_y * cos_theta + m.y * (m_prime_y * sin_phi * sin_theta - m_prime_x * cos_phi) + m.z * (m_prime_y * cos_phi * sin_theta + m_prime_x * sin_phi);
    let bottom = m.x * m_prime_x * cos_theta + m.y * (m_prime_x * sin_phi * sin_theta + m_prime_y * cos_phi) + m.z * ( m_prime_x * cos_phi * sin_theta - m_prime_y * sin_phi);
    top.atan2(bottom) * RADIAN_TO_DEGREES

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
