use std::rc::Rc;
use std::cell::RefCell;
use std::thread::sleep;
use std::time::Duration;

use i2cdev_lsm9ds0::*;
use i2csensors::{Accelerometer, Gyroscope, Magnetometer};
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

use configurations::{Calibrations, Config};
use logger::ModuleLogger;

pub struct IMU {
    gyroscope: Rc<RefCell<Gyroscope<Error = LinuxI2CError>>>,
    accelerometer: Rc<RefCell<Accelerometer<Error = LinuxI2CError>>>,
    magnetometer: Rc<RefCell<Magnetometer<Error = LinuxI2CError>>>,
    logger: ModuleLogger,
}

impl IMU {
    pub fn new() -> Result<IMU, ()> {
        let calibs = Calibrations::new();
        let config = Config::new().unwrap();
        let logger = ModuleLogger::new("IMU", None);

        let mut gyroscope: Option<Rc<RefCell<Gyroscope<Error = LinuxI2CError>>>> = None;
        let mut accelerometer: Option<Rc<RefCell<Accelerometer<Error = LinuxI2CError>>>> = None;
        let mut magnetometer: Option<Rc<RefCell<Magnetometer<Error = LinuxI2CError>>>> = None;

        match config.hardware.gyroscope.name.as_ref() {
            "LSM9DS0" => {
                logger.log("Initializing LSM9DS0.");
                match get_lsm9ds0() {
                    Ok(lsm9ds0) => {
                        let lsm9ds0_ref = Rc::new(RefCell::new(lsm9ds0));
                        gyroscope = Some(lsm9ds0_ref.clone());
                        accelerometer = Some(lsm9ds0_ref.clone());
                        magnetometer = Some(lsm9ds0_ref.clone());
                    }
                    Err(()) => {
                        logger.error(
                            "Couldn't initialize LSM9DS0 gyroscope, accelerometer, magnetometer.",
                        );
                        return Err(());
                    }
                }
            }
            _ => {
                logger.error("Unknown gyroscope model. Check your configuration file.");
                return Err(());
            }
        };

        match config.hardware.accelerometer.name.as_ref() {
            "LSM9DS0" => {}
            _ => {
                logger.error("Unknown gyroscope model. Check your configuration file.");
                return Err(());
            }
        };

        match config.hardware.magnetometer.name.as_ref() {
            "LSM9DS0" => {}
            _ => {
                logger.error("Unknown gyroscope model. Check your configuration file.");
                return Err(());
            }
        };

        let imu = IMU {
            gyroscope: gyroscope.unwrap().clone(),
            accelerometer: accelerometer.unwrap().clone(),
            magnetometer: magnetometer.unwrap().clone(),
            logger: logger,
        };

        // Test IMU readings
        sleep(Duration::from_millis(50));
        match imu.gyroscope.borrow_mut().angular_rate_reading() {
            Ok(_) => {
                &imu.logger.log("Gyroscope check.");
            }
            Err(_) => {
                &imu.logger.error("Gyroscope failed to read.");
                return Err(());
            }
        };

        sleep(Duration::from_millis(50));
        match imu.accelerometer.borrow_mut().acceleration_reading() {
            Ok(_) => {
                &imu.logger.log("Accelerometer check.");
            }
            Err(_) => {
                &imu.logger.error("Accelerometer failed to read.");
                return Err(());
            }
        };

        sleep(Duration::from_millis(50));
        match imu.magnetometer.borrow_mut().magnetic_reading() {
            Ok(_) => {
                &imu.logger.log("Magnetometer check.");
            }
            Err(_) => {
                &imu.logger.error("Magnetometer failed to read.");
                return Err(());
            }
        };

        Ok(imu)
    }
}

fn get_lsm9ds0() -> Result<LSM9DS0<LinuxI2CDevice>, ()> {
    let mut gyro_settings = LSM9DS0GyroscopeSettings {
        DR: LSM9DS0GyroscopeDataRate::Hz95,
        BW: LSM9DS0GyroscopeBandwidth::BW1,
        power_mode: LSM9DS0PowerMode::Normal,
        zen: true,
        yen: true,
        xen: true,
        sensitivity: LSM9DS0GyroscopeFS::dps500,
        continuous_update: true,
        high_pass_filter_enabled: true,
        high_pass_filter_mode: Some(LSM9DS0GyroscopeHighPassFilterMode::NormalMode),
        high_pass_filter_configuration: Some(LSM9DS0HighPassFilterCutOffConfig::HPCF_3),
    };

    let mut accel_mag_settings = LSM9DS0AccelerometerMagnetometerSettings {
        continuous_update: true,
        accelerometer_data_rate: LSM9DS0AccelerometerUpdateRate::Hz100,
        accelerometer_anti_alias_filter_bandwidth: LSM9DS0AccelerometerFilterBandwidth::Hz50,
        azen: true,
        ayen: true,
        axen: true,
        accelerometer_sensitivity: LSM9DS0AccelerometerFS::g4,
        magnetometer_resolution: LSM9DS0MagnetometerResolution::Low,
        magnetometer_data_rate: LSM9DS0MagnetometerUpdateRate::Hz50,
        magnetometer_low_power_mode: false,
        magnetometer_mode: LSM9DS0MagnetometerMode::ContinuousConversion,
        magnetometer_sensitivity: LSM9DS0MagnetometerFS::gauss2,
    };

    let (gyro, accel) = get_default_lsm9ds0_linux_i2c_devices().unwrap();

    match LSM9DS0::new(accel, gyro, gyro_settings, accel_mag_settings) {
        Ok(lsm9ds0) => Ok(lsm9ds0),
        Err(_) => Err(()),
    }
}
