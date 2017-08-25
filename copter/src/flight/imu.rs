use hardware::sensors::{MultiSensorData,start_sensors,SensorInput};

use std::sync::mpsc::Receiver;
use std::f32::consts::PI;

// x: roll
// y: pitch
// z: yaw
pub type Attitude = MultiSensorData;

// Position in space relative to the tracking start
// x: North/South
// y: East/West
// z: Altitude
pub type Position = MultiSensorData;

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
    last_angular_rate: MultiSensorData,
    last_acceleration: MultiSensorData,
    last_magnetic_reading: MultiSensorData,
    last_temperature: f32,
    last_pressure: f32,

    // Previous states
    last_attitude: Attitude,
    last_position: Position,

    // Dead reckoning
    tracking: bool,
    relative_location: MultiSensorData,

    north_reading: MultiSensorData
}

impl IMU {
    pub fn new() -> IMU {
        let input_rx = start_sensors();
        IMU {
            input_rx: start_sensors(),
            last_angular_rate: MultiSensorData::zeros(),
            last_acceleration: MultiSensorData::zeros(),
            last_magnetic_reading: MultiSensorData::zeros(),
            last_temperature: 0.0,
            last_pressure: 0.0,
            last_attitude: Attitude::zeros(),
            last_position: Position::zeros(),
            tracking: false,
            relative_location: MultiSensorData::zeros(),
            north_reading: MultiSensorData { x: -0.37, y: 0.0, z: 0.6 },
        }
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
    }

    pub fn get_attitude(&mut self, dt: f32) -> Attitude {
        // Integrate angular rate
        self.last_attitude = self.last_attitude + self.last_angular_rate * dt;

        // Compute angles from gravity
        let pitch_a = self.last_acceleration.x.atan2(self.last_acceleration.z) * RADIAN_TO_DEGREES;
        let roll_a = self.last_acceleration.y.atan2(self.last_acceleration.z) * RADIAN_TO_DEGREES;

        let alpha = 0.02;
        self.last_attitude = self.last_attitude * (1.0 - alpha) + Attitude { x: roll_a, y: pitch_a, z: self.last_attitude.z } * alpha;

        self.last_attitude
    }

    pub fn get_angular_rate(&self) -> MultiSensorData {
        self.last_angular_rate
    }

    pub fn get_bearing(&self) -> f32 {
        let by2 = self.last_magnetic_reading.z * self.last_attitude.x.sin() - self.last_magnetic_reading.y * self.last_attitude.x.cos();
        let bz2 = self.last_magnetic_reading.y * self.last_attitude.x.sin() + self.last_magnetic_reading.z * self.last_attitude.x.cos();
        let bx3 = self.last_magnetic_reading.x * self.last_attitude.y.sin() + bz2 * self.last_attitude.y.cos();
        by2.atan2(bx3) * RADIAN_TO_DEGREES

        // let dphi = self.last_angular_rate.x + self.last_angular_rate.y * self.last_attitude.x.sin() * self.last_attitude.y.tan() + self.last_angular_rate.z * self.last_attitude.x.cos() * self.last_attitude.y.tan();
        // let dtheta = self.last_angular_rate.y * self.last_attitude.x.cos() – self.last_angular_rate.z * self.last_attitude.x.sin();
        // let dpsi = self.last_angular_rate.y * self.last_attitude.x.sin() / self.last_attitude.y.cos() + self.last_angular_rate.z * self.last_attitude.x.cos() / self.last_attitude.y.cos();

        // let x_tilted = self.last_magnetic_reading.x * self.last_attitude.y.cos() +
        // self.last_magnetic_reading.y * self.last_attitude.x.sin() * self.last_attitude.y.sin() -
        // self.last_magnetic_reading.z * self.last_attitude.x.cos() * self.last_attitude.y.sin();
        //
        // let y_tilted = self.last_magnetic_reading.y * self.last_attitude.x.cos() + self.last_magnetic_reading.z * self.last_attitude.x.sin();
        // (x_tilted / y_tilted).atan() * RADIAN_TO_DEGREES
    }

    pub fn get_altitude(&self) -> f32 {
        0.0
    }

    pub fn get_position(&self) -> Position {
        Position::zeros()
    }

    pub fn start_tracking(&mut self) {
        self.tracking = true;
        self.relative_location = MultiSensorData::zeros();
    }

    pub fn stop_tracking(&mut self) {
        self.tracking = false;
    }
}

//use config::Config;
//
//use sensor_manager;
//use sensor_manager::{MultiSensorData,SensorManager};
//
//use std::sync::mpsc::{Sender,Receiver,channel};
//use std::thread;
//use std::time::Duration;
//use std::f32::consts::PI;
//
//pub struct IMUData {
//
//}
//
//fn acceleration_data_to_angles(acceleration: MultiSensorData) -> MultiSensorData {
//    let angle_acc_x = acceleration.y.atan2(acceleration.z) * 180.0 / PI;
//    let angle_acc_y = acceleration.x.atan2(acceleration.z) * 180.0 / PI * -1.0;
//    MultiSensorData {
//        x: angle_acc_x,
//        y: angle_acc_y,
//        z: 0.0
//    }
//}
//
//fn magnetic_vector_to_bearing(magnetic_vector: MultiSensorData, current_angle: MultiSensorData) -> MultiSensorData {
//    MultiSensorData::zeros()
//}
//
//fn bearing_gyro_z_to_angle() -> MultiSensorData {
//    MultiSensorData::zeros()
//}
//
//pub struct IMU {
//
//}
//
//impl IMU {
//
//
//
//    pub fn start_imu(&mut self) -> (Sender<()>, Receiver<IMUData>) {
//
//        //Signal that PID loop is ready for more data
//        let (tx, ready_signal): (Sender<()>, Receiver<()>) = channel();
//        let (data_transmitter, rx): (Sender<IMUData>, Receiver<IMUData>) = channel();
//
//        thread::Builder::new().name("IMU loop".to_string()).spawn(move || {
//            let config = Config::new();
//            let loop_time = Duration::new(0, 1000000000 / config.sensor_sample_frequency);
//
//            let sensor_manager = SensorManager::new();
//            let (gyro_pipe, accel_pipe, mag_pipe) = sensor_manager.start_sensor_manager();
//
//            let mut last_gyro = MultiSensorData::zeros();
//            let mut last_accel = MultiSensorData::zeros();
//            let mut last_mag = MultiSensorData::zeros();
//            let mut last_angle = MultiSensorDat::zeros();
//
//            loop {
//                //Receive any new raw data
//                match gyro_pipe.try_recv() {
//                    Ok(dps) => {
//                        last_gyro = dps;
//                    },
//                    Err(e) => {}
//                }
//                match accel_pipe.try_recv() {
//                    Ok(accel) => {
//                        last_accel = accel;
//                    },
//                    Err(e) => {}
//                }
//                match mag_pipe.try_recv() {
//                    Ok(mag) => {
//                        last_mag = mag;
//                    },
//                    Err(e) => {}
//                }
//
//
//                let accel_angles = acceleration_data_to_angles(last_accel);
//
//                let alpha = 0.02;
//                last_angle = last_angle +
//
//
//
//                thread::sleep(loop_time);
//            }
//        });
//        (tx, rx)
//    }
//
//}
