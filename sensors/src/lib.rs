#![allow(dead_code)]

extern crate i2cdev;
extern crate time;
extern crate byteorder;

mod constants;
use constants::*;

use std::thread;
use std::time::Duration;
use time::PreciseTime;

use i2cdev::core::*;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

use byteorder::{ByteOrder, LittleEndian};
use std::ops::{Add, Sub, Mul, Div};

use f32::consts::PI;
use std::f32;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};

const GYRO_ADDRESS: u16 = 0x6B;
const ACCELEROMETER_ADDRESS: u16 = 0x1d;

type DegreesPerSecond = f32;

#[derive(Copy, Clone, Debug)]
pub struct OrientationData<T : Add + Sub> {
    pub x: T,
    pub y: T,
    pub z: T
}

type AccelerometerData = OrientationData<i16>;
pub type GyroSensorData = OrientationData<DegreesPerSecond>;

impl Add for GyroSensorData {
    type Output = GyroSensorData;

    fn add(self, f: GyroSensorData) -> GyroSensorData {
        GyroSensorData {
            x: self.x + f.x,
            y: self.y + f.y,
            z: self.z + f.z,
        }
    }
}

impl Sub for GyroSensorData {
    type Output = GyroSensorData;

    fn sub(self, f: GyroSensorData) -> GyroSensorData {
        GyroSensorData {
            x: self.x - f.x,
            y: self.y - f.y,
            z: self.z - f.z,
        }
    }
}

impl Mul<f32> for GyroSensorData {
    type Output = GyroSensorData;

    fn mul(self, f: f32) -> GyroSensorData {
        GyroSensorData {
            x: self.x * f,
            y: self.y * f,
            z: self.z * f,
        }
    }
}


impl Div<f32> for GyroSensorData {
    type Output = GyroSensorData;

    fn div(self, f: f32) -> GyroSensorData {
        GyroSensorData {
            x: self.x / f,
            y: self.y / f,
            z: self.z / f,
        }
    }
}

fn init_gyro(mut device : &mut LinuxI2CDevice) {
    // init sequence
    device.smbus_write_byte_data(L3G_CTRL_REG1, 0b00001111).unwrap();
    // Set the dps to 2k.
    device.smbus_write_byte_data(L3G_CTRL_REG4, 0b00110000).unwrap();
    thread::sleep(Duration::from_millis(200));
}

fn sample_gyro(mut device : &mut LinuxI2CDevice) -> GyroSensorData {
    let results : Vec<u8> = device.smbus_read_i2c_block_data(0x80 | L3G_OUT_X_L, 6).unwrap();
    // This comes in the wrong order. WOW!
    let x = (((results[1] as i16) << 8)  | (results[0] as i16) ) as f32 * G_GAIN;
    let y = (((results[3] as i16) << 8) | (results[2] as i16)) as f32 * G_GAIN;
    let z = (((results[5] as i16) << 8) | (results[4] as i16)) as f32 * G_GAIN;
    GyroSensorData {x: x, y: y, z: z}
}

fn inite_accelerometer(mut device : &mut LinuxI2CDevice) {
    // init sequence
    //  z,y,x axis enabled , 100Hz data rate
    device.smbus_write_byte_data(LSM303_CTRL_REG1_A, 0b01010111).unwrap();
    // +/- 8G full scale: FS = 10 on DLHC, high resolution output mode
    device.smbus_write_byte_data(LSM303_CTRL_REG4_A, 0b00101000).unwrap();
    thread::sleep(Duration::from_millis(200));
}


fn sample_accelerometer(mut device : &mut LinuxI2CDevice) -> AccelerometerData {
    let buf : Vec<u8> = device.smbus_read_i2c_block_data(0x80 | LSM303_OUT_X_L_A, 6).unwrap();
    let x : i16 = LittleEndian::read_i16(&[buf[0], buf[1]]);
    let y : i16 = LittleEndian::read_i16(&[buf[2], buf[3]]);
    let z : i16 = LittleEndian::read_i16(&[buf[4], buf[5]]);
    AccelerometerData {x: x, y: y, z: z}
}

// Maybe go lower?
const offset_growth : f32 = 0.001;


pub fn start_sensors(sensor_poll_time: i64) -> Result<Receiver<GyroSensorData>, LinuxI2CError> {

    println!("Sensor poll time: {}", sensor_poll_time);
    let sample_time = time::Duration::milliseconds(sensor_poll_time);

    let (transmitter, receiver): (Sender<GyroSensorData>, Receiver<GyroSensorData>) = channel();
    match LinuxI2CDevice::new("/dev/i2c-1", GYRO_ADDRESS) {
        Ok(mut gyroscope) => {
            match LinuxI2CDevice::new("/dev/i2c-1", ACCELEROMETER_ADDRESS) {
                Ok(mut accelerometer) => {
                    init_gyro(&mut gyroscope);
                    inite_accelerometer(&mut accelerometer);
                    std::thread::spawn(move || {

                        // Assume we start on a relatively flat surface.
                        let mut sum = GyroSensorData { x: 0.0, y: 0.0, z: 0.0 };
                        let mut last_sample_time = PreciseTime::now();
                        // We add an offset to account for the gyro's tendency to randomly increase.
                        let mut gyro_offset = GyroSensorData { x: 0.0, y: 0.0, z: 0.0 };
                        loop {
                            let curr_time = PreciseTime::now();
                            let dt: f32 = last_sample_time.to(curr_time).num_microseconds().unwrap() as f32 / 1000000.0;
                            // Returns angular speed with respect to time. degrees/dt
                            let degrees_per_second = sample_gyro(&mut gyroscope) - gyro_offset;
                            let change_in_degrees = degrees_per_second * dt;

                            // compute changing offset very slowly over time as to not interfere with actual changes.
                            gyro_offset = gyro_offset * (1.0 - offset_growth) + change_in_degrees * offset_growth;
                            sum = sum + change_in_degrees;

                            let linear_acceleration = sample_accelerometer(&mut accelerometer);


                            let angle_acc_x = (linear_acceleration.x as f32).atan2(linear_acceleration.z as f32) * 180.0 / PI;
                            let angle_acc_y = (linear_acceleration.y as f32).atan2(linear_acceleration.z as f32) * 180.0 / PI;

                            // Comment this out to just try the gyro readings.
                            sum = sum  * 0.98 + GyroSensorData {x: angle_acc_x, y: angle_acc_y, z: sum.z} * 0.02;

                            transmitter.send(sum).unwrap();// Should handle error here in the future.

                            // Sleep until the sample time has passed +- 10 millis.
                            while last_sample_time.to(PreciseTime::now()) < sample_time {
                                thread::sleep(Duration::from_millis(1));
                            }
                            last_sample_time = curr_time;
                        }
                    });

                    return Ok(receiver);
                },
                Err(e) => {
                    println!("Failed to connect to accelerometer. {}", e);
                    Err(e)
                }
            }
        },
        Err(e) => {
            println!("Failed to connect to gyro. {}", e);
            Err(e)
        }
    }
}