//use self::i2cdev_bmp180::*;
use super::i2cdev_bmp280::*;
//use super::i2cdev_l3gd20::*;
//use super::i2cdev_lsm303d::*;
use super::i2cdev_lsm9ds0::*;
use super::i2cdev::linux::LinuxI2CDevice;
use super::i2cdev::core::I2CDevice;

pub fn get_bmp280(poll_rate: i64) -> BMP280<LinuxI2CDevice> {
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
        Ok(bmp280) => {
            return bmp280;
        },
        Err(e) => {
            panic!("Couldn't start bmp280");
        }
    }
}

pub fn get_lsm9ds0(poll_rate: i64) -> LSM9DS0<LinuxI2CDevice> {

    let mut gyro_settings = LSM9DS0GyroscopeSettings {
        DR: LSM9DS0GyroscopeDataRate::Hz190,
        BW: LSM9DS0GyroscopeBandwidth::BW1,
        power_mode: LSM9DS0PowerMode::Normal,
        zen: true,
        yen: true,
        xen: true,
        sensitivity: LSM9DS0GyroscopeFS::dps500,
        continuous_update: true
    };

    if poll_rate <= 95 {
        gyro_settings.DR = LSM9DS0GyroscopeDataRate::Hz95;
    }
    else if poll_rate <= 190 {
        gyro_settings.DR = LSM9DS0GyroscopeDataRate::Hz190;
        gyro_settings.BW = LSM9DS0GyroscopeBandwidth::BW2;
    }
    else if poll_rate <= 380 {
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
    let (gyro, accel) = get_default_lsm9ds0_linux_i2c_devices().unwrap();

    match LSM9DS0::new(accel, gyro, gyro_settings, accel_mag_settings) {
        Ok(lsm9ds0) => {
            let lsm9ds0_ref = return lsm9ds0;
        },
        Err(e) => {
            panic!("Couldn't initialize LSM9DS0");
        }
    }
}