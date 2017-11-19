use super::rust_pigpio::*;
use super::rust_pigpio::pwm::*;

use std;
use std::thread;
use std::thread::sleep;
use std::thread::JoinHandle;
use std::time::Duration;
use std::f32;
use std::io::stdin;
use std::io::Write;

use logger::{FlightLogger, ModuleLogger};
use configurations::Config;
use time;

use networking::p2p_connection::InputStream;
use config::Config;
use debug_server;

use super::pca9685::*;
use std::thread::sleep_ms;

use logger::ModuleLogger;
use configurations::Config;

// use sensor_manager::{InertialMeasurement,MultiSensorData};

const MAX_VALUE_F: f32 = 2000.0;
const MIN_VALUE_F: f32 = 1000.0;
const MAX_VALUE: u32 = 2000;
const MIN_VALUE: u32 = 1000;

pub struct MotorOutput {
    m1: f32,
    m2: f32,
    m3: f32,
    m4: f32,
}

pub train MotorManager {
    fn arm(&self);
    fn terminate(&self);
    fn set_powers(&self, input: MotorOutput);
}

// pub struct SoftwareMotorManager {
//     pub motors: Vec<u32>,
// };

// impl SoftwareMotor {
//     pub fn new(pin: u32) -> SoftwareMotor {
//         SoftwareMotor {
//             pin: pin,
//         }
//     }
// }

// impl MotorManager for SoftwareMotorManager {
//     pub fn set_powers(&self, input: MotorOutput) {
//         for &motor in self.motors {
//             pwm(motor, power).unwrap();
//         }
//     }
// }

pub struct SerialMotorManager {
    pub motors: Vec<u32>,
    device: PCA9685,
    logger: ModuleLogger,
};

impl SerialMotorManager {
    pub fn new() -> SerialMotor {
        let config = Config::new();
        let logger = ModuleLogger::new("Motors", Some("Check if your serial pwm controller is properly connected or change your configuration."));
        logger.log("Initializing Motor Manager.");
        let device = match LinuxI2CDevice::new("/dev/i2c-1", 0x40) {
            Ok(device) => device,
            Err(e) => logger.error()
	    let mut pca9685 = PCA9685::new(device, 50).unwrap();
        pca9685.set_all_duty_cycle(0);
        pca9685.set_pwm_frequency(100);
        sleep_ms(10);
        SerialMotor {
            motors: config.hardware.motors.pins,
            device: pca9685,
            logger: logger,
        }
    } 
}

impl MotorManager for SerialMotorManager {
    pub fn arm(&self) {
        self.logger.log("Arming Motors.");
        match self.device.set_all_pulse_length(MIN_VALUE_F) {
            Ok(()) => {},
            Err(e) => {
                self.logger.error(format!("{}", e));
                panic!(e.to_string());
            }
        }
        sleep_ms(2000);
        self.logger.success("Motors Armed.");
    }

    pub fn terminate(&self) {
        self.logger.log("Terminating Motors.");
        match self.device.set_all_duty_cycle(0) {
            Ok(()) => {},
            Err(e) => {
                self.logger.error(format!("{}", e));
                panic!(e.to_string());            
            }
        }
        sleep_ms(2000);
        self.logger.success("Motors Off.");
    }

    pub fn set_powers(&self, input: MotorOutput) {
        for i in 0..self.motors.len() {
            match self.set_pulse_length(self.motors[i], input[i]);
        }
    }
}

pub fn terminate_all_motors() {
    println!("[Motors]: TERMINATING MOTORS!");

    for x in Config::new().motors {
        write(x, OFF).unwrap();
    }

    terminate();
    sleep(Duration::from_secs(1));
}

pub struct MotorManager {
    logger: ModuleLogger,
    pub motors: Vec<u32>,
    motors_on: bool,
    serial: bool,
    pub last_m1: u32,
    pub last_m2: u32,
    pub last_m3: u32,
    pub last_m4: u32
}

impl MotorManager {
    pub fn new() -> MotorManager {
        let config = Config::new();
        let motor_pins = config.hardware.motors.pins;

        let mm = MotorManager { logger: ModuleLogger::new("Motors"), motors: config.motors.clone(), motors_on: config.motors_on,
                                last_m1: 0, last_m2: 0, last_m3: 0, last_m4: 0  };
        initialize().unwrap();
        for &motor in motors.clone() {
            initialize_motor(motor);
        }
        self.arm();
        println!("[Motors]: Initialized Motor Manager!");
        mm
    }

    fn initialize(&self) {
        
    }

    pub fn terminate(&mut self) {
        for motor in self.motors.clone() {
            stop(motor);
        }
        terminate();
        println!("[Motors]: Stopped.");
    }

    fn arm(&self) {
        if self.motors_on {
            println!("[Motors]: Arming motors.");

            let mut handles: Vec<JoinHandle<()>> = Vec::new();

            for motor in self.motors.clone() {
                handles.push(arm(motor));
            }

            for handle in handles {
                handle.join().unwrap();
            }

            println!("[Motors]: Motors armed.");
            for motor in self.motors.clone() {
                set_power(motor, MIN_VALUE);
            }
        }
    }

    pub fn new_motor(&mut self, gpio_pin: u32) {
        initialize_motor(gpio_pin);
        self.motors.push(gpio_pin);
    }

    pub fn set_powers(&mut self, mut m1: f32, mut m2: f32, mut m3: f32, mut m4: f32) {

        let max = m1.max(m2).max(m3).max(m4);
        let min = m1.min(m2).min(m3).min(m4);
        if max > MAX_VALUE_F {
            let diff = max - MAX_VALUE_F;
            m1 -= diff; m2 -= diff; m3 -= diff; m4 -= diff;
        } else if min < MIN_VALUE_F {
            let diff = min - MIN_VALUE_F;
            m1 -= diff; m2 -= diff; m3 -= diff; m4 -= diff;
        }

        self.last_m1 = m1 as u32;
        self.last_m2 = m2 as u32;
        self.last_m3 = m3 as u32;
        self.last_m4 = m4 as u32;

        println!("{},{},{},{}", self.last_m1, self.last_m2, self.last_m3, self.last_m4);

        if self.motors_on {
            set_power(self.motors[0], self.last_m1);
            set_power(self.motors[1], self.last_m2);
            set_power(self.motors[2], self.last_m3);
            set_power(self.motors[3], self.last_m4);
        }
        else {
            // println!("m1: {}, m2: {}, m3: {}, m4: {}", m1 as u32, m2 as u32, m3 as u32, m4 as u32);
        }
    }
}

impl std::ops::Drop for MotorManager {
    fn drop(&mut self) {
        self.terminate();
    }
}


fn initialize_motor(gpio_pin: u32) -> u32 {
    let config = Config::new();
    let range: u32 = 1000000 / config.motor_frequency;
    set_mode(gpio_pin, OUTPUT).unwrap();
    set_pwm_range(gpio_pin, range).unwrap();
    set_pwm_frequency(gpio_pin, config.motor_frequency).unwrap();
    gpio_pin
}

pub fn calibrate() {
    initialize().unwrap();

    for x in Config::new().motors {
        initialize_motor(x);
    }

    sleep(Duration::from_secs(2));

    let config = Config::new();
    println!("[Motors]: Calibrating");
    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    for motor in config.motors.clone() {
//        pwm(motor, 0).unwrap();
    }
    println!("{}", Yellow.paint("[Motors]: Raspberry Pi must be connected to an external power source. Unplug battery. Then press enter."));
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error");

    for motor in config.motors.clone() {
       pwm(motor, 2000).unwrap();
    }

    println!("{}", Green.paint("[Motors]: Plug in the battery now. Then press enter."));
    input = String::new();
    stdin().read_line(&mut input).expect("Error");

    println!("{}", Yellow.paint("[Motors]: Wait until the rising tones finish. Then press enter."));
    input = String::new();
    stdin().read_line(&mut input).expect("Error");
    for motor in config.motors.clone() {
       pwm(motor, 1000).unwrap();
    }

    sleep(Duration::from_secs(4));

    for motor in config.motors.clone() {
        write(motor, OFF).unwrap();
    }

    println!("{}", Cyan.paint("[Motors]: Finished calibrating. You can now reconnect the Pi to the battery."));
    sleep(Duration::from_secs(3));
    println!("[Motors]: Shutting down");

    for motor in config.motors.clone() {
        stop(motor);
    }

    terminate();
    thread::sleep(Duration::from_secs(2));
}

fn arm(motor: u32) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        pwm(motor, 1250).unwrap();
        sleep(Duration::from_secs(1));
        pwm(motor, MIN_VALUE).unwrap();
        sleep(Duration::from_secs(2));
    })
}

fn set_power(motor: u32, mut power: u32) {
    pwm(motor, power).unwrap();
}

fn stop(motor: u32) {
    write(motor, OFF).unwrap();
}
