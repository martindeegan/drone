// pub mod gps;
// pub mod motors;
// pub mod sensors;
mod barometer;
mod imu;

use logger::ModuleLogger;
// use configurations::Config;
// use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;

use self::barometer::BarometerThermometer;
use self::imu::IMU;
// use self::motors::{MotorManager, SerialMotorManager};

/*
Channels:
1) IMU
2) GPS
3) Barometer / Thermometer
4) Motor
*/

pub fn initialize_hardware() {
    let hardware_logger = ModuleLogger::new(
        "Hardware",
        Some("Failed to initialize all hardware. Exiting."),
    );

    hardware_logger.log("Initializing hardware.");

    // let barometer = match BarometerThermometer::new() {
    //     Ok(barometer) => barometer,
    //     Err(_) => {
    //         hardware_logger.error("Barometer initialization failed.");
    //         panic!("Barometer initialization failed.");
    //     }
    // };

    hardware_logger.success("Barometer initialized.");

    let imu = match IMU::new() {
        Ok(imu) => imu,
        Err(_) => {
            hardware_logger.error("IMU initialization failed.");
            panic!("IMU initialization failed.");
        }
    };

    hardware_logger.success("IMU initialized.");

    let hardware_handle = thread::spawn(move || {});
}

// pub fn initialize_hardware() -> (
//     Sender<motors::MotorOutput>,
//     Receiver<sensors::SensorInput>,
//     JoinHandle<()>,
// ) {
//     let (sensor_tx, sensor_rx): (Sender<sensors::SensorInput>, Receiver<sensors::SensorInput>) =
//         channel();
//     let (motor_tx, motor_rx): (Sender<motors::MotorOutput>, Receiver<motors::MotorOutput>) =
//         channel();

//     thread::spawn(move || {
//         let config = Config::new();
//         let sensor_manager = SensorManager::new();

//         let motor_manager = match config.motors.serial_pwm {
//             true => SerialMotorManager::new(),
//             false => SerialMotorManager::new(),
//         };

//         let gps = gps::get_gps();
//     });

//     (motor_tx, sensor_rx)
// }
