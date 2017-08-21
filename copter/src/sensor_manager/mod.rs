extern crate i2csensors;
//extern crate i2cdev_bmp180;
extern crate i2cdev_bmp280;
//extern crate i2cdev_l3gd20;
//extern crate i2cdev_lsm303d;
extern crate i2cdev_lsm9ds0;
extern crate i2cdev;

use config::Config;
use config::SensorCalibrations;

use std::io::stdin;
use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use std::marker::Sync;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use std::error::Error;
use std::time::Duration as _Duration;
use time::{PreciseTime, Duration};

use self::i2csensors::{Barometer, Thermometer, Gyroscope, Accelerometer, Magnetometer, Vec3};
use self::i2cdev::linux::LinuxI2CError;
mod sensors;
use self::sensors::*;

const LOCATION_SAMPLING_RATE_MS: f32 = 50.0;

const RADIAN_DEGREES: f32 = 180.0 / 3.14159265;

pub type MultiSensorData = Vec3;

#[derive(Copy,Clone)]
pub struct InertialMeasurement {
    pub angles: MultiSensorData,
    pub rotation_rate: MultiSensorData,
    pub altitude: f32
}

#[derive(Copy,Clone)]
pub struct Location {
    pub latitude: f32,
    pub longitude: f32,
    pub altitude: f32
}

//pub struct SensorManager {
//
//}
//
//impl SensorManager {
//    pub fn new() -> SensorManager {
//        SensorManager{}
//    }
//
//    pub fn start_sensor_manager() -> (Receiver<MultiSensorData>, Receiver<MultiSensorData>, Receiver<MultiSensorData>) {
//        let config = Config::new();
//        let sensor_poll_rate = config.sensor_poll_rate;
//        let sensor_poll_delay = (1000000.0 / (sensor_poll_rate as f32)) as u32;
//
//        let (gyroscope_transmitter, rx_g): (Sender<MultiSensorData>, Receiver<MultiSensorData>) = channel();
//        let (accelerometer_transmitter, rx_a): (Sender<MultiSensorData>, Receiver<MultiSensorData>) = channel();
//        let (magnetometer_transmitter, rx_m): (Sender<MultiSensorData>, Receiver<MultiSensorData>) = channel();
//
//        thread::Builder::new().name("Sensor Manager Loop".to_string()).spawn(move || {
//            let (mut barometer, mut thermometer, mut gyroscope, mut accelerometer, mut magnetometer) = get_sensors(sensor_poll_rate / 4);
//
//        });
//
//        (rx_g, rx_a, rx_m)
//}

fn get_sensors(poll_rate: i64) -> (Rc<RefCell<Barometer<Error = LinuxI2CError>>>,
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

    for sensor in config.sensors {
        match sensor.as_ref() {
            "BMP180" => {

            },
            "BMP280" => {
                let bmp280 = Rc::new(RefCell::new(get_bmp280(poll_rate)));
                barometer = Some(bmp280.clone());
                thermometer = Some(bmp280.clone());
            },
            "L3GD20" => {

            },
            "LSM303D" => {

            },
            "LSM9DS0" => {
                let lsm9ds0 = Rc::new(RefCell::new(get_lsm9ds0(poll_rate)));
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

//Returns Orientation Receiver and a Location Receiver
pub fn start_sensors() -> (Receiver<InertialMeasurement>, Receiver<Location>) {
    //Eventually GPS

    let config = Config::new();
    let sensor_poll_rate = config.sensor_sample_frequency;
    let sensor_poll_delay = (1000000000 / sensor_poll_rate) as u32;
    let loop_duration = _Duration::new(0, sensor_poll_delay);

    let (orientation_transimitter, orientation_receiver): (Sender<InertialMeasurement>, Receiver<InertialMeasurement>) = channel();
    let (location_transmitter, location_receiver): (Sender<Location>, Receiver<Location>) = channel();

    thread::Builder::new().name("Sensor Loop".to_string()).spawn(move || {
        let (mut barometer, mut thermometer, mut gyroscope, mut accelerometer, mut magnetometer) = get_sensors(sensor_poll_rate as i64 / 4);
        //MultiSensorData { x: 1.7352998, y: 0.38937503, z: -9.603826 }
        let calibs = SensorCalibrations::new();
        let (gyro_calib, accel_calib) = (Vec3 { x: calibs.gyro_x, y: calibs.gyro_y, z: calibs.gyro_z }, Vec3 { x: calibs.accel_x, y: calibs.accel_y, z: calibs.accel_z });

        let mut last_time = PreciseTime::now();
        let mut current_euler_angles = MultiSensorData::zeros();
        let mut bearing_calib = 0.0;
        let mut bearing = 0.0;
        let mut count = 0;

        for i in 0..10 {
            match magnetometer.borrow_mut().magnetic_reading() {
                Ok(magnetometer_output) => {
                    if magnetometer_output.y > 0.0 {
                        bearing_calib += 90.0 - (magnetometer_output.x / magnetometer_output.y).atan() * RADIAN_DEGREES;
                    } else if magnetometer_output.y < 0.0 {
                        bearing_calib += 270.0 - (magnetometer_output.x / magnetometer_output.y).atan() * RADIAN_DEGREES;
                    } else if magnetometer_output.x > 0.0 {
                        bearing_calib += 180.0
                    }
                },
                Err(e) => {}
            }
            thread::sleep(_Duration::from_millis(50));
        }

        bearing_calib /= 10.0;

        let alpha = 0.02;

        loop {
            let current_time = PreciseTime::now();
            let dt = last_time.to(current_time).num_microseconds().unwrap() as f32 / 1000000.0;

            //Integrate gyroscope output
            let gyroscope_output = gyroscope.borrow_mut().angular_rate_reading().unwrap() - gyro_calib;
            current_euler_angles = current_euler_angles + gyroscope_output * dt;

            //Compute angles from accelerometer
            let mut accelerometer_output = accelerometer.borrow_mut().acceleration_reading().unwrap() - accel_calib;
            let angle_acc_x = accelerometer_output.y.atan2(accelerometer_output.z) * RADIAN_DEGREES;
            let angle_acc_y = accelerometer_output.x.atan2(accelerometer_output.z) * RADIAN_DEGREES * -1.0;
            let accelerometer_angles = MultiSensorData {
                x: angle_acc_x,
                y: angle_acc_y,
                z: 0.0
            };

            //Complementary Filter
            current_euler_angles = current_euler_angles * (1.0 - alpha) + accelerometer_angles * alpha;

            //Compute bearing.
            if count % 4 == 0 {
                match magnetometer.borrow_mut().magnetic_reading() {
                    Ok(magnetometer_output) => {
                    if magnetometer_output.y > 0.0 {
                    bearing = 90.0 - (magnetometer_output.x / magnetometer_output.y).atan() * RADIAN_DEGREES;
                    } else if magnetometer_output.y < 0.0 {
                    bearing = 0.0 - (magnetometer_output.x / magnetometer_output.y).atan() * RADIAN_DEGREES;
                    } else if magnetometer_output.x > 0.0 {
                    bearing = 180.0
                    }
                    },
                    Err(e) => {}
                }
            }
            count += 1;
//            bearing -= bearing_calib;

            current_euler_angles.z = bearing;
//            if current_euler_angles.z > 360.0 {
//                current_euler_angles.z -= 360.0;
//            }
//            else if current_euler_angles.z < 0.0 {
//                current_euler_angles.z += 360.0;
//            }

            match orientation_transimitter.send(InertialMeasurement {
                angles: current_euler_angles,
                rotation_rate: gyroscope_output,
                altitude: 0.0
            }) {
                Ok(o) => {},
                Err(e) => {
                    return;
                }
            }

            last_time = current_time;
            thread::sleep(loop_duration);
        }
    });

    thread::spawn(move || {
        loop {

        }
    });


    (orientation_receiver, location_receiver)
}

pub fn calibrate_sensors() {
    println!("Calibrating.");
    let (mut barometer, mut thermometer, mut gyroscope, mut accelerometer, mut magnetometer) = get_sensors(400);

    let mut acceleration_calibration = Vec3::zeros();
    let mut gyroscope_calibration = Vec3::zeros();
    for i in 0..100 {
        acceleration_calibration = acceleration_calibration + accelerometer.borrow_mut().acceleration_reading().unwrap();
        gyroscope_calibration = gyroscope_calibration + gyroscope.borrow_mut().angular_rate_reading().unwrap();
        thread::sleep(_Duration::from_millis(50));
    }

    println!("Rotate 180 degrees then press enter.");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error");

    for i in 0..100 {
        acceleration_calibration = acceleration_calibration + accelerometer.borrow_mut().acceleration_reading().unwrap();
        gyroscope_calibration = gyroscope_calibration + gyroscope.borrow_mut().angular_rate_reading().unwrap();
        thread::sleep(_Duration::from_millis(50));
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