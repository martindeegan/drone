use super::i2cdev_bmp180::*;
use super::i2cdev_bmp280::*;
use super::i2cdev_l3gd20::*;
use super::i2cdev_lsm303d::*;
use super::i2cdev_lsm9ds0::*;
use super::i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};
use super::i2cdev::core::I2CDevice;
use super::i2csensors::*;

use logger::{FlightLogger, ModuleLogger};
use configurations::{Calibrations, Config};
use rulinalg::vector::Vector;
use rulinalg::matrix::Matrix;

use time::{Duration, PreciseTime};

use std::rc::Rc;
use std::cell::RefCell;
use std::thread;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::io::stdin;
use std::collections::VecDeque;
use std::vec::Vec;

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

fn convert_to_lin_alg(vec: Vec3) -> Vector<f32> {
    let conversion: Vector<f32> = Vector::new(vec![vec.x, vec.y, vec.z]);
    conversion
}

pub struct SensorInput {
    pub angular_rate: Vector<f32>,
    pub acceleration: Vector<f32>,
    pub magnetic_reading: Vector<f32>,
    pub temperature: f32,
    pub pressure: f32,
}

pub struct SensorManager {
    barometer: Rc<RefCell<Barometer<Error = LinuxI2CError>>>,
    thermometer: Rc<RefCell<Thermometer<Error = LinuxI2CError>>>,
    gyroscope: Rc<RefCell<Gyroscope<Error = LinuxI2CError>>>,
    accelerometer: Rc<RefCell<Accelerometer<Error = LinuxI2CError>>>,
    magnetometer: Rc<RefCell<Magnetometer<Error = LinuxI2CError>>>,
}

impl SensorManager {
    pub fn new() -> SensorManager {
        let config = Config::new();
        let (baro, thermo, gyro, accel, mag) = get_sensors();
        SensorManager {
            barometer: baro,
            thermometer: thermo,
            gyroscope: gyro,
            accelerometer: accel,
            magnetometer: mag,
        }
    }

    pub fn read_sensors(&mut self) -> SensorInput {
        let angular_rate = match self.gyroscope.borrow_mut().angular_rate_reading() {
            Ok(reading) => convert_to_lin_alg(reading),
            Err(_) => Vector::zeros(3),
        };
        let acceleration = match self.gyroscope.borrow_mut().acceleration_reading() {
            Ok(reading) => convert_to_lin_alg(reading),
            Err(_) => Vector::zeros(3),
        };
        let magnetism = match self.magnetometer.borrow_mut().magnetic_reading() {
            Ok(reading) => convert_to_lin_alg(reading),
            Err(_) => Vector::zeros(3),
        };
        let pressure = match self.barometer.borrow_mut().pressure_kpa() {
            Ok(reading) => reading,
            Err(_) => 0.0,
        };
        let temperature = match self.barometer.borrow_mut().temperature_celsius() {
            Ok(reading) => reading,
            Err(_) => 0.0,
        };

        SensorInput {
            angular_rate: angular_rate,
            acceleration: acceleration,
            magnetic_reading: magnetism,
            temperature: temperature,
            pressure: pressure,
        }
    }
}

fn get_sensors() -> (
    Rc<RefCell<Barometer<Error = LinuxI2CError>>>,
    Rc<RefCell<Thermometer<Error = LinuxI2CError>>>,
    Rc<RefCell<Gyroscope<Error = LinuxI2CError>>>,
    Rc<RefCell<Accelerometer<Error = LinuxI2CError>>>,
    Rc<RefCell<Magnetometer<Error = LinuxI2CError>>>,
) {
    let config = Config::new();
    let mut barometer: Option<Rc<RefCell<Barometer<Error = LinuxI2CError>>>> = None;
    let mut thermometer: Option<Rc<RefCell<Thermometer<Error = LinuxI2CError>>>> = None;
    let mut gyroscope: Option<Rc<RefCell<Gyroscope<Error = LinuxI2CError>>>> = None;
    let mut accelerometer: Option<Rc<RefCell<Accelerometer<Error = LinuxI2CError>>>> = None;
    let mut magnetometer: Option<Rc<RefCell<Magnetometer<Error = LinuxI2CError>>>> = None;

<<<<<<< HEAD

    match sensor.as_ref() {
        "BMP180" => {
            let bmp180 = Rc::new(RefCell::new(
                get_bmp180(config.hardware.barometer.update_rate).unwrap(),
            ));
            barometer = Some(bmp180.clone());
            thermometer = Some(bmp180.clone());
        }
        "BMP280" => {
            let bmp280 = Rc::new(RefCell::new(get_bmp280(
                config.hardware.barometer.update_rate,
            )));
            barometer = Some(bmp280.clone());
            thermometer = Some(bmp280.clone());
        }
        "L3GD20" => {
            let l3gd20 = Rc::new(RefCell::new(get_l3gd20(
                config.hardware.gyroscope.update_rate,
            )));
            gyroscope = Some(l3gd20.clone());
        }
        "LSM303D" => {
            let lsm303d = Rc::new(RefCell::new(get_lsm303d(
                config.hardware.accelerometer.update_rate,
            )));
            accelerometer = Some(lsm303d.clone());
            magnetometer = Some(lsm303d.clone());
        }
        "LSM9DS0" => {
            let lsm9ds0 = Rc::new(RefCell::new(get_lsm9ds0(
                config.hardware.gyroscope.update_rate,
            )));
            gyroscope = Some(lsm9ds0.clone());
            accelerometer = Some(lsm9ds0.clone());
            magnetometer = Some(lsm9ds0.clone());
        }
        _ => {
            return panic!("Undefined sensor: {}.", sensor);
=======
    
        match sensor.as_ref() {
            "BMP180" => {
                let bmp180 = Rc::new(RefCell::new(get_bmp180(config.hardware.barometer.update_rate).unwrap()));
                barometer = Some(bmp180.clone());
                thermometer = Some(bmp180.clone());
            },
            "BMP280" => {
                let bmp280 = Rc::new(RefCell::new(get_bmp280(config.hardware.barometer.update_rate)));
                barometer = Some(bmp280.clone());
                thermometer = Some(bmp280.clone());
            },
            "L3GD20" => {
                let l3gd20 = Rc::new(RefCell::new(get_l3gd20(config.hardware.gyroscope.update_rate)));
                gyroscope = Some(l3gd20.clone());
            }
            "LSM303D" => {
                let lsm303d = Rc::new(RefCell::new(get_lsm303d(config.hardware.accelerometer.update_rate)));
                accelerometer = Some(lsm303d.clone());
                magnetometer = Some(lsm303d.clone());
            },
            "LSM9DS0" => {
                let lsm9ds0 = Rc::new(RefCell::new(get_lsm9ds0(config.hardware.gyroscope.update_rate)));
                gyroscope = Some(lsm9ds0.clone());
                accelerometer = Some(lsm9ds0.clone());
                magnetometer = Some(lsm9ds0.clone());
            },
            _ => {
                return panic!("Undefined sensor: {}.", sensor);
            }
>>>>>>> 475611dd57d28da45f15a2e19a603c45099a08ac
        }

    match barometer {
        Some(_) => {}
        None => {
            panic!("Error: No barometer set.");
        }
    }

    match thermometer {
        Some(_) => {}
        None => {
            panic!("Error: No thermometer set.");
        }
    }

    match gyroscope {
        Some(_) => {}
        None => {
            panic!("Error: No gyroscope set.");
        }
    }

    match accelerometer {
        Some(_) => {}
        None => {
            panic!("Error: No accelerometer set.");
        }
    }

    match magnetometer {
        Some(_) => {}
        None => {
            panic!("Error: No magnetometer set.");
        }
    }

    (
        barometer.unwrap(),
        thermometer.unwrap(),
        gyroscope.unwrap(),
        accelerometer.unwrap(),
        magnetometer.unwrap(),
    )
}

fn read_calibration_values() -> (
    MultiSensorData,
    MultiSensorData,
    MultiSensorData,
    Matrix3<f32>,
) {
    let calibs = SensorCalibrations::new();
    (
        MultiSensorData {
            x: calibs.gyro_x,
            y: calibs.gyro_y,
            z: calibs.gyro_z,
        },
        MultiSensorData {
            x: calibs.accel_x,
            y: calibs.accel_y,
            z: calibs.accel_z,
        },
        MultiSensorData {
            x: calibs.mag_ofs_x,
            y: calibs.mag_ofs_y,
            z: calibs.mag_ofs_z,
        },
        Matrix3::new(
            calibs.mag_rot_11,
            calibs.mag_rot_12,
            calibs.mag_rot_13,
            calibs.mag_rot_21,
            calibs.mag_rot_22,
            calibs.mag_rot_23,
            calibs.mag_rot_31,
            calibs.mag_rot_32,
            calibs.mag_rot_33,
        ),
    )
}

// Returns (gyro_accel_rx, mag_rx, thermo_baro_rx);
pub fn start_sensors() -> (Receiver<SensorInput>) {
    let config = Config::new();
    let sensor_poll_rate = config.sensor_sample_frequency;
    let sensor_poll_delay = (1000000000 / sensor_poll_rate) as i64;

    let (sensor_tx, sensor_rx): (Sender<SensorInput>, Receiver<SensorInput>) = channel();

    let magnetometer_counter = sensor_poll_rate / 100;

<<<<<<< HEAD
    thread::Builder::new()
        .name("Sensor Thread".to_string())
        .spawn(move || {
            let loop_duration = Duration::nanoseconds(sensor_poll_delay);
            let mut count = 0;
            let (
                mut barometer,
                mut thermometer,
                mut gyroscope,
                mut accelerometer,
                mut magnetometer,
            ) = get_sensors();

            let (mut gyro_calib, mut accel_calib, mut mag_ofs, calib_matrix) =
                read_calibration_values();
            loop {
                let start_time = PreciseTime::now();

                let mut input = SensorInput {
                    angular_rate: MultiSensorData::zeros(),
                    acceleration: MultiSensorData::zeros(),
                    magnetic_reading: None,
                    temperature: 0.0,
                    pressure: 0.0,
                };

                match gyroscope.borrow_mut().angular_rate_reading() {
                    Ok(angular_rate) => {
                        // let alpha = 0.00003;
                        // gyro_calib = gyro_calib * (1.0 - alpha) + angular_rate * alpha;
                        // println!("gyro calib: {:?}", gyro_calib);
                        let alpha = 0.005;
                        gyro_calib = gyro_calib * (1.0 - alpha) + angular_rate * alpha;
                        input.angular_rate = angular_rate - gyro_calib;
                        input.angular_rate.y *= -1.0;
                        input.angular_rate.z *= -1.0;
                        // println!("gyro: {:?}", input.angular_rate);
                    }
                    Err(e) => {}
                }

                match accelerometer.borrow_mut().acceleration_reading() {
                    Ok(mut acceleration) => {
                        acceleration = acceleration - Vec3 {
                            x: -0.176987750378325,
                            y: -0.0643969179666939,
                            z: 0.0368680289439468,
                        };
                        acceleration.x = acceleration.x * 1.00126703403732
                            + acceleration.y * 0.00662310660530386
                            + acceleration.z * -0.000510382093277389;
                        acceleration.y = acceleration.x * 0.00662310660530371
                            + acceleration.y * 0.997902617513774
                            + acceleration.z * -0.00135008782991750;
                        acceleration.z = acceleration.x * -0.000510382093277452
                            + acceleration.y * -0.00135008782991756
                            + acceleration.z * 1.00418778375736;

                        input.acceleration = acceleration;
                        input.acceleration.x *= -1.0;
                        // println!("accel: {:?}", input.acceleration);
                    }
                    Err(e) => {}
                }

                if count % magnetometer_counter == 0 {
                    match magnetometer.borrow_mut().magnetic_reading() {
                        Ok(mut magnetism) => {
                            // println!("{:?}", magnetism);
                            magnetism = magnetism - Vec3 {
                                x: -0.189575328870267,
                                y: 0.0597052440863164,
                                z: 0.0285933538364212,
                            };;
                            magnetism.x = magnetism.x * 1.66314648899102
                                + magnetism.y * -0.0365578805671075
                                + magnetism.z * 0.0481109910962223;
                            magnetism.y = magnetism.x * -0.0365578805671075
                                + magnetism.y * 2.09504054789850
                                + magnetism.z * 0.0379828626601234;
                            magnetism.z = magnetism.x * 0.0481109910962223
                                + magnetism.y * 0.0379828626601234
                                + magnetism.z * 2.06953892520177;
                            input.magnetic_reading = Some(magnetism);
                        }
                        Err(e) => {}
                    }
                }
=======
    thread::Builder::new().name("Sensor Thread".to_string()).spawn(move || {
        let loop_duration = Duration::nanoseconds(sensor_poll_delay);
        let mut count = 0;
        let (mut barometer, mut thermometer, mut gyroscope, mut accelerometer, mut magnetometer) = get_sensors();

        let (mut gyro_calib, mut accel_calib, mut mag_ofs, calib_matrix) = read_calibration_values();
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
                    // let alpha = 0.00003;
                    // gyro_calib = gyro_calib * (1.0 - alpha) + angular_rate * alpha;
                    // println!("gyro calib: {:?}", gyro_calib);
                    let alpha = 0.005;
                    gyro_calib = gyro_calib * (1.0 - alpha) + angular_rate * alpha;
                    input.angular_rate = angular_rate - gyro_calib;
                    input.angular_rate.y *= -1.0;
                    input.angular_rate.z *= -1.0;
                    // println!("gyro: {:?}", input.angular_rate);
                },
                Err(e) => {}
            }

            match accelerometer.borrow_mut().acceleration_reading() {
                Ok(mut acceleration) => {
                    acceleration = acceleration - Vec3 { x: -0.176987750378325, y: -0.0643969179666939, z: 0.0368680289439468 };
                    acceleration.x = acceleration.x * 1.00126703403732 + acceleration.y * 0.00662310660530386 + acceleration.z * -0.000510382093277389;
                    acceleration.y = acceleration.x * 0.00662310660530371 + acceleration.y * 0.997902617513774 + acceleration.z * -0.00135008782991750;
                    acceleration.z = acceleration.x * -0.000510382093277452 + acceleration.y * -0.00135008782991756 + acceleration.z * 1.00418778375736;

                    input.acceleration = acceleration;
                    input.acceleration.x *= -1.0;
                    // println!("accel: {:?}", input.acceleration);
                },
                Err(e) => {}
            }
>>>>>>> 475611dd57d28da45f15a2e19a603c45099a08ac

                match thermometer.borrow_mut().temperature_celsius() {
                    Ok(temp) => {
                        input.temperature = temp;
                    }
                    Err(e) => {}
                }

                match barometer.borrow_mut().pressure_kpa() {
                    Ok(pressure) => {
                        input.pressure = pressure;
                    }
                    Err(e) => {}
                }

                match self.gps.try_recv() {
                    Ok(data) => {}
                    Err(e) => {}
                }

<<<<<<< HEAD
                sensor_tx.send(input);
=======
            match self.gps.try_recv() {
                Ok(data) => {},
                Err(e) => {}
            }

            sensor_tx.send(input);
>>>>>>> 475611dd57d28da45f15a2e19a603c45099a08ac

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
<<<<<<< HEAD
    let (mut barometer, mut thermometer, mut gyroscope, mut accelerometer, mut magnetometer) =
        get_sensors();
=======
    let (mut barometer, mut thermometer, mut gyroscope, mut accelerometer, mut magnetometer) = get_sensors();
>>>>>>> 475611dd57d28da45f15a2e19a603c45099a08ac
    println!("[Sensors]: Place drone on a flat surface. Then press enter.");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error");
    let mut acceleration_calibration = Vec3::zeros();
    let mut gyroscope_calibration = Vec3::zeros();
    println!("[Sensors]: Calibrating gyroscope. Leave the drone still.");

    for i in 0..50 {
        gyroscope_calibration =
            gyroscope_calibration + gyroscope.borrow_mut().angular_rate_reading().unwrap();
        thread::sleep_ms(50);
    }

    println!("[Sensors]: Rotate 90 degrees then press enter.");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error");

    for i in 0..50 {
        gyroscope_calibration =
            gyroscope_calibration + gyroscope.borrow_mut().angular_rate_reading().unwrap();
        thread::sleep_ms(50);
    }

    println!("[Sensors]: Rotate 90 degrees then press enter.");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error");

    for i in 0..50 {
        gyroscope_calibration =
            gyroscope_calibration + gyroscope.borrow_mut().angular_rate_reading().unwrap();
        thread::sleep_ms(50);
    }

    println!("[Sensors]: Rotate 90 degrees then press enter.");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error");

    for i in 0..50 {
        gyroscope_calibration =
            gyroscope_calibration + gyroscope.borrow_mut().angular_rate_reading().unwrap();
        thread::sleep_ms(50);
    }

    gyroscope_calibration = gyroscope_calibration / 200.0;

    println!("[Sensors]: Calibrating magnetometer and accelerometer with ellipsoid fitting.");
    println!("Press enter to continue then slowly tumble rotate the drone without any extra accelerations along the drone's axes.");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error");

    let mut accelerometer_readings: Vec<MultiSensorData> = Vec::new();
    let mut magnetometer_readings: Vec<MultiSensorData> = Vec::new();

    for i in 0..15000 {
        let mag_reading = magnetometer.borrow_mut().magnetic_reading().unwrap();
        magnetometer_readings.push(mag_reading);
        thread::sleep_ms(10);
    }
    // 'accel_mag: loop {
    //     println!("{}", Green.paint("[Sensors]: Rotate the drone arbitrarily and rest it. Press enter to get reading. Type 'stop' to stop."));
    //     let mut input = String::new();
    //     stdin().read_line(&mut input).expect("Error");
    //     match input.trim() {
    //         "stop" => { break 'accel_mag; },
    //         _ => {
    //             let mut accel_mean = MultiSensorData::zeros();
    //             let mut mag_mean = MultiSensorData::zeros();
    //             for i in 0..150 {
    //                 let accel_reading = accelerometer.borrow_mut().acceleration_reading().unwrap();
    //                 let mag_reading = magnetometer.borrow_mut().magnetic_reading().unwrap();
    //                 accel_mean = accel_mean + accel_reading / 150.0;
    //                 mag_mean = mag_mean + mag_reading / 150.0;
    //                 thread::sleep_ms(10);
    //             }
    //             accelerometer_readings.push(accel_mean);
    //             magnetometer_readings.push(mag_mean);
    //         }
    //     }
    // }

    for i in 0..magnetometer_readings.len() {
        println!(
            "{},{},{}",
            magnetometer_readings[i].x, magnetometer_readings[i].y, magnetometer_readings[i].z
        );
    }
    // for i in 0..magnetometer_readings.len() {
    //     println!("{},{},{},{},{},{}", accelerometer_readings[i].x, accelerometer_readings[i].y, accelerometer_readings[i].z, magnetometer_readings[i].x, magnetometer_readings[i].y, magnetometer_readings[i].z);
    // }

    // println!("{}", Cyan.paint("[Sensors]: Finished gathering data. Computing calibration settings now."));
    // let mut accelerometer_D: Vec<f32> = Vec::new();
    // let mut magnetometer_D: Vec<f32> = Vec::new();
    //
    // for reading in accelerometer_readings {
    //     let D_1 = reading.x * reading.x;
    //     let D_2 = reading.y * reading.y;
    //     let D_3 = reading.z * reading.z;
    //     let D_4 = reading.x * reading.y * 2.0;
    //     let D_5 = reading.x * reading.z * 2.0;
    //     let D_6 = reading.y * reading.z * 2.0;
    //     let D_7 = reading.x * 2.0;
    //     let D_8 = reading.y * 2.0;
    //     let D_9 = reading.z * 2.0;
    //     accelerometer_D.push(D_1);
    //     accelerometer_D.push(D_2);
    //     accelerometer_D.push(D_3);
    //     accelerometer_D.push(D_4);
    //     accelerometer_D.push(D_5);
    //     accelerometer_D.push(D_6);
    //     accelerometer_D.push(D_7);
    //     accelerometer_D.push(D_8);
    //     accelerometer_D.push(D_9);
    // }
    //
    // for reading in magnetometer_readings {
    //     println!("{},{},{}", reading.x, reading.y, reading.z);
    //     let D_1 = reading.x * reading.x;
    //     let D_2 = reading.y * reading.y;
    //     let D_3 = reading.z * reading.z;
    //     let D_4 = reading.x * reading.y * 2.0;
    //     let D_5 = reading.x * reading.z * 2.0;
    //     let D_6 = reading.y * reading.z * 2.0;
    //     let D_7 = reading.x * 2.0;
    //     let D_8 = reading.y * 2.0;
    //     let D_9 = reading.z * 2.0;
    //     magnetometer_D.push(D_1);
    //     magnetometer_D.push(D_2);
    //     magnetometer_D.push(D_3);
    //     magnetometer_D.push(D_4);
    //     magnetometer_D.push(D_5);
    //     magnetometer_D.push(D_6);
    //     magnetometer_D.push(D_7);
    //     magnetometer_D.push(D_8);
    //     magnetometer_D.push(D_9);
    // }
    // return;
    //
    //
    // // Compute 9 unknowns 'v'
    // let ones: DMatrix<f32> = DMatrix::from_element(reading_len, 1, 1.0);
    // let D_mag: DMatrix<f32> = DMatrix::from_iterator(reading_len, 9, magnetometer_D.iter().cloned());
    // let D_mag_transpose = D_mag.transpose();
    //
    // let v = (D_mag_transpose.clone() * D_mag).try_inverse().unwrap() * (D_mag_transpose * ones);
    //
    // // Compute auxiliary matricies
    // let a = v.data[0];
    // let b = v.data[1];
    // let c = v.data[2];
    // let d = v.data[3];
    // let e = v.data[4];
    // let f = v.data[5];
    // let g = v.data[6];
    // let h = v.data[7];
    // let i = v.data[8];
    //
    // let v_ghi = DMatrix::from_row_slice(3, 1, &[g, h, i]);
    // let A_4 = DMatrix::from_column_slice(4, 4, &[a, d, e, g, d, b, f, h, e, f, c, i, g, h, i, -1.0]);
    // let A_3 = DMatrix::from_column_slice(3, 3, &[a, d, e, d, b, f, e, f, c]);
    // let o = -1.0 * A_3.try_inverse().unwrap() * v_ghi;
    //
    // let T = DMatrix::from_column_slice(4, 4, &[1.0, 0.0, 0.0, 0.0,
    //                                            0.0, 1.0, 0.0, 0.0,
    //                                            0.0, 0.0, 1.0, 0.0,
    //                                            o.data[0], o.data[1], o.data[2], 1.0]);
    //
    // let B_4 = T.clone() * A_4 * T.transpose();
    // let B_3 = B_4.slice((0, 0), (3, 3)) / B_4.data[15];
    //
    // let qr = QR::new(B_3);
    //
    //
    // // let D: Matrix<f32, na::U1000, na::U9, na::MatrixArray<f32, na::U1000, na::U9>> = Matrix::new();
    //
    // acceleration_calibration = acceleration_calibration / 200.0;
    // acceleration_calibration.z -= 1.0;
    //
    // println!("Accelerometer calibration values: {:?}", acceleration_calibration);
    // println!("Gyroscope calibration values: {:?}", gyroscope_calibration);

    // let calibs = SensorCalibrations {
    //     gyro_x: gyroscope_calibration.x,
    //     gyro_y: gyroscope_calibration.y,
    //     gyro_z: gyroscope_calibration.z,
    //     accel_x: acceleration_calibration.x,
    //     accel_y: acceleration_calibration.y,
    //     accel_z: acceleration_calibration.z
    // };
    //
    // calibs.write_calibration();
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
        power_mode: BMP280PowerMode::NormalMode,
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
        high_pass_filter_configuration: Some(L3GD20HighPassFilterCutOffConfig::HPCF_3),
    };

    if frequency <= 190 {
        gyro_settings.DR = L3GD20GyroscopeDataRate::Hz190;
        gyro_settings.BW = L3GD20GyroscopeBandwidth::BW2;
    } else if frequency <= 380 {
        gyro_settings.DR = L3GD20GyroscopeDataRate::Hz380;
        gyro_settings.BW = L3GD20GyroscopeBandwidth::BW3;
    } else {
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
        magnetometer_sensitivity: LSM303DMagnetometerFS::gauss2,
    };

    if frequency <= 200 {
        accel_mag_settings.accelerometer_data_rate = LSM303DAccelerometerUpdateRate::Hz200;
        accel_mag_settings.accelerometer_anti_alias_filter_bandwidth =
            LSM303DAccelerometerFilterBandwidth::Hz50;
    } else if frequency <= 400 {
        accel_mag_settings.accelerometer_data_rate = LSM303DAccelerometerUpdateRate::Hz400;
        accel_mag_settings.accelerometer_anti_alias_filter_bandwidth =
            LSM303DAccelerometerFilterBandwidth::Hz194;
    } else if frequency <= 800 {
        accel_mag_settings.accelerometer_data_rate = LSM303DAccelerometerUpdateRate::Hz800;
        accel_mag_settings.accelerometer_anti_alias_filter_bandwidth =
            LSM303DAccelerometerFilterBandwidth::Hz194;
    } else {
        accel_mag_settings.accelerometer_data_rate = LSM303DAccelerometerUpdateRate::Hz1600;
        accel_mag_settings.accelerometer_anti_alias_filter_bandwidth =
            LSM303DAccelerometerFilterBandwidth::Hz773;
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
        high_pass_filter_configuration: Some(LSM9DS0HighPassFilterCutOffConfig::HPCF_3),
    };

    if frequency <= 95 {
        gyro_settings.DR = LSM9DS0GyroscopeDataRate::Hz95;
    } else if frequency <= 190 {
        gyro_settings.DR = LSM9DS0GyroscopeDataRate::Hz190;
        gyro_settings.BW = LSM9DS0GyroscopeBandwidth::BW2;
    } else if frequency <= 380 {
        gyro_settings.DR = LSM9DS0GyroscopeDataRate::Hz380;
        gyro_settings.BW = LSM9DS0GyroscopeBandwidth::BW3;
    } else {
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
        magnetometer_sensitivity: LSM9DS0MagnetometerFS::gauss2,
    };

    if frequency <= 100 {
        accel_mag_settings.accelerometer_data_rate = LSM9DS0AccelerometerUpdateRate::Hz100;
        accel_mag_settings.accelerometer_anti_alias_filter_bandwidth =
            LSM9DS0AccelerometerFilterBandwidth::Hz50;
    } else if frequency <= 200 {
        accel_mag_settings.accelerometer_data_rate = LSM9DS0AccelerometerUpdateRate::Hz200;
        accel_mag_settings.accelerometer_anti_alias_filter_bandwidth =
            LSM9DS0AccelerometerFilterBandwidth::Hz50;
    } else if frequency <= 400 {
        accel_mag_settings.accelerometer_data_rate = LSM9DS0AccelerometerUpdateRate::Hz400;
        accel_mag_settings.accelerometer_anti_alias_filter_bandwidth =
            LSM9DS0AccelerometerFilterBandwidth::Hz194;
    } else if frequency <= 800 {
        accel_mag_settings.accelerometer_data_rate = LSM9DS0AccelerometerUpdateRate::Hz800;
        accel_mag_settings.accelerometer_anti_alias_filter_bandwidth =
            LSM9DS0AccelerometerFilterBandwidth::Hz194;
    } else {
        accel_mag_settings.accelerometer_data_rate = LSM9DS0AccelerometerUpdateRate::Hz1600;
        accel_mag_settings.accelerometer_anti_alias_filter_bandwidth =
            LSM9DS0AccelerometerFilterBandwidth::Hz773;
    }

    let (gyro, accel) = get_default_lsm9ds0_linux_i2c_devices().unwrap();

    match LSM9DS0::new(accel, gyro, gyro_settings, accel_mag_settings) {
        Ok(lsm9ds0) => lsm9ds0,
        Err(e) => {
            panic!("Couldn't initialize LSM9DS0");
        }
    }
}
