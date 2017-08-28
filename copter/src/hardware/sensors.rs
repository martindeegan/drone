use super::i2cdev_bmp180::*;
use super::i2cdev_bmp280::*;
use super::i2cdev_l3gd20::*;
use super::i2cdev_lsm303d::*;
use super::i2cdev_lsm9ds0::*;
use super::i2cdev::linux::{LinuxI2CDevice,LinuxI2CError};
use super::i2cdev::core::I2CDevice;
use super::i2csensors::*;

use config::{Config,SensorCalibrations};

use time::{Duration,PreciseTime};

use std::rc::Rc;
use std::cell::RefCell;
use std::thread;
use std::sync::mpsc::{Sender,Receiver,channel};
use std::io::stdin;
use std::collections::VecDeque;

pub type MultiSensorData = Vec3;

// pub trait Vec3_{
//     fn magnitude(&self) -> f32;
// }

// impl<T> Vec3_ for T 
//     where T: Vec3 
// {
//     fn magnitude(&self) -> f32 {
//         (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
//     }
// }


pub struct SensorInput {
    pub angular_rate: MultiSensorData,
    pub acceleration: MultiSensorData,
    pub magnetic_reading: Option<MultiSensorData>,
    pub temperature: f32,
    pub pressure: f32
}

fn get_sensors() -> (Rc<RefCell<Barometer<Error = LinuxI2CError>>>,
                     Rc<RefCell<Thermometer<Error = LinuxI2CError>>>,
                     Rc<RefCell<Gyroscope<Error = LinuxI2CError>>>,
                     Rc<RefCell<Accelerometer<Error = LinuxI2CError>>>,
                     Rc<RefCell<Magnetometer<Error = LinuxI2CError>>>)
{
    let config = Config::new();
    let mut barometer: Option<Rc<RefCell<Barometer<Error = LinuxI2CError>>>> = None;
    let mut thermometer: Option<Rc<RefCell<Thermometer<Error = LinuxI2CError>>>> = None;
    let mut gyroscope: Option<Rc<RefCell<Gyroscope<Error = LinuxI2CError>>>> = None;
    let mut accelerometer: Option<Rc<RefCell<Accelerometer<Error = LinuxI2CError>>>> = None;
    let mut magnetometer: Option<Rc<RefCell<Magnetometer<Error = LinuxI2CError>>>> = None;

    fn read_calibration_values() -> (MultiSensorData, MultiSensorData) {
        let calibs = SensorCalibrations::new();
        (MultiSensorData {
            x: calibs.gyro_x,
            y: calibs.gyro_y,
            z: calibs.gyro_z,
        },
        MultiSensorData {
            x: calibs.accel_x,
            y: calibs.accel_y,
            z: calibs.accel_z
        })
    }

    for sensor in config.sensors {
        match sensor.as_ref() {
            "BMP180" => {
                let bmp180 = Rc::new(RefCell::new(get_bmp180(config.sensor_sample_frequency).unwrap()));
                barometer = Some(bmp180.clone());
                thermometer = Some(bmp180.clone());
            },
            "BMP280" => {
                let bmp280 = Rc::new(RefCell::new(get_bmp280(config.sensor_sample_frequency)));
                barometer = Some(bmp280.clone());
                thermometer = Some(bmp280.clone());
            },
            "L3GD20" => {
                let l3gd20 = Rc::new(RefCell::new(get_l3gd20(config.sensor_sample_frequency)));
                gyroscope = Some(l3gd20.clone());
            }
            "LSM303D" => {
                let lsm303d = Rc::new(RefCell::new(get_lsm303d(config.sensor_sample_frequency)));
                accelerometer = Some(lsm303d.clone());
                magnetometer = Some(lsm303d.clone());
            },
            "LSM9DS0" => {
                let lsm9ds0 = Rc::new(RefCell::new(get_lsm9ds0(config.sensor_sample_frequency)));
                gyroscope = Some(lsm9ds0.clone());
                accelerometer = Some(lsm9ds0.clone());
                magnetometer = Some(lsm9ds0.clone());
            },
            _ => {
                return panic!("Undefined sensor: {}.", sensor);
            }
        }
    }

    match barometer {
        Some(_) => {},
        None => {
            panic!("Error: No barometer set.");
        }
    }

    match thermometer {
        Some(_) => {},
        None => {
            panic!("Error: No thermometer set.");
        }
    }

    match gyroscope {
        Some(_) => {},
        None => {
            panic!("Error: No gyroscope set.");
        }
    }

    match accelerometer {
        Some(_) => {},
        None => {
            panic!("Error: No accelerometer set.");
        }
    }

    match magnetometer {
        Some(_) => {},
        None => {
            panic!("Error: No magnetometer set.");
        }
    }

    (barometer.unwrap(), thermometer.unwrap(), gyroscope.unwrap(), accelerometer.unwrap(), magnetometer.unwrap())
}


// Returns (gyro_accel_rx, mag_rx, thermo_baro_rx);
pub fn start_sensors() -> (Receiver<SensorInput>) {
    let config = Config::new();
    let sensor_poll_rate = config.sensor_sample_frequency;
    let sensor_poll_delay = (1000000000 / sensor_poll_rate) as i64;

    let (sensor_tx, sensor_rx): (Sender<SensorInput>, Receiver<SensorInput>) = channel();

    let magnetometer_counter = sensor_poll_rate / 100;

    thread::Builder::new().name("Sensor Thread".to_string()).spawn(move || {
        let loop_duration = Duration::nanoseconds(sensor_poll_delay);
        let mut count = 0;
        let (mut barometer, mut thermometer, mut gyroscope, mut accelerometer, mut magnetometer) = get_sensors();

        let calibs = SensorCalibrations::new();
        let (mut gyro_calib, mut accel_calib) = (Vec3 { x: calibs.gyro_x, y: calibs.gyro_y, z: calibs.gyro_z }, Vec3 { x: calibs.accel_x, y: calibs.accel_y, z: calibs.accel_z });

        loop {
            let start_time = PreciseTime::now();

            let mut input = SensorInput {
                angular_rate: MultiSensorData::zeros(),
                acceleration: MultiSensorData::zeros(),
                magnetic_reading: None,
                temperature: 0.0,
                pressure: 0.0
            };

            match gyroscope.borrow_mut().angular_rate_reading() {
                Ok(angular_rate) => {
                    input.angular_rate = angular_rate - gyro_calib;
                    input.angular_rate.y *= -1.0;
                    input.angular_rate.z *= -1.0;
                },
                Err(e) => {}
            }

            match accelerometer.borrow_mut().acceleration_reading() {
                Ok(acceleration) => {
                    input.acceleration = acceleration - accel_calib;
                    input.acceleration.x *= -1.0;
                },
                Err(e) => {}
            }

            if count % magnetometer_counter == 0 {
                match magnetometer.borrow_mut().magnetic_reading() {
                    Ok(magnetism) => {
                        input.magnetic_reading = Some(magnetism);
                    },
                    Err(e) => {}
                }
            }

            match thermometer.borrow_mut().temperature_celsius() {
                Ok(temp) => {
                    input.temperature = temp;
                },
                Err(e) => {}
            }

            match barometer.borrow_mut().pressure_kpa() {
                Ok(pressure) => {
                    input.pressure = pressure;
                },
                Err(e) => {}
            }

            sensor_tx.send(input);

            count += 1;

            let remaining = loop_duration - start_time.to(PreciseTime::now());
            if remaining > Duration::zero() {
                thread::sleep(Duration::to_std(&remaining).unwrap());
            }
        }
    });

    sensor_rx
}

pub fn calibrate_sensors() {
    println!("Calibrating.");
    let (mut barometer, mut thermometer, mut gyroscope, mut accelerometer, mut magnetometer) = get_sensors();

    let mut acceleration_calibration = Vec3::zeros();
    let mut gyroscope_calibration = Vec3::zeros();
    for i in 0..100 {
        acceleration_calibration = acceleration_calibration + accelerometer.borrow_mut().acceleration_reading().unwrap();
        gyroscope_calibration = gyroscope_calibration + gyroscope.borrow_mut().angular_rate_reading().unwrap();
        thread::sleep_ms(50);
    }

    println!("Rotate 180 degrees then press enter.");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error");

    for i in 0..100 {
        acceleration_calibration = acceleration_calibration + accelerometer.borrow_mut().acceleration_reading().unwrap();
        gyroscope_calibration = gyroscope_calibration + gyroscope.borrow_mut().angular_rate_reading().unwrap();
        thread::sleep_ms(50);
    }

    acceleration_calibration = acceleration_calibration / 200.0;
    acceleration_calibration.z -= 1.0;
    gyroscope_calibration = gyroscope_calibration / 200.0;

    println!("Accelerometer calibration values: {:?}", acceleration_calibration);
    println!("Gyroscope calibration values: {:?}", gyroscope_calibration);

    let calibs = SensorCalibrations {
        gyro_x: gyroscope_calibration.x,
        gyro_y: gyroscope_calibration.y,
        gyro_z: gyroscope_calibration.z,
        accel_x: acceleration_calibration.x,
        accel_y: acceleration_calibration.y,
        accel_z: acceleration_calibration.z
    };

    calibs.write_calibration();
}

//-----------------------Specific Sensors-------------------------//

fn get_bmp180(frequency: u32) -> Option<BMP180BarometerThermometer<LinuxI2CDevice>> {
    // Left for someone who owns a bmp180
    None
}

fn get_bmp280(frequency: u32) -> BMP280<LinuxI2CDevice> {
    let settings = BMP280Settings {
        compensation: BMP280CompensationAlgorithm::B64,
        t_sb: BMP280Timing::ms0_5,
        iir_filter_coeff: BMP280FilterCoefficient::Off,
        osrs_t: BMP280TemperatureOversampling::x1,
        osrs_p: BMP280PressureOversampling::StandardResolution,
        power_mode: BMP280PowerMode::NormalMode
    };

    let baro = get_linux_bmp280_i2c_device().unwrap();
    match BMP280::new(baro, settings) {
        Ok(bmp280) => bmp280,
        Err(e) => {
            panic!("Couldn't start bmp280");
        }
    }
}

fn get_l3gd20(frequency: u32) -> L3GD20<LinuxI2CDevice> {
    let mut gyro_settings = L3GD20GyroscopeSettings {
        DR: L3GD20GyroscopeDataRate::Hz190,
        BW: L3GD20GyroscopeBandwidth::BW1,
        power_mode: L3GD20PowerMode::Normal,
        zen: true,
        yen: true,
        xen: true,
        sensitivity: L3GD20GyroscopeFS::dps500,
        continuous_update: true,
        high_pass_filter_enabled: true,
        high_pass_filter_mode: Some(L3GD20GyroscopeHighPassFilterMode::NormalMode),
        high_pass_filter_configuration: Some(L3GD20HighPassFilterCutOffConfig::HPCF_3)
    };

    if frequency <= 95 {
        gyro_settings.DR = L3GD20GyroscopeDataRate::Hz95;
    }
    else if frequency <= 190 {
        gyro_settings.DR = L3GD20GyroscopeDataRate::Hz190;
        gyro_settings.BW = L3GD20GyroscopeBandwidth::BW2;
    }
    else if frequency <= 380 {
        gyro_settings.DR = L3GD20GyroscopeDataRate::Hz380;
        gyro_settings.BW = L3GD20GyroscopeBandwidth::BW3;
    }
    else {
        gyro_settings.DR = L3GD20GyroscopeDataRate::Hz760;
        gyro_settings.BW = L3GD20GyroscopeBandwidth::BW4;
    }

    let gyro_device = get_linux_l3gd20_i2c_device().unwrap();
    match L3GD20::new(gyro_device, gyro_settings) {
        Ok(l3gd20) => l3gd20,
        Err(e) => {
            panic!("Couldn't start l3gd20");
        }
    }
}

fn get_lsm303d(frequency: u32) -> LSM303D<LinuxI2CDevice> {
    let mut accel_mag_settings = LSM303DSettings {
        continuous_update: true,
        accelerometer_data_rate: LSM303DAccelerometerUpdateRate::Hz200,
        accelerometer_anti_alias_filter_bandwidth: LSM303DAccelerometerFilterBandwidth::Hz50,
        azen: true,
        ayen: true,
        axen: true,
        accelerometer_sensitivity: LSM303DAccelerometerFS::g4,
        magnetometer_resolution: LSM303DMagnetometerResolution::Low,
        magnetometer_data_rate: LSM303DMagnetometerUpdateRate::Hz100,
        magnetometer_low_power_mode: false,
        magnetometer_mode: LSM303DMagnetometerMode::ContinuousConversion,
        magnetometer_sensitivity: LSM303DMagnetometerFS::gauss2
    };

    if frequency <= 100 {
        accel_mag_settings.accelerometer_data_rate = LSM303DAccelerometerUpdateRate::Hz100;
        accel_mag_settings.accelerometer_anti_alias_filter_bandwidth = LSM303DAccelerometerFilterBandwidth::Hz50;
    }
    else if frequency <= 200 {
        accel_mag_settings.accelerometer_data_rate = LSM303DAccelerometerUpdateRate::Hz200;
        accel_mag_settings.accelerometer_anti_alias_filter_bandwidth = LSM303DAccelerometerFilterBandwidth::Hz50;
    }
    else if frequency <= 400 {
        accel_mag_settings.accelerometer_data_rate = LSM303DAccelerometerUpdateRate::Hz400;
        accel_mag_settings.accelerometer_anti_alias_filter_bandwidth = LSM303DAccelerometerFilterBandwidth::Hz194;
    }
    else if frequency <= 800 {
        accel_mag_settings.accelerometer_data_rate = LSM303DAccelerometerUpdateRate::Hz800;
        accel_mag_settings.accelerometer_anti_alias_filter_bandwidth = LSM303DAccelerometerFilterBandwidth::Hz194;
    }
    else {
        accel_mag_settings.accelerometer_data_rate = LSM303DAccelerometerUpdateRate::Hz1600;
        accel_mag_settings.accelerometer_anti_alias_filter_bandwidth = LSM303DAccelerometerFilterBandwidth::Hz773;
    }

    let accel_mag_device = get_linux_lsm303d_i2c_device().unwrap();
    match LSM303D::new(accel_mag_device, accel_mag_settings) {
        Ok(lsm303d) => lsm303d,
        Err(e) => {
            panic!("Couldn't start lsm303d");
        }
    }
}

fn get_lsm9ds0(frequency: u32) -> LSM9DS0<LinuxI2CDevice> {

    let mut gyro_settings = LSM9DS0GyroscopeSettings {
        DR: LSM9DS0GyroscopeDataRate::Hz190,
        BW: LSM9DS0GyroscopeBandwidth::BW1,
        power_mode: LSM9DS0PowerMode::Normal,
        zen: true,
        yen: true,
        xen: true,
        sensitivity: LSM9DS0GyroscopeFS::dps500,
        continuous_update: true,
        high_pass_filter_enabled: true,
        high_pass_filter_mode: Some(LSM9DS0GyroscopeHighPassFilterMode::NormalMode),
        high_pass_filter_configuration: Some(LSM9DS0HighPassFilterCutOffConfig::HPCF_3)
    };

    if frequency <= 95 {
        gyro_settings.DR = LSM9DS0GyroscopeDataRate::Hz95;
    }
    else if frequency <= 190 {
        gyro_settings.DR = LSM9DS0GyroscopeDataRate::Hz190;
        gyro_settings.BW = LSM9DS0GyroscopeBandwidth::BW2;
    }
    else if frequency <= 380 {
        gyro_settings.DR = LSM9DS0GyroscopeDataRate::Hz380;
        gyro_settings.BW = LSM9DS0GyroscopeBandwidth::BW3;
    }
    else {
        gyro_settings.DR = LSM9DS0GyroscopeDataRate::Hz760;
        gyro_settings.BW = LSM9DS0GyroscopeBandwidth::BW4;
    }

    let mut accel_mag_settings = LSM9DS0AccelerometerMagnetometerSettings {
        continuous_update: true,
        accelerometer_data_rate: LSM9DS0AccelerometerUpdateRate::Hz200,
        accelerometer_anti_alias_filter_bandwidth: LSM9DS0AccelerometerFilterBandwidth::Hz50,
        azen: true,
        ayen: true,
        axen: true,
        accelerometer_sensitivity: LSM9DS0AccelerometerFS::g4,
        magnetometer_resolution: LSM9DS0MagnetometerResolution::Low,
        magnetometer_data_rate: LSM9DS0MagnetometerUpdateRate::Hz100,
        magnetometer_low_power_mode: false,
        magnetometer_mode: LSM9DS0MagnetometerMode::ContinuousConversion,
        magnetometer_sensitivity: LSM9DS0MagnetometerFS::gauss2
    };

    if frequency <= 100 {
        accel_mag_settings.accelerometer_data_rate = LSM9DS0AccelerometerUpdateRate::Hz100;
        accel_mag_settings.accelerometer_anti_alias_filter_bandwidth = LSM9DS0AccelerometerFilterBandwidth::Hz50;
    }
    else if frequency <= 200 {
        accel_mag_settings.accelerometer_data_rate = LSM9DS0AccelerometerUpdateRate::Hz200;
        accel_mag_settings.accelerometer_anti_alias_filter_bandwidth = LSM9DS0AccelerometerFilterBandwidth::Hz50;
    }
    else if frequency <= 400 {
        accel_mag_settings.accelerometer_data_rate = LSM9DS0AccelerometerUpdateRate::Hz400;
        accel_mag_settings.accelerometer_anti_alias_filter_bandwidth = LSM9DS0AccelerometerFilterBandwidth::Hz194;
    }
    else if frequency <= 800 {
        accel_mag_settings.accelerometer_data_rate = LSM9DS0AccelerometerUpdateRate::Hz800;
        accel_mag_settings.accelerometer_anti_alias_filter_bandwidth = LSM9DS0AccelerometerFilterBandwidth::Hz194;
    }
    else {
        accel_mag_settings.accelerometer_data_rate = LSM9DS0AccelerometerUpdateRate::Hz1600;
        accel_mag_settings.accelerometer_anti_alias_filter_bandwidth = LSM9DS0AccelerometerFilterBandwidth::Hz773;
    }

    let (gyro, accel) = get_default_lsm9ds0_linux_i2c_devices().unwrap();

    match LSM9DS0::new(accel, gyro, gyro_settings, accel_mag_settings) {
        Ok(lsm9ds0) => lsm9ds0,
        Err(e) => {
            panic!("Couldn't initialize LSM9DS0");
        }
    }
}
