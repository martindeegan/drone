use sensor_manager::MultiSensorData;

pub struct AttitudeOutput {
    current_attitude: Option<MultiSensorData>,
    current_angular_rate: Option<MultiSensorData>,
    current_altitude: Option<f32>
}

pub struct IMU {
    //
    tracking: bool,
    relative_location: MultiSensorData
}

impl IMU {
    pub fn new() -> IMU {
        IMU {
            tracking: false,
            relative_location: MultiSensorData::zeros()
        }
    }

    pub fn get_orientation(&mut self) {

    }

    pub fn start_tracking(&mut self) {
        self.tracking = true;
        self.relative_location = MultiSensorData::zeros();
    }

    pub fn stop_tracking(&mut self) {
        self.tracking = false;
    }

    // Dead-reckoning
    // Returns (relative_x, relative_y, altitude)
    pub fn get_positioning(&mut self) -> (f32, f32, f32) {
        if self.tracking {

        }

        (0.0,0.0,0.0)
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
