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

use std::io;
use std::io::Write;



const MAX_ERROR: f32 = 200.0;
const MAX_PID_POWER: f32 = 1400.0;
const MIN_PID_POWER: f32 = 1200.0;

static mut ki_mut: f32 = 0.003;

pub fn TERMINATE_ALL_MOTORS() {
    println!("TERMINATING MOTORS!");

    write(19, OFF);
    write(20, OFF);
    write(21, OFF);
    write(26, OFF);

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
        let mut sensor_input: Receiver<GyroSensorData>;
        let mut controller_input = connection::get_peer() {

        }
        match start_sensors() {
            Ok(recv) => { sensor_input = recv; },
            Err(e) => {
                println!("Couldn't start sensors. Stopping. {:?}", e);
                return;
            }
        };
        //let mut controller_input = peer.subscribe();

        let MOTOR_1 = self.motors[0];
        let MOTOR_2 = self.motors[1];
        let MOTOR_3 = self.motors[2];
        let MOTOR_4 = self.motors[3];

        //PID thread
        thread::spawn(move || {
            let mut desired_orientation = GyroSensorData { x: 0.0, y: 0.0, z: 0.0 };


            let mut last_sample_time = Instant::now();

            let mut integral: f32 = GyroSensorData { x: 0.0, y: 0.0, z: 0.0 };
            let mut last_proportional = GyroSensorData { x: 0.0, y: 0.0, z: 0.0 };

            loop {
                let dt: f32 = Instant::now().duration_since(last_sample_time).subsec_nanos() as f32 / 1000000000.0;
                let mut last_sample_time = Instant::now();


                //                match controller_input.try_recv() {
                //                    Ok(orientation) => { desired_orientation = orientation; },
                //                    TryRecvError::Empty => { },
                //                    TryRecvError::Disconnected => {
                //                        println!("Lost connection with controller! Defaulting to level orientation.");
                //                        desired_orientation = GyroSensorData{ x: 0.0, y: 0.0, z: 0.0};
                //                    }
                //                }

                let mut current_orientation = GyroSensorData { x: 0.0, y: 0.0, z: 0.0 };
                match sensor_input.recv() {
                    Ok(orientation) => { current_orientation = orientation; },
                    Err(_) => {},
                    //                    Err(TryRecvError::Disconnected) => {
                    //                        println!("Lost connection with sensors! We're fucked!");
                    //                        sensor_input = start_sensors().unwrap();
                    //                    }
                }


                //Safety check
                if current_orientation.x > 30.0 || current_orientation.x < -30.0
                    || current_orientation.y > 30.0 || current_orientation.y < -30.0 {
                    TERMINATE_ALL_MOTORS();
                    std::process::exit(0);
                }

                let proportional = (desired_orientation - current_orientation) / 45.0;

                integral = (integral + prop * dt);
                integral = integral * 0.998;

                let derivative = (proportional - last_proportional) / dt;
                last_proportional = proportional;

                //                println!("integral {}", integral);

                let (m1, m2, m3, m4) = calculate_corrections(proportional, integral, derivative);

                set_power(MOTOR_1, m1);
                set_power(MOTOR_2, m2);
                set_power(MOTOR_3, m3);
                set_power(MOTOR_4, m4);

                println!("a: {}", format!("{:.2}", current_orientation.x));
            }
        });
    }
}

//                   //Cleanflight:

const MAX_RANGE: f32 = 300.0;
const MIN_RANGE: f32 = 100.0;

const MAX_MID_ACCEL: f32 = 10;
const MAX_MIN_DECCEL: f32 = -10;

const KP: f32 = 0.2; //0.12029;
const KI: f32 = 0.0; //0.244381;
const KD: f32 = 0.0; //0.000529;

fn calculate_corrections(prop: GyroSensorData, int: GyroSensorData, der: GyroSensorData) -> (u32, u32, u32, u32) {
    let mid = 1250.0;
    let range = 150.0;

    let u: GyroSensorData = prop * KP + int * KI + der * KD;
    let power = u * range;

    let x_1 = mid - power.x;
    let x_2 = mid - power.x;
    let x_3 = mid + power.x;
    let x_4 = mid + power.x;

    let y_2 = mid - power.y;
    let y_3 = mid - power.y;
    let y_1 = mid + power.y;
    let y_4 = mid + power.y;

    ((x_1 + y_1) / 2,
     (x_2 + y_2) / 2,
     (x_3 + y_3) / 2,
     (x_4 + y_4) / 2)
}

fn calculate_error(current: GyroSensorData, desired: GyroSensorData) -> f32 {
    let diff_x = desired.x - current.x;
    let diff_y = desired.y - current.y;

    (diff_x * diff_x + diff_y - diff_y).sqrt()
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


