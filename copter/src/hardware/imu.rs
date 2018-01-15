use std::rc::Rc;
use std::cell::RefCell;
use std::thread::sleep;
use std::time::Duration;
use std::io;

use i2cdev_lsm9ds0::*;
use i2csensors::{Accelerometer, Gyroscope, Magnetometer};
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

use na::{try_inverse, zero, Matrix, Matrix3, Matrix4, MatrixArray, MatrixMN, MatrixN, MatrixVec,
         Vector, Vector3, VectorN};
use na::{U100, U3, U4, U9};
use na::geometry::UnitQuaternion;
use typenum::U200;
use num::traits::Zero;

type Matrix200x9 = Matrix<f64, U200, U9, MatrixArray<f64, U200, U9>>;
type Vector9 = VectorN<f64, U9>;
type Vector200 = VectorN<f64, U200>;

use configurations::{Calibrations, Config, Ellipsoid, Simple};
use logger::ModuleLogger;

const G_TO_MPSPS: f64 = 9.80665;

use super::mock::MockSensor;

pub struct IMU {
    gyroscope: Rc<RefCell<Gyroscope<Error = LinuxI2CError>>>,
    gyroscope_offsets: Vector3<f64>,
    accelerometer: Rc<RefCell<Accelerometer<Error = LinuxI2CError>>>,
    accelerometer_offsets: Vector3<f64>,
    magnetometer: Rc<RefCell<Magnetometer<Error = LinuxI2CError>>>,
    magnetometer_offsets: Vector3<f64>,
    magnetometer_rotation: Matrix3<f64>,
    magnetometer_gains: Vector3<f64>,
    logger: ModuleLogger,
    // calibrations: Calibrations,
}

impl IMU {
    pub fn new() -> Result<IMU, ()> {
        let logger = ModuleLogger::new("IMU", None);
        let config = Config::new().unwrap();

        let mut gyroscope: Option<Rc<RefCell<Gyroscope<Error = LinuxI2CError>>>> = None;
        let mut accelerometer: Option<Rc<RefCell<Accelerometer<Error = LinuxI2CError>>>> = None;
        let mut magnetometer: Option<Rc<RefCell<Magnetometer<Error = LinuxI2CError>>>> = None;

        #[cfg(not(target_arch = "arm"))]
        {
            logger.log("Initializing mock IMU.");
            let mock_sensor = Rc::new(RefCell::new(MockSensor::new()));
            gyroscope = Some(mock_sensor.clone());
            accelerometer = Some(mock_sensor.clone());
            magnetometer = Some(mock_sensor.clone());
        }

        #[cfg(target_arch = "arm")]
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

        #[cfg(target_arch = "arm")]
        match config.hardware.accelerometer.name.as_ref() {
            "LSM9DS0" => {}
            _ => {
                logger.error("Unknown gyroscope model. Check your configuration file.");
                return Err(());
            }
        };

        #[cfg(target_arch = "arm")]
        match config.hardware.magnetometer.name.as_ref() {
            "LSM9DS0" => {}
            _ => {
                logger.error("Unknown gyroscope model. Check your configuration file.");
                return Err(());
            }
        };

        let mut imu = IMU {
            gyroscope: gyroscope.unwrap().clone(),
            gyroscope_offsets: Vector3::zero(),
            accelerometer: accelerometer.unwrap().clone(),
            accelerometer_offsets: Vector3::zero(),
            magnetometer: magnetometer.unwrap().clone(),
            magnetometer_offsets: Vector3::zero(),
            magnetometer_rotation: Matrix3::zero(),
            magnetometer_gains: Vector3::zero(),
            logger: logger,
        };

        // Read calibrations
        let calibs = Calibrations::new().unwrap();
        if calibs.gyroscope.is_some() {
            imu.gyroscope_offsets = calibs.gyroscope.unwrap().get_offsets();
        }
        if calibs.accelerometer.is_some() {
            imu.accelerometer_offsets = calibs.accelerometer.unwrap().get_offsets();
        }
        if calibs.magnetometer.is_some() {
            let mag_calibs = calibs.magnetometer.unwrap();
            imu.magnetometer_offsets = mag_calibs.get_offsets();
            imu.magnetometer_rotation = mag_calibs.get_rotation();
            imu.magnetometer_gains = mag_calibs.get_gains();
        }

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

    fn read_gyroscope_raw(&mut self) -> Result<Vector3<f64>, ()> {
        match self.gyroscope.borrow_mut().angular_rate_reading() {
            Ok(angular_rate) => Ok(Vector3::new(
                // Right hand rule. LSM9DS0 is opposite for roll and yaw...
                -angular_rate.x.to_radians() as f64,
                angular_rate.y.to_radians() as f64,
                -angular_rate.z.to_radians() as f64,
            )),
            Err(_) => {
                self.logger.error("Couldn't read gyroscope.");
                return Err(());
            }
        }
    }

    fn read_accelerometer_raw(&mut self) -> Result<Vector3<f64>, ()> {
        match self.accelerometer.borrow_mut().acceleration_reading() {
            Ok(acceleration) => Ok(Vector3::new(
                acceleration.x as f64,
                -acceleration.y as f64,
                (acceleration.z as f64) * G_TO_MPSPS,
            )),
            Err(_) => {
                self.logger.error("Couldn't read accelerometer.");
                return Err(());
            }
        }
    }

    fn read_magnetometer_raw(&mut self) -> Result<Vector3<f64>, ()> {
        match self.magnetometer.borrow_mut().magnetic_reading() {
            Ok(magnetic) => Ok(Vector3::new(
                magnetic.x as f64,
                magnetic.y as f64,
                magnetic.z as f64,
            )),
            Err(_) => {
                self.logger.error("Couldn't read magnetometer.");
                return Err(());
            }
        }
    }

    pub fn read_gyroscope(&mut self) -> Result<Vector3<f64>, ()> {
        match self.read_gyroscope_raw() {
            Ok(angular_rate_raw) => Ok(angular_rate_raw - self.gyroscope_offsets),
            Err(_) => Err(()),
        }
    }
    pub fn read_accelerometer(&mut self) -> Result<Vector3<f64>, ()> {
        match self.read_accelerometer_raw() {
            Ok(acceleration_raw) => Ok(acceleration_raw - self.accelerometer_offsets),
            Err(_) => Err(()),
        }
    }
    pub fn read_magnetometer(&mut self) -> Result<Vector3<f64>, ()> {
        match self.read_magnetometer_raw() {
            Ok(magnetic_reading_raw) => {
                let offset_corrected = magnetic_reading_raw - self.magnetometer_offsets;
                let rotation_corrected = self.magnetometer_rotation * offset_corrected;
                Ok(rotation_corrected.component_div(&self.magnetometer_gains))
            }
            Err(_) => Err(()),
        }
    }

    pub fn calibrate_sensors(&mut self) {
        let mut calibrations = Calibrations::new().unwrap();
        // self.calibrate_magnetometer(&mut calibrations);
        self.calibrate_gyroscope(&mut calibrations);
        self.calibrate_accelerometer(&mut calibrations);

        calibrations.save().unwrap();
    }

    fn calibrate_magnetometer(&mut self, calibs: &mut Calibrations) {
        println!("You are now going to calibrate the magnetometer.");
        println!("Keep the drone several feet away from any metal.");
        println!("During the calibration spin the drone to get samples from every direction.");
        println!("Press enter to begin...");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut D = Matrix200x9::new_random();

        println!("3...");
        sleep(Duration::from_secs(1));
        println!("2...");
        sleep(Duration::from_secs(1));
        println!("1...");
        sleep(Duration::from_secs(1));
        println!("Go!");
        for i in 0..200 {
            let magnetic_reading = self.read_magnetometer_raw().unwrap();
            let D_i = Vector9::from_fn(|r, c| match r {
                0 => magnetic_reading.x * magnetic_reading.x,
                1 => magnetic_reading.y * magnetic_reading.y,
                2 => magnetic_reading.z * magnetic_reading.z,
                3 => 2.0 * magnetic_reading.x * magnetic_reading.y,
                4 => 2.0 * magnetic_reading.x * magnetic_reading.z,
                5 => 2.0 * magnetic_reading.y * magnetic_reading.z,
                6 => 2.0 * magnetic_reading.x,
                7 => 2.0 * magnetic_reading.y,
                8 => 2.0 * magnetic_reading.z,
                _ => 0.0,
            });

            D.row_mut(i).copy_from(&D_i.transpose());

            sleep(Duration::from_millis(100));
        }

        // Rotated ellipsoid fitting
        let mut ones: Vector200 = Vector200::from_element(1.0);

        // v[9x1] = (D^T D)^(-1)D^T
        let v = (D.transpose() * D).try_inverse().unwrap() * (&D.transpose() * ones);

        // Auxillary matrices
        #[rustfmt_skip]
        let A_4: Matrix4<f64> = Matrix4::new(v.data[0], v.data[3], v.data[4], v.data[6], 
                                             v.data[3], v.data[1], v.data[5], v.data[7], 
                                             v.data[4], v.data[5], v.data[2], v.data[8], 
                                             v.data[6], v.data[7], v.data[8], -1.0,
        );

        #[rustfmt_skip]
        let A_3: Matrix3<f64> = Matrix3::new(v.data[0],v.data[3],v.data[4],
                                             v.data[3],v.data[1],v.data[5],
                                             v.data[4],v.data[5],v.data[2],
        );
        let v_ghi: Vector3<f64> = Vector3::new(v.data[6], v.data[7], v.data[8]);

        // Compute offsets
        let offsets = -1.0 * A_3.try_inverse().unwrap() * v_ghi;

        // More auxillary matrices
        #[rustfmt_skip]
        let T: Matrix4<f64> = Matrix4::new(1.0,0.0,0.0,0.0,
                                           0.0,1.0,0.0,0.0,
                                           0.0,0.0,1.0,0.0,
                                           offsets.data[0],offsets.data[1],offsets.data[2],1.0);
        let B_4 = T * A_4 * T.transpose();

        let b_44 = -1.0 * B_4.data[15];
        let B_3: Matrix3<f64> = B_4.fixed_resize(0.0) / b_44;

        // Compute gains and rotation
        let eigen_decomp = B_3.symmetric_eigen();
        let gains: Vector3<f64> =
            Vector3::from_fn(|r, c| (1.0 / eigen_decomp.eigenvalues.data[r]).sqrt());
        let rotation: Matrix3<f64> = eigen_decomp.eigenvectors.try_inverse().unwrap();

        calibs.magnetometer = Some(Ellipsoid::new(offsets, rotation, gains));
    }

    fn calibrate_gyroscope(&mut self, calibs: &mut Calibrations) {
        println!("Now calibrating the gyroscope.");
        println!("Place the drone on a level surface.");
        println!("Press enter to begin...");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        println!("3...");
        sleep(Duration::from_secs(1));
        println!("2...");
        sleep(Duration::from_secs(1));
        println!("1...");
        sleep(Duration::from_secs(1));
        println!("Go!");

        let mut offsets = self.read_gyroscope_raw().unwrap();
        for i in 0..200 {
            offsets += self.read_gyroscope_raw().unwrap();
            sleep(Duration::from_millis(20));
        }

        offsets /= 201.0;

        calibs.gyroscope = Some(Simple::new(offsets));
    }

    fn calibrate_accelerometer(&mut self, calibs: &mut Calibrations) {
        println!("Now calibrating accelerometer.");
        println!("Place the drone on a level surface.");
        println!("Press enter to begin...");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        println!("3...");
        sleep(Duration::from_secs(1));
        println!("2...");
        sleep(Duration::from_secs(1));
        println!("1...");
        sleep(Duration::from_secs(1));
        println!("Go!");

        let mut offsets = self.read_accelerometer_raw().unwrap();
        for i in 0..200 {
            offsets += self.read_accelerometer_raw().unwrap();
            sleep(Duration::from_millis(20));
        }

        offsets /= 201.0;
        offsets = offsets - Vector3::new(0.0, 0.0, G_TO_MPSPS);

        calibs.accelerometer = Some(Ellipsoid::new(offsets, Matrix3::zero(), Vector3::zero()));
    }
}

#[cfg(target_arch = "arm")]
fn get_lsm9ds0() -> Result<LSM9DS0<LinuxI2CDevice>, ()> {
    let gyro_settings = LSM9DS0GyroscopeSettings {
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

    let accel_mag_settings = LSM9DS0AccelerometerMagnetometerSettings {
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

// fn get_l3gd20(frequency: u32) -> L3GD20<LinuxI2CDevice> {
//     let mut gyro_settings = L3GD20GyroscopeSettings {
//         DR: L3GD20GyroscopeDataRate::Hz190,
//         BW: L3GD20GyroscopeBandwidth::BW1,
//         power_mode: L3GD20PowerMode::Normal,
//         zen: true,
//         yen: true,
//         xen: true,
//         sensitivity: L3GD20GyroscopeFS::dps500,
//         continuous_update: true,
//         high_pass_filter_enabled: true,
//         high_pass_filter_mode: Some(L3GD20GyroscopeHighPassFilterMode::NormalMode),
//         high_pass_filter_configuration: Some(L3GD20HighPassFilterCutOffConfig::HPCF_3),
//     };

//     if frequency <= 190 {
//         gyro_settings.DR = L3GD20GyroscopeDataRate::Hz190;
//         gyro_settings.BW = L3GD20GyroscopeBandwidth::BW2;
//     } else if frequency <= 380 {
//         gyro_settings.DR = L3GD20GyroscopeDataRate::Hz380;
//         gyro_settings.BW = L3GD20GyroscopeBandwidth::BW3;
//     } else {
//         gyro_settings.DR = L3GD20GyroscopeDataRate::Hz760;
//         gyro_settings.BW = L3GD20GyroscopeBandwidth::BW4;
//     }

//     let gyro_device = get_linux_l3gd20_i2c_device().unwrap();
//     match L3GD20::new(gyro_device, gyro_settings) {
//         Ok(l3gd20) => l3gd20,
//         Err(e) => {
//             panic!("Couldn't start l3gd20");
//         }
//     }
// }

// fn get_lsm303d(frequency: u32) -> LSM303D<LinuxI2CDevice> {
//     let mut accel_mag_settings = LSM303DSettings {
//         continuous_update: true,
//         accelerometer_data_rate: LSM303DAccelerometerUpdateRate::Hz200,
//         accelerometer_anti_alias_filter_bandwidth: LSM303DAccelerometerFilterBandwidth::Hz50,
//         azen: true,
//         ayen: true,
//         axen: true,
//         accelerometer_sensitivity: LSM303DAccelerometerFS::g4,
//         magnetometer_resolution: LSM303DMagnetometerResolution::Low,
//         magnetometer_data_rate: LSM303DMagnetometerUpdateRate::Hz100,
//         magnetometer_low_power_mode: false,
//         magnetometer_mode: LSM303DMagnetometerMode::ContinuousConversion,
//         magnetometer_sensitivity: LSM303DMagnetometerFS::gauss2,
//     };

//     if frequency <= 200 {
//         accel_mag_settings.accelerometer_data_rate = LSM303DAccelerometerUpdateRate::Hz200;
//         accel_mag_settings.accelerometer_anti_alias_filter_bandwidth =
//             LSM303DAccelerometerFilterBandwidth::Hz50;
//     } else if frequency <= 400 {
//         accel_mag_settings.accelerometer_data_rate = LSM303DAccelerometerUpdateRate::Hz400;
//         accel_mag_settings.accelerometer_anti_alias_filter_bandwidth =
//             LSM303DAccelerometerFilterBandwidth::Hz194;
//     } else if frequency <= 800 {
//         accel_mag_settings.accelerometer_data_rate = LSM303DAccelerometerUpdateRate::Hz800;
//         accel_mag_settings.accelerometer_anti_alias_filter_bandwidth =
//             LSM303DAccelerometerFilterBandwidth::Hz194;
//     } else {
//         accel_mag_settings.accelerometer_data_rate = LSM303DAccelerometerUpdateRate::Hz1600;
//         accel_mag_settings.accelerometer_anti_alias_filter_bandwidth =
//             LSM303DAccelerometerFilterBandwidth::Hz773;
//     }

//     let accel_mag_device = get_linux_lsm303d_i2c_device().unwrap();
//     match LSM303D::new(accel_mag_device, accel_mag_settings) {
//         Ok(lsm303d) => lsm303d,
//         Err(e) => {
//             panic!("Couldn't start lsm303d");
//         }
//     }
// }

// fn get_lsm9ds0(frequency: u32) -> LSM9DS0<LinuxI2CDevice> {
//     let mut gyro_settings = LSM9DS0GyroscopeSettings {
//         DR: LSM9DS0GyroscopeDataRate::Hz190,
//         BW: LSM9DS0GyroscopeBandwidth::BW1,
//         power_mode: LSM9DS0PowerMode::Normal,
//         zen: true,
//         yen: true,
//         xen: true,
//         sensitivity: LSM9DS0GyroscopeFS::dps500,
//         continuous_update: true,
//         high_pass_filter_enabled: true,
//         high_pass_filter_mode: Some(LSM9DS0GyroscopeHighPassFilterMode::NormalMode),
//         high_pass_filter_configuration: Some(LSM9DS0HighPassFilterCutOffConfig::HPCF_3),
//     };

//     if frequency <= 95 {
//         gyro_settings.DR = LSM9DS0GyroscopeDataRate::Hz95;
//     } else if frequency <= 190 {
//         gyro_settings.DR = LSM9DS0GyroscopeDataRate::Hz190;
//         gyro_settings.BW = LSM9DS0GyroscopeBandwidth::BW2;
//     } else if frequency <= 380 {
//         gyro_settings.DR = LSM9DS0GyroscopeDataRate::Hz380;
//         gyro_settings.BW = LSM9DS0GyroscopeBandwidth::BW3;
//     } else {
//         gyro_settings.DR = LSM9DS0GyroscopeDataRate::Hz760;
//         gyro_settings.BW = LSM9DS0GyroscopeBandwidth::BW4;
//     }

//     let mut accel_mag_settings = LSM9DS0AccelerometerMagnetometerSettings {
//         continuous_update: true,
//         accelerometer_data_rate: LSM9DS0AccelerometerUpdateRate::Hz200,
//         accelerometer_anti_alias_filter_bandwidth: LSM9DS0AccelerometerFilterBandwidth::Hz50,
//         azen: true,
//         ayen: true,
//         axen: true,
//         accelerometer_sensitivity: LSM9DS0AccelerometerFS::g4,
//         magnetometer_resolution: LSM9DS0MagnetometerResolution::Low,
//         magnetometer_data_rate: LSM9DS0MagnetometerUpdateRate::Hz100,
//         magnetometer_low_power_mode: false,
//         magnetometer_mode: LSM9DS0MagnetometerMode::ContinuousConversion,
//         magnetometer_sensitivity: LSM9DS0MagnetometerFS::gauss2,
//     };

//     if frequency <= 100 {
//         accel_mag_settings.accelerometer_data_rate = LSM9DS0AccelerometerUpdateRate::Hz100;
//         accel_mag_settings.accelerometer_anti_alias_filter_bandwidth =
//             LSM9DS0AccelerometerFilterBandwidth::Hz50;
//     } else if frequency <= 200 {
//         accel_mag_settings.accelerometer_data_rate = LSM9DS0AccelerometerUpdateRate::Hz200;
//         accel_mag_settings.accelerometer_anti_alias_filter_bandwidth =
//             LSM9DS0AccelerometerFilterBandwidth::Hz50;
//     } else if frequency <= 400 {
//         accel_mag_settings.accelerometer_data_rate = LSM9DS0AccelerometerUpdateRate::Hz400;
//         accel_mag_settings.accelerometer_anti_alias_filter_bandwidth =
//             LSM9DS0AccelerometerFilterBandwidth::Hz194;
//     } else if frequency <= 800 {
//         accel_mag_settings.accelerometer_data_rate = LSM9DS0AccelerometerUpdateRate::Hz800;
//         accel_mag_settings.accelerometer_anti_alias_filter_bandwidth =
//             LSM9DS0AccelerometerFilterBandwidth::Hz194;
//     } else {
//         accel_mag_settings.accelerometer_data_rate = LSM9DS0AccelerometerUpdateRate::Hz1600;
//         accel_mag_settings.accelerometer_anti_alias_filter_bandwidth =
//             LSM9DS0AccelerometerFilterBandwidth::Hz773;
//     }

//     let (gyro, accel) = get_default_lsm9ds0_linux_i2c_devices().unwrap();

//     match LSM9DS0::new(accel, gyro, gyro_settings, accel_mag_settings) {
//         Ok(lsm9ds0) => lsm9ds0,
//         Err(e) => {
//             panic!("Couldn't initialize LSM9DS0");
//         }
//     }
// }
