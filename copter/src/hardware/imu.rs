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
use typenum::U200;
use num::traits::Zero;

type Matrix200x9 = Matrix<f32, U200, U9, MatrixArray<f32, U200, U9>>;
type Vector9 = VectorN<f32, U9>;
type Vector200 = VectorN<f32, U200>;

use configurations::{Calibrations, Config, Ellipsoid, Simple};
use logger::ModuleLogger;

use super::mock::MockSensor;

pub struct IMU {
    gyroscope: Rc<RefCell<Gyroscope<Error = LinuxI2CError>>>,
    accelerometer: Rc<RefCell<Accelerometer<Error = LinuxI2CError>>>,
    magnetometer: Rc<RefCell<Magnetometer<Error = LinuxI2CError>>>,
    logger: ModuleLogger,
    // calibrations: Calibrations,
}

impl IMU {
    pub fn new() -> Result<IMU, ()> {
        let calibs = Calibrations::new();
        let config = Config::new().unwrap();
        let logger = ModuleLogger::new("IMU", None);

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

    pub fn read_gyroscope(&mut self) -> Result<Vector3<f32>, ()> {
        match self.gyroscope.borrow_mut().angular_rate_reading() {
            Ok(angular_rate) => Ok(Vector3::new(angular_rate.x, angular_rate.y, angular_rate.z)),
            Err(_) => {
                self.logger.error("Couldn't read gyroscope.");
                return Err(());
            }
        }
    }

    pub fn read_accelerometer(&mut self) -> Result<Vector3<f32>, ()> {
        match self.accelerometer.borrow_mut().acceleration_reading() {
            Ok(acceleration) => Ok(Vector3::new(acceleration.x, acceleration.y, acceleration.z)),
            Err(_) => {
                self.logger.error("Couldn't read accelerometer.");
                return Err(());
            }
        }
    }

    pub fn read_magnetometer(&mut self) -> Result<Vector3<f32>, ()> {
        match self.magnetometer.borrow_mut().magnetic_reading() {
            Ok(magnetic) => Ok(Vector3::new(magnetic.x, magnetic.y, magnetic.z)),
            Err(_) => {
                self.logger.error("Couldn't read magnetometer.");
                return Err(());
            }
        }
    }

    pub fn calibrate_sensors(&mut self) {
        let mut calibrations = Calibrations::new().unwrap();
        self.calibrate_magnetometer(&mut calibrations);
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
            let magnetic_reading = self.read_magnetometer().unwrap();
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

            sleep(Duration::from_millis(125));
        }

        // Rotated ellipsoid fitting
        let mut ones: Vector200 = Vector200::from_element(1.0);

        // v[9x1] = (D^T D)^(-1)D^T
        let v = (D.transpose() * D).try_inverse().unwrap() * (&D.transpose() * ones);

        // Auxillary matrices
        let A_4: Matrix4<f32> = Matrix4::new(
            v.data[0],
            v.data[3],
            v.data[4],
            v.data[6],
            v.data[3],
            v.data[1],
            v.data[5],
            v.data[7],
            v.data[4],
            v.data[5],
            v.data[2],
            v.data[8],
            v.data[6],
            v.data[7],
            v.data[8],
            -1.0,
        );
        let A_3: Matrix3<f32> = Matrix3::new(
            v.data[0],
            v.data[3],
            v.data[4],
            v.data[3],
            v.data[1],
            v.data[5],
            v.data[4],
            v.data[5],
            v.data[2],
        );
        let v_ghi: Vector3<f32> = Vector3::new(v.data[6], v.data[7], v.data[8]);

        // Compute offsets
        let offsets = -1.0 * A_3.try_inverse().unwrap() * v_ghi;

        // More auxillary matrices
        let T: Matrix4<f32> = Matrix4::new(
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            0.0,
            offsets.data[0],
            offsets.data[1],
            offsets.data[2],
            1.0,
        );
        let B_4 = T * A_4 * T.transpose();
        let b_44 = -1.0 * B_4.data[15];
        let B_3: Matrix3<f32> = B_4.fixed_resize(0.0) / b_44;

        // Compute gains and rotation
        let eigen_decomp = B_3.symmetric_eigen();
        let gains: Vector3<f32> = eigen_decomp.eigenvalues;
        let rotation: Matrix3<f32> = eigen_decomp.eigenvectors;

        calibs.magnetometer = Some(Ellipsoid::new(offsets, rotation, gains));
    }

    fn calibrate_gyroscope(&mut self, calibs: &mut Calibrations) {
        println!("Place the drone on a level surface.");
        sleep(Duration::from_secs(5));
        println!("3...");
        sleep(Duration::from_secs(1));
        println!("2...");
        sleep(Duration::from_secs(1));
        println!("1...");
        sleep(Duration::from_secs(1));
        println!("Go!");

        let mut offsets = self.read_gyroscope().unwrap();
        for i in 0..200 {
            offsets += self.read_gyroscope().unwrap();
            sleep(Duration::from_millis(20));
        }

        offsets /= 201.0;

        calibs.gyroscope = Some(Simple::new(offsets));
    }

    fn calibrate_accelerometer(&mut self, calibs: &mut Calibrations) {
        println!("Place the drone on a level surface.");
        sleep(Duration::from_secs(5));
        println!("3...");
        sleep(Duration::from_secs(1));
        println!("2...");
        sleep(Duration::from_secs(1));
        println!("1...");
        sleep(Duration::from_secs(1));
        println!("Go!");

        let mut offsets = self.read_accelerometer().unwrap();
        for i in 0..200 {
            offsets += self.read_accelerometer().unwrap();
            sleep(Duration::from_millis(20));
        }

        offsets /= 201.0;

        calibs.accelerometer = Some(Ellipsoid::new(offsets, Matrix3::zero(), Vector3::zero()));
    }
}

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


//     let (mut barometer, mut thermometer, mut gyroscope, mut accelerometer, mut magnetometer) =
//         get_sensors();
//     println!("[Sensors]: Place drone on a flat surface. Then press enter.");
//     let mut input = String::new();
//     stdin().read_line(&mut input).expect("Error");
//     let mut acceleration_calibration = Vec3::zeros();
//     let mut gyroscope_calibration = Vec3::zeros();
//     println!("[Sensors]: Calibrating gyroscope. Leave the drone still.");

//     for i in 0..50 {
//         gyroscope_calibration =
//             gyroscope_calibration + gyroscope.borrow_mut().angular_rate_reading().unwrap();
//         thread::sleep_ms(50);
//     }

//     println!("[Sensors]: Rotate 90 degrees then press enter.");
//     let mut input = String::new();
//     stdin().read_line(&mut input).expect("Error");

//     for i in 0..50 {
//         gyroscope_calibration =
//             gyroscope_calibration + gyroscope.borrow_mut().angular_rate_reading().unwrap();
//         thread::sleep_ms(50);
//     }

//     println!("[Sensors]: Rotate 90 degrees then press enter.");
//     let mut input = String::new();
//     stdin().read_line(&mut input).expect("Error");

//     for i in 0..50 {
//         gyroscope_calibration =
//             gyroscope_calibration + gyroscope.borrow_mut().angular_rate_reading().unwrap();
//         thread::sleep_ms(50);
//     }

//     println!("[Sensors]: Rotate 90 degrees then press enter.");
//     let mut input = String::new();
//     stdin().read_line(&mut input).expect("Error");

//     for i in 0..50 {
//         gyroscope_calibration =
//             gyroscope_calibration + gyroscope.borrow_mut().angular_rate_reading().unwrap();
//         thread::sleep_ms(50);
//     }

//     gyroscope_calibration = gyroscope_calibration / 200.0;

//     println!("[Sensors]: Calibrating magnetometer and accelerometer with ellipsoid fitting.");
//     println!("Press enter to continue then slowly tumble rotate the drone without any extra accelerations along the drone's axes.");
//     let mut input = String::new();
//     stdin().read_line(&mut input).expect("Error");

//     let mut accelerometer_readings: Vec<MultiSensorData> = Vec::new();
//     let mut magnetometer_readings: Vec<MultiSensorData> = Vec::new();

//     for i in 0..15000 {
//         let mag_reading = magnetometer.borrow_mut().magnetic_reading().unwrap();
//         magnetometer_readings.push(mag_reading);
//         thread::sleep_ms(10);
//     }
//     // 'accel_mag: loop {
//     //     println!("{}", Green.paint("[Sensors]: Rotate the drone arbitrarily and rest it. Press enter to get reading. Type 'stop' to stop."));
//     //     let mut input = String::new();
//     //     stdin().read_line(&mut input).expect("Error");
//     //     match input.trim() {
//     //         "stop" => { break 'accel_mag; },
//     //         _ => {
//     //             let mut accel_mean = MultiSensorData::zeros();
//     //             let mut mag_mean = MultiSensorData::zeros();
//     //             for i in 0..150 {
//     //                 let accel_reading = accelerometer.borrow_mut().acceleration_reading().unwrap();
//     //                 let mag_reading = magnetometer.borrow_mut().magnetic_reading().unwrap();
//     //                 accel_mean = accel_mean + accel_reading / 150.0;
//     //                 mag_mean = mag_mean + mag_reading / 150.0;
//     //                 thread::sleep_ms(10);
//     //             }
//     //             accelerometer_readings.push(accel_mean);
//     //             magnetometer_readings.push(mag_mean);
//     //         }
//     //     }
//     // }

//     for i in 0..magnetometer_readings.len() {
//         println!(
//             "{},{},{}",
//             magnetometer_readings[i].x, magnetometer_readings[i].y, magnetometer_readings[i].z
//         );
//     }
//     // for i in 0..magnetometer_readings.len() {
//     //     println!("{},{},{},{},{},{}", accelerometer_readings[i].x, accelerometer_readings[i].y, accelerometer_readings[i].z, magnetometer_readings[i].x, magnetometer_readings[i].y, magnetometer_readings[i].z);
//     // }

//     // println!("{}", Cyan.paint("[Sensors]: Finished gathering data. Computing calibration settings now."));
//     // let mut accelerometer_D: Vec<f32> = Vec::new();
//     // let mut magnetometer_D: Vec<f32> = Vec::new();
//     //
//     // for reading in accelerometer_readings {
//     //     let D_1 = reading.x * reading.x;
//     //     let D_2 = reading.y * reading.y;
//     //     let D_3 = reading.z * reading.z;
//     //     let D_4 = reading.x * reading.y * 2.0;
//     //     let D_5 = reading.x * reading.z * 2.0;
//     //     let D_6 = reading.y * reading.z * 2.0;
//     //     let D_7 = reading.x * 2.0;
//     //     let D_8 = reading.y * 2.0;
//     //     let D_9 = reading.z * 2.0;
//     //     accelerometer_D.push(D_1);
//     //     accelerometer_D.push(D_2);
//     //     accelerometer_D.push(D_3);
//     //     accelerometer_D.push(D_4);
//     //     accelerometer_D.push(D_5);
//     //     accelerometer_D.push(D_6);
//     //     accelerometer_D.push(D_7);
//     //     accelerometer_D.push(D_8);
//     //     accelerometer_D.push(D_9);
//     // }
//     //
//     // for reading in magnetometer_readings {
//     //     println!("{},{},{}", reading.x, reading.y, reading.z);
//     //     let D_1 = reading.x * reading.x;
//     //     let D_2 = reading.y * reading.y;
//     //     let D_3 = reading.z * reading.z;
//     //     let D_4 = reading.x * reading.y * 2.0;
//     //     let D_5 = reading.x * reading.z * 2.0;
//     //     let D_6 = reading.y * reading.z * 2.0;
//     //     let D_7 = reading.x * 2.0;
//     //     let D_8 = reading.y * 2.0;
//     //     let D_9 = reading.z * 2.0;
//     //     magnetometer_D.push(D_1);
//     //     magnetometer_D.push(D_2);
//     //     magnetometer_D.push(D_3);
//     //     magnetometer_D.push(D_4);
//     //     magnetometer_D.push(D_5);
//     //     magnetometer_D.push(D_6);
//     //     magnetometer_D.push(D_7);
//     //     magnetometer_D.push(D_8);
//     //     magnetometer_D.push(D_9);
//     // }
//     // return;
//     //
//     //
//     // // Compute 9 unknowns 'v'
//     // let ones: DMatrix<f32> = DMatrix::from_element(reading_len, 1, 1.0);
//     // let D_mag: DMatrix<f32> = DMatrix::from_iterator(reading_len, 9, magnetometer_D.iter().cloned());
//     // let D_mag_transpose = D_mag.transpose();
//     //
//     // let v = (D_mag_transpose.clone() * D_mag).try_inverse().unwrap() * (D_mag_transpose * ones);
//     //
//     // // Compute auxiliary matricies
//     // let a = v.data[0];
//     // let b = v.data[1];
//     // let c = v.data[2];
//     // let d = v.data[3];
//     // let e = v.data[4];
//     // let f = v.data[5];
//     // let g = v.data[6];
//     // let h = v.data[7];
//     // let i = v.data[8];
//     //
//     // let v_ghi = DMatrix::from_row_slice(3, 1, &[g, h, i]);
//     // let A_4 = DMatrix::from_column_slice(4, 4, &[a, d, e, g, d, b, f, h, e, f, c, i, g, h, i, -1.0]);
//     // let A_3 = DMatrix::from_column_slice(3, 3, &[a, d, e, d, b, f, e, f, c]);
//     // let o = -1.0 * A_3.try_inverse().unwrap() * v_ghi;
//     //
//     // let T = DMatrix::from_column_slice(4, 4, &[1.0, 0.0, 0.0, 0.0,
//     //                                            0.0, 1.0, 0.0, 0.0,
//     //                                            0.0, 0.0, 1.0, 0.0,
//     //                                            o.data[0], o.data[1], o.data[2], 1.0]);
//     //
//     // let B_4 = T.clone() * A_4 * T.transpose();
//     // let B_3 = B_4.slice((0, 0), (3, 3)) / B_4.data[15];
//     //
//     // let qr = QR::new(B_3);
//     //
//     //
//     // // let D: Matrix<f32, na::U1000, na::U9, na::MatrixArray<f32, na::U1000, na::U9>> = Matrix::new();
//     //
//     // acceleration_calibration = acceleration_calibration / 200.0;
//     // acceleration_calibration.z -= 1.0;
//     //
//     // println!("Accelerometer calibration values: {:?}", acceleration_calibration);
//     // println!("Gyroscope calibration values: {:?}", gyroscope_calibration);

//     // let calibs = SensorCalibrations {
//     //     gyro_x: gyroscope_calibration.x,
//     //     gyro_y: gyroscope_calibration.y,
//     //     gyro_z: gyroscope_calibration.z,
//     //     accel_x: acceleration_calibration.x,
//     //     accel_y: acceleration_calibration.y,
//     //     accel_z: acceleration_calibration.z
//     // };
//     //
//     // calibs.write_calibration();
// }

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
