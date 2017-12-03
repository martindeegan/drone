
// pub mod gps;
// pub mod motors;
// pub mod sensors;
pub mod barometer;

use logger::{FlightLogger, ModuleLogger};
use configurations::Config;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;

// use barometer::Barometer;
// use self::motors::{MotorManager, SerialMotorManager};

/*
Channels:
1) IMU
2) GPS
3) Barometer / Thermometer
4) Motor
*/

pub fn initialize_hardware() {
    // Barometer::new();
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
