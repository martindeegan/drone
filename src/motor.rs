#![cfg(rpi)] //Add to .bashrc: export RUST_PI_COMPILATION="rpi"

use rust_pigpio::*;
use rust_pigpio::pwm::*;

use std;
use std::thread;
use std::thread::sleep;
use std::thread::JoinHandle;
use std::time::Duration;

const MAX_VALUE : u32 = 1975;
const MIN_VALUE : u32 = 1100;

use std::f32;
use std::sync::mpsc::{Receiver, TryRecvError};
use std::error::Error;

use sensors::GyroSensorData;
use sensors::start_sensors;

use std::time::Instant;

const KP: f32 = 0.032029;
const KI: f32 = 0.244381;
const KD: f32 = 0.000529;

pub fn TERMINATE_ALL_MOTORS() {
    for motor in 2..28 {
        match write(motor, OFF) {
            Ok(out) => {},
            Err(e) => {continue;},
        }
    }
    terminate();
}

pub struct MotorManager {
    pub motors: Vec<u32>
}

impl MotorManager {
    pub fn new() -> MotorManager {
        let mm = MotorManager { motors: Vec::new() };
        mm.initialize();
        mm
    }

    fn initialize(&self) {
        initialize().unwrap();
        println!("Initialized Motor Manager!");
    }

    pub fn terminate(&mut self) {
        for motor in self.motors.clone() {
            stop(motor);
        }
        terminate();
        println!("Stopped.");
    }

    pub fn arm(&self) {
        println!("Arming motors.");

        let mut handles: Vec<JoinHandle<()>> = Vec::new();

        for motor in self.motors.clone() {
            handles.push(arm(motor));
        }

        for mut handle in handles {
            handle.join();
        }

        println!("Motors armed.");

        println!("Starting motors.");

        for motor in self.motors.clone() {
            set_power(motor, MIN_VALUE);
        }
    }

    pub fn new_motor(&mut self, gpio_pin: u32) {
        initialize_motor(gpio_pin);
        self.motors.push(gpio_pin);
    }

    pub fn set_power(&self, motor_num: u32, power: u32) {
        set_power(motor_num, power);
    }

    //PID STUFF

    pub fn start_pid_loop(&self) {
        let mut sensor_input = start_sensors().unwrap();
        //let mut controller_input = peer.subscribe();

        let MOTOR_1 = self.motors[0];
        let MOTOR_2 = self.motors[1];
        let MOTOR_3 = self.motors[2];
        let MOTOR_4 = self.motors[3];
        
        //PID thread
        thread::spawn(move || {
            let mut desired_orientation = GyroSensorData { x: 0.0, y: 0.0, z: 0.0 };
            let mut current_orientation = GyroSensorData { x: 0.0, y: 0.0, z: 0.0 };

            let mut last_sample_time = Instant::now();

            let mut last_error = 0.0;
            let mut running_error = 0.0;

            loop {
                let dt: f32 = Instant::now().duration_since(last_sample_time).subsec_nanos() as f32 / 1000000000.0;
                let mut last_sample_time = Instant::now();

                //Safety check
                if current_orientation.x > 30.0 || current_orientation.x < -30.0
                    || current_orientation.y > 30.0 || current_orientation.y < -30.0 {
                    TERMINATE_ALL_MOTORS();
                }

                //                match controller_input.try_recv() {
                //                    Ok(orientation) => { desired_orientation = orientation; },
                //                    TryRecvError::Empty => { },
                //                    TryRecvError::Disconnected => {
                //                        println!("Lost connection with controller! Defaulting to level orientation.");
                //                        desired_orientation = GyroSensorData{ x: 0.0, y: 0.0, z: 0.0};
                //                    }
                //                }

                match sensor_input.try_recv() {
                    Ok(orientation) => { current_orientation = orientation; },
                    Err(TryRecvError::Empty) => {},
                    Err(TryRecvError::Disconnected) => {
                        println!("Lost connection with sensors! We're fucked!");
                        sensor_input = start_sensors().unwrap();
                    }
                }

                let error = calculate_error(current_orientation, desired_orientation);

                let proportional = KP * error;
                let integral = KI * error * dt;
                let derivative = (KD * error) / dt;
            }
        });
    }
}

fn calculate_error(current: GyroSensorData, desired: GyroSensorData) -> f32 {
    0.0
}

impl std::ops::Drop for MotorManager {
    fn drop(&mut self) {
        self.terminate();
    }
}

/* -------------------------- INDIVIDUAL MOTORS -------------------------------------*/

fn initialize_motor(gpio_pin: u32) -> u32 {
    set_mode(gpio_pin, OUTPUT).unwrap();
    set_pwm_range(gpio_pin, 2000).unwrap();
    set_pwm_frequency(gpio_pin, 500).unwrap();
    gpio_pin
}

//pub fn calibrate(motor: MotorHandle) -> thread::JoinHandle<()> {
//    thread::spawn(move || {
//        servo(motor, 0).unwrap();
//        sleep(Duration::from_secs(4));
//        servo(motor, 2000).unwrap();
//        sleep(Duration::from_secs(4));
//        servo(motor, 1000).unwrap();
//        sleep(Duration::from_secs(8));
//        write(motor, OFF).unwrap();
//        sleep(Duration::from_secs(8));
//    })
//}

fn arm(motor: u32) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        pwm(motor, 1000).unwrap();
        sleep(Duration::from_secs(2));

        pwm(motor, 1100).unwrap();
    })
}

fn set_power(motor: u32, mut power: u32) {
    if power > MAX_VALUE {
        power = MAX_VALUE;
    } else if power < MIN_VALUE {
        power = MIN_VALUE;
    }

    pwm(motor, power).unwrap();
}

fn stop(motor: u32) {
    write(motor, OFF).unwrap();
}


