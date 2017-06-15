#![allow(dead_code)]

extern crate i2cdev;

extern crate time;
extern crate byteorder;

use std::thread;
use std::time::Duration;
use time::PreciseTime;

use i2cdev::core::*;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

use byteorder::{ByteOrder, LittleEndian};
use std::ops::{Add, Sub, Mul, Div};

const L3G_CTRL_REG1 : u8 = 0x20;
const L3G_CTRL_REG2 : u8 = 0x21;
const L3G_CTRL_REG3 : u8 = 0x22;
const L3G_CTRL_REG4 : u8 =  0x23;
const L3G_CTRL_REG5 : u8 =  0x24;
const L3G_REFERENCE : u8 =   0x25;
const L3G_OUTd_TEMP  : u8 =  0x26;
const L3G_STATUS_REG : u8 =  0x27;

const L3G_OUT_X_L   : u8    =  0x28;
const L3G_OUT_X_H   : u8    =  0x29;
const L3G_OUT_Y_L  : u8     =  0x2A;
const L3G_OUT_Y_H   : u8    =  0x2B;
const L3G_OUT_Z_L    : u8   =  0x2C;
const L3G_OUT_Z_H    : u8   =  0x2D;

const G_GAIN : f32 = 0.070;

const GYRO_ADDRESS: u16 = 0x6B;

// Accelerometer
const ACCELEROMETER_ADDRESS: u16 = 0x1d;

// register addresses
const MAG_ADDRESS          : u8 =  (0x3C >> 1);
const ACC_ADDRESS            : u8 = (0x32 >> 1);
const ACC_ADDRESS_SA0_A_LOW  : u8 = (0x30 >> 1);
const ACC_ADDRESS_SA0_A_HIGH : u8 = (0x32 >> 1);

const LSM303_CTRL_REG1_A   : u8   = 0x20;
const LSM303_CTRL_REG2_A   : u8   = 0x21;
const LSM303_CTRL_REG3_A   : u8   = 0x22;
const LSM303_CTRL_REG4_A   : u8   = 0x23;
const LSM303_CTRL_REG5_A   : u8   = 0x24;
const LSM303_CTRL_REG6_A   : u8   = 0x25; // DLHC only
const LSM303_HP_FILTER_RESET_A : u8= 0x25; // DLH, DLM only
const LSM303_REFERENCE_A   : u8   = 0x26;
const LSM303_STATUS_REG_A  : u8   = 0x27;

const LSM303_OUT_X_L_A     : u8   = 0x28;
const LSM303_OUT_X_H_A     : u8   = 0x29;
const LSM303_OUT_Y_L_A     : u8   = 0x2A;
const LSM303_OUT_Y_H_A     : u8   = 0x2B;
const LSM303_OUT_Z_L_A     : u8   = 0x2C;
const LSM303_OUT_Z_H_A     : u8   = 0x2D;

const LSM303_FIFO_CTRL_REG_A : u8  = 0x2E; // DLHC only
const LSM303_FIFO_SRC_REG_A: u8   = 0x2F; // DLHC only

const LSM303_INT1_CFG_A    : u8   = 0x30;
const LSM303_INT1_SRC_A    : u8   = 0x31;
const LSM303_INT1_THS_A    : u8   = 0x32;
const LSM303_INT1_DURATION_A : u8  = 0x33;
const LSM303_INT2_CFG_A    : u8   = 0x34;
const LSM303_INT2_SRC_A    : u8   = 0x35;
const LSM303_INT2_THS_A    : u8   = 0x36;
const LSM303_INT2_DURATION_A : u8  = 0x37;

const LSM303_CLICK_CFG_A   : u8   = 0x38; // DLHC only
const LSM303_CLICK_SRC_A   : u8   = 0x39; // DLHC only
const LSM303_CLICK_THS_A   : u8   = 0x3A; // DLHC only
const LSM303_TIME_LIMIT_A  : u8   = 0x3B; // DLHC only
const LSM303_TIME_LATENCY_A: u8   = 0x3C; // DLHC only
const LSM303_TIME_WINDOW_A : u8   = 0x3D; // DLHC only

const LSM303_CRA_REG_M     : u8   = 0x00;
const LSM303_CRB_REG_M     : u8   = 0x01;
const LSM303_MR_REG_M      : u8   = 0x02;

const LSM303_OUT_X_H_M     : u8   = 0x03;
const LSM303_OUT_X_L_M     : u8   = 0x04;
//const LSM303_OUT_Y_H_M   : u8   =   -1;   // The addresses of the Y and Z magnetometer output registers
//const LSM303_OUT_Y_L_M    : u8   =  -2 ;  // are reversed on the DLM and DLHC relative to the DLH.
//const LSM303_OUT_Z_H_M    : u8   =  -3 ;  // These four defines have dummy values so the library can
//const LSM303_OUT_Z_L_M    : u8   =  -4 ;  // determine the correct address based on the device type.

const LSM303_SR_REG_M      : u8   = 0x09;
const LSM303_IRA_REG_M     : u8   = 0x0A;
const LSM303_IRB_REG_M     : u8   = 0x0B;
const LSM303_IRC_REG_M     : u8   = 0x0C;

const LSM303_WHO_AM_I_M    : u8   = 0x0F; // DLM only

const LSM303_TEMP_OUT_H_M  : u8   = 0x31; // DLHC only
const LSM303_TEMP_OUT_L_M  : u8   = 0x32; // DLHC only
const LSM303DLH_OUT_Y_H_M  : u8   = 0x05;
const LSM303DLH_OUT_Y_L_M  : u8   = 0x06;
const LSM303DLH_OUT_Z_H_M  : u8   = 0x07;
const LSM303DLH_OUT_Z_L_M  : u8   = 0x08;

const LSM303DLM_OUT_Z_H_M  : u8   = 0x05;
const LSM303DLM_OUT_Z_L_M  : u8   = 0x06;
const LSM303DLM_OUT_Y_H_M  : u8   = 0x07;
const LSM303DLM_OUT_Y_L_M  : u8   = 0x08;

const LSM303DLHC_OUT_Z_H_M : u8   = 0x05;
const LSM303DLHC_OUT_Z_L_M : u8   = 0x06;

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

fn initGyro(mut device : &mut LinuxI2CDevice) {
    // init sequence
    device.smbus_write_byte_data(L3G_CTRL_REG1, 0b00001111).unwrap();
    // Set the dps to 2k.
    device.smbus_write_byte_data(L3G_CTRL_REG4, 0b00110000).unwrap();
    thread::sleep(Duration::from_millis(200));
}

fn sampleGyro(mut device : &mut LinuxI2CDevice) -> GyroSensorData {
    let results : Vec<u8> = device.smbus_read_i2c_block_data(0x80 | L3G_OUT_X_L, 6).unwrap();
    // This comes in the wrong order. WOW!
    let x = (((results[1] as i16) << 8)  | (results[0] as i16) ) as f32 * G_GAIN;
    let y = (((results[3] as i16) << 8) | (results[2] as i16)) as f32 * G_GAIN;
    let z = (((results[5] as i16) << 8) | (results[4] as i16)) as f32 * G_GAIN;
    GyroSensorData {x: x, y: y, z: z}
}

fn initAccelerometer(mut device : &mut LinuxI2CDevice) {
    // init sequence
    //  z,y,x axis enabled , 100Hz data rate
    device.smbus_write_byte_data(LSM303_CTRL_REG1_A, 0b01010111).unwrap();
    // +/- 8G full scale: FS = 10 on DLHC, high resolution output mode
    device.smbus_write_byte_data(LSM303_CTRL_REG4_A, 0b00101000).unwrap();
    thread::sleep(Duration::from_millis(200));
}


fn sampleAccelerometer(mut device : &mut LinuxI2CDevice) -> AccelerometerData {
    let buf : Vec<u8> = device.smbus_read_i2c_block_data(0x80 | LSM303_OUT_X_L_A, 6).unwrap();
    let x : i16 = LittleEndian::read_i16(&[buf[0], buf[1]]);
    let y : i16 = LittleEndian::read_i16(&[buf[2], buf[3]]);
    let z : i16 = LittleEndian::read_i16(&[buf[4], buf[5]]);
    AccelerometerData {x: x, y: y, z: z}
}


use f32::consts::PI;
use std::f32;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};
use std::error::Error;

// Maybe go lower?
const offset_growth : f32 = 0.001;

const KP: f32 = 0.032029;
const KI: f32 = 0.244381;
const KD: f32 = 0.000529;

pub fn STOP_SENSORS() {

}

pub fn start_sensors(sensor_poll_time: i64) -> Result<Receiver<GyroSensorData>, LinuxI2CError> {

    println!("Sensor poll time: {}", sensor_poll_time);
    let sample_time = time::Duration::milliseconds(sensor_poll_time);

    let (transmitter, receiver): (Sender<GyroSensorData>, Receiver<GyroSensorData>) = channel();
    match LinuxI2CDevice::new("/dev/i2c-1", GYRO_ADDRESS) {
        Ok(mut gyroscope) => {
            match LinuxI2CDevice::new("/dev/i2c-1", ACCELEROMETER_ADDRESS) {
                Ok(mut accelerometer) => {
                    initGyro(&mut gyroscope);
                    initAccelerometer(&mut accelerometer);
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
                            let degrees_per_second = sampleGyro(&mut gyroscope) - gyro_offset;
                            let change_in_degrees = degrees_per_second * dt;

                            // compute changing offset very slowly over time as to not interfere with actual changes.
                            gyro_offset = gyro_offset * (1.0 - offset_growth) + change_in_degrees * offset_growth;
                            sum = sum + change_in_degrees;

                            let linear_acceleration = sampleAccelerometer(&mut accelerometer);


                            let angle_acc_x = (linear_acceleration.x as f32).atan2(linear_acceleration.z as f32) * 180.0 / PI;
                            let angle_acc_y = (linear_acceleration.y as f32).atan2(linear_acceleration.z as f32) * 180.0 / PI;

                            // Comment this out to just try the gyro readings.
                            sum = sum  * 0.98 + GyroSensorData {x: angle_acc_x, y: angle_acc_y, z: sum.z} * 0.02;

                            transmitter.send(sum).unwrap();// Should handle error here in the future.

                            // Sleep until the sample time has passed +- 10 millis.
                            while (last_sample_time.to(PreciseTime::now()) < sample_time) {
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