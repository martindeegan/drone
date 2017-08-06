use i2cdev_bmp180::*;
use i2cdev_bmp280::*;
use i2cdev_l3gd20::*;
use i2cdev_lsm303d::*;
use i2cdev_lsm9ds0::*;

pub fn get_bmp180() -> BMP180 {

}

pub fn get_bmp280() -> BMP280 {
    let settings = BMP280Settings {
        compensation: BMP280CompensationAlgorithm::B64,
        t_sb: BMP280Timing::ms62_5,
        iir_filter_coeff: BMP280FilterCoefficient::Off,
        osrs_t: BMP280TemperatureOversampling::x1,
        osrs_p: BMP280PressureOversampling::StandardResolution,
        power_mode: BMP280PowerMode::NormalMode
    };

    let baro = get_linux_bmp280_i2c_device().unwrap();
    match BMP280::new(baro, settings) {
        Ok(bmp280) => {
            let bmp280_ref = RefCell::new(bmp280);
        },
        Err(e) => {
            panic!("Couldn't start bmp280");
        }
    }
}

pub fn get_l3gd20() -> L3GD20 {

}

pub fn get_lsm303d() -> LSM303D {

}

pub fn get_lsm9ds0() -> LSM9DS0 {
    let gyro_settings = LSM9DS0GyroscopeSettings {
        DR: LSM9DS0GyroscopeDataRate::Hz190,
        BW: LSM9DS0GyroscopeBandwidth::BW1,
        power_mode: LSM9DS0PowerMode::Normal,
        zen: true,
        yen: true,
        xen: true,
        sensitivity: LSM9DS0GyroscopeFS::dps500,
        continuous_update: true
    };
    let accel_mag_settings = LSM9DS0AccelerometerMagnetometerSettings {
        continuous_update: true,
        accelerometer_data_rate: LSM9DS0AccelerometerUpdateRate::Hz100,
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
            let lsm9ds0_ref = RefCell::new(lsm9ds0);
            gyroscope = lsm9ds0_ref.borrow_mut();
            accelerometer = lsm9ds0_ref.borrow_mut();
            magnetometer = lsm9ds0_ref.borrow_mut();
        },
        Err(e) => {
            panic!("Couldn't initialize LSM9DS0");
        }
    }
}