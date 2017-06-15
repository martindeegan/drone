use rust_pigpio::*;
use rust_pigpio::pwm::*;

use std;
use std::thread;
use std::thread::sleep;
use std::thread::JoinHandle;

use std::collections::VecDeque;
use std::time::Duration;
use std::f32;
use std::sync::mpsc::{Receiver, TryRecvError};

const MAX_VALUE: u32 = 1400;
const MIN_VALUE: u32 = 1100;

use sensors::GyroSensorData;
use sensors::start_sensors;

use time;

use std::io::Write;

use connection::Peer;

use config::Config;

use debug_server;

pub fn terminate_all_motors() {
    println!("TERMINATING MOTORS!");

    for x in Config::new().motors {
        write(x, OFF);
    }

    terminate();
    sleep(Duration::from_secs(1));
}

pub struct MotorManager {
    pub motors: Vec<u32>,
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

    //PID STUFF

    pub fn start_pid_loop(&self, config: Config, peer: &mut Peer) {
        let debug_pipe = debug_server::init_debug_port();

        let ci = peer.subscribe_input();
        let controller_input = ci;

        let sensor_input: Receiver<GyroSensorData>;
        let sensor_poll_time = config.sensor_poll_time;
        match start_sensors(sensor_poll_time) {
            Ok(recv) => {
                sensor_input = recv;
            }
            Err(e) => {
                println!("Couldn't start sensors. Stopping. {:?}", e);
                return;
            }
        };

        let MOTOR_1 = self.motors[0];
        let MOTOR_2 = self.motors[1];
        let MOTOR_3 = self.motors[2];
        let MOTOR_4 = self.motors[3];

        //PID thread
        thread::spawn(move || {

            let mut desired_orientation = GyroSensorData {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };

            let mut integral = GyroSensorData {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            let mut last_proportional = GyroSensorData {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            let mut int_decay: GyroSensorData = GyroSensorData {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };

            let mut last_sample_time = time::PreciseTime::now();
            let start = time::PreciseTime::now();

            let mut last_n_samples: VecDeque<GyroSensorData> = VecDeque::new();

            writeln!(&mut std::io::stderr(), "time,power,p,i,d");

            let kp = config.kp;
            let ki = config.ki;
            let kd = config.kd;

            let integral_decay = 0.995;
            loop {
                match controller_input.try_recv() {
                    Ok(desired) => {
                        desired_orientation.x = desired.get_orientation().x;
                        desired_orientation.y = desired.get_orientation().y;

                        'inner: loop {
                            match controller_input.try_recv() {
                                Ok(desired) => {
                                    desired_orientation.x = desired.get_orientation().x;
                                    desired_orientation.y = desired.get_orientation().y;
                                },
                                Err(_) => {
                                    break 'inner;
                                }
                            }
                        }
                    },
                    Err(_) => { }
                }
                let mut current_orientation = sensor_input.recv().unwrap();
                loop {
                    match sensor_input.try_recv() {
                        Ok(orientation) => {
                            current_orientation = orientation;
                        }
                        Err(_) => {
                            break;
                        }
                    }
                }


                let t = time::PreciseTime::now();
                let dt: f32 = last_sample_time.to(t).num_microseconds().unwrap() as f32 / 1000000.0;
                let mut last_sample_time = t;

                //Safety check
                if current_orientation.x.abs() > config.motor_cutoff {
                    println!("Tilted too far. {:?}", current_orientation);
                    terminate_all_motors();
                    std::process::exit(0);
                }

                let proportional = (desired_orientation - current_orientation);

                last_n_samples.push_front(proportional * dt);
                integral = integral + proportional * dt;
                integral = integral * integral_decay;
                let derivative = (last_proportional - proportional) / dt;
                last_proportional = proportional;


                let range = 1.0;

                let u: GyroSensorData = proportional * kp + integral * ki + derivative * kd;
                let power = u * range;

                let debug_data = debug_server::DebugInfo {
                    time: start
                        .to(time::PreciseTime::now())
                        .num_microseconds()
                        .unwrap(),
                    power: power.x,
                    p: proportional.x * kp,
                    i: integral.x * ki,
                    d: derivative.x * kd,
                };

                debug_pipe.send(debug_data);

                let mid = 1200.0;
                let x_1 = mid - power.x;
                let x_2 = mid - power.x;
                let x_3 = mid + power.x;
                let x_4 = mid + power.x;

                if config.motors_on {
                    set_power(MOTOR_1, x_1 as u32);
                    set_power(MOTOR_2, x_2 as u32);
                    set_power(MOTOR_3, x_3 as u32);
                    set_power(MOTOR_4, x_4 as u32);
                }
            }
        });
    }
}

// Cleanflight:
//0.12029;
// 0.244381;
//0.000529;

const MAX_RANGE: f32 = 300.0;
const MIN_RANGE: f32 = 100.0;

const MAX_MID_ACCEL: f32 = 10.0;
const MAX_MIN_DECCEL: f32 = -10.0;

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
