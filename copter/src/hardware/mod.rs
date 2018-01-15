// pub mod gps;
// pub mod sensors;


use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::thread::{sleep, Builder, JoinHandle};
use std::default::Default;

use na::Vector3;
use na::geometry::UnitQuaternion;
use num::traits::Zero;

use time::{Duration, PreciseTime};
use i2csensors::{Accelerometer, Barometer, Gyroscope, Magnetometer};

use logger::ModuleLogger;
// use configurations::Config;

mod barometer;
mod imu;
mod motors;
mod gps;
mod battery;
mod mock;

use self::barometer::BarometerThermometer;
use self::imu::IMU;
use self::motors::{MotorManager, SerialMotorManager};
use self::gps::get_gps;
use self::battery::{BatteryMonitor, BatteryStatus};

pub use self::motors::MotorCommand;
pub use self::gps::GPSData;

const MILLISECONDS_PER_SECOND: i64 = 1000;
const LOOP_FREQUENCY: i64 = 75; // 95 Hz Loop

pub fn initialize_hardware() -> (
    JoinHandle<()>,
    Receiver<PredictionReading>,
    Receiver<UpdateReading>,
    Sender<MotorCommand>,
    Sender<()>,
) {
    let (pred_tx, pred_rx): (Sender<PredictionReading>, Receiver<PredictionReading>) = channel();
    let (update_tx, update_rx): (Sender<UpdateReading>, Receiver<UpdateReading>) = channel();
    let (motor_tx, motor_rx): (Sender<MotorCommand>, Receiver<MotorCommand>) = channel();
    let (control_tx, control_rx): (Sender<()>, Receiver<()>) = channel();

    let hardware_handle: JoinHandle<()> = Builder::new()
        .name(String::from("Hardware Thread"))
        .spawn(move || {
            let hardware_logger = ModuleLogger::new(
                "Hardware",
                Some("Failed to initialize all hardware. Exiting."),
            );
            hardware_logger.log("Initializing hardware.");

            let mut barometer = match BarometerThermometer::new() {
                Ok(barometer) => barometer,
                Err(_) => {
                    hardware_logger.error("Barometer initialization failed.");
                    panic!("Barometer initialization failed.");
                }
            };
            hardware_logger.success("Barometer initialized.");

            let mut imu = match IMU::new() {
                Ok(imu) => imu,
                Err(_) => {
                    hardware_logger.error("IMU initialization failed.");
                    panic!("IMU initialization failed.");
                }
            };
            hardware_logger.success("IMU initialized.");

            let mut motor_manager = match SerialMotorManager::new() {
                Ok(motors) => motors,
                Err(_) => {
                    hardware_logger.error("Motor initialization failed.");
                    panic!("Motor initialization failed.");
                }
            };
            hardware_logger.success("Motors initialized.");


            let mut gps_rx = get_gps();
            hardware_logger.success("GPS started.");

            /*
            let battery_monitor = match BatteryMonitor::new() {
                Ok(monitor) => monitor,
                Err(()) => {
                    hardware_logger.error("Battery monitor initialization failed.");
                    panic!("Battery monitor initialization failed.");
                }
            };

            hardware_logger.success("Battery monitor initialized.");
            */

            hardware_logger.success("All hardware initialized successfully.");

            hardware_loop(
                &mut barometer,
                &mut imu,
                &mut motor_manager,
                gps_rx,
                pred_tx.clone(),
                update_tx.clone(),
                motor_rx,
                control_rx,
            );
        })
        .unwrap();

    sleep(Duration::seconds(1).to_std().unwrap());
    (hardware_handle, pred_rx, update_rx, motor_tx, control_tx)
}

pub fn calibrate_sensors() {
    let hardware_logger =
        ModuleLogger::new("Hardware", Some("Failed to calibrate hardware. Exiting."));

    let mut imu = match IMU::new() {
        Ok(imu) => imu,
        Err(_) => {
            hardware_logger.error("IMU initialization failed.");
            panic!("IMU initialization failed.");
        }
    };

    imu.calibrate_sensors();
}

pub fn calibrate_motors() {
    let hardware_logger =
        ModuleLogger::new("Hardware", Some("Failed to calibrate hardware. Exiting."));

    let mut motor_manager = match SerialMotorManager::new() {
        Ok(motors) => motors,
        Err(_) => {
            hardware_logger.error("Motor initialization failed.");
            panic!("Motor initialization failed.");
        }
    };

    motor_manager.calibrate();
}

#[derive(Debug)]
pub struct PredictionReading {
    pub angular_rate: Vector3<f64>,
    pub acceleration: Vector3<f64>,
}

impl Default for PredictionReading {
    fn default() -> PredictionReading {
        PredictionReading {
            angular_rate: Vector3::zero(),
            acceleration: Vector3::zero(),
        }
    }
}

#[derive(Debug)]
pub struct UpdateReading {
    pub acceleration: Vector3<f64>,
    pub magnetic_reading: Option<Vector3<f64>>,
    pub pressure: f64,
    pub gps_information: Option<GPSData>,
}

impl Default for UpdateReading {
    fn default() -> UpdateReading {
        UpdateReading {
            acceleration: Vector3::zero(),
            magnetic_reading: None,
            pressure: 0.0,
            gps_information: None,
        }
    }
}

fn hardware_loop(
    barometer: &mut BarometerThermometer,
    imu: &mut IMU,
    motor_manager: &mut SerialMotorManager,
    gps_rx: Receiver<GPSData>,
    prediction_tx: Sender<PredictionReading>,
    update_tx: Sender<UpdateReading>,
    motor_rx: Receiver<MotorCommand>,
    control_rx: Receiver<()>,
) {
    let hardware_logger = ModuleLogger::new("Hardware", None);

    let mut loop_count = 0;
    'hardware: loop {
        loop_count += 1;

        let start_time = PreciseTime::now();

        let angular_rate = imu.read_gyroscope().unwrap();
        let acceleration = imu.read_accelerometer().unwrap();

        let prediction_reading = PredictionReading {
            angular_rate: angular_rate,
            acceleration: acceleration,
        };

        match prediction_tx.send(prediction_reading) {
            Ok(_) => {}
            Err(_) => {
                hardware_logger.error("Failed to send predictive readings.");
            }
        }

        let gps_information: Option<GPSData> = match gps_rx.try_recv() {
            Ok(gps_data) => Some(gps_data),
            Err(_) => None,
        };

        let pressure = barometer.read_pressure();
        let mut magnetic_reading: Option<Vector3<f64>> = None;
        if loop_count % 4 == 0 {
            magnetic_reading = Some(imu.read_magnetometer().unwrap());
        }

        let mut update_reading = UpdateReading {
            acceleration: acceleration,
            magnetic_reading: magnetic_reading,
            pressure: pressure as f64,
            gps_information: gps_information,
        };

        match update_tx.send(update_reading) {
            Ok(_) => {}
            Err(_) => {
                hardware_logger.error("Failed to send update readings.");
            }
        };

        match motor_rx.recv() {
            Ok(command) => {
                motor_manager.process_command(command);
            }
            Err(_) => {
                hardware_logger.error("Failed to receive motor commands");
            }
        };

        let finish_time = PreciseTime::now();
        let diff = Duration::milliseconds(MILLISECONDS_PER_SECOND / LOOP_FREQUENCY)
            - start_time.to(finish_time);
        if diff > Duration::zero() {
            sleep(diff.to_std().unwrap());
        }

        match control_rx.try_recv() {
            Ok(_) => {
                hardware_logger.log("Stopping hardware.");
                sleep(Duration::seconds(2).to_std().unwrap());
                break 'hardware;
            }
            Err(_) => {}
        };
    }
}
