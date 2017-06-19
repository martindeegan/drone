use rust_pigpio::*;
use rust_pigpio::pwm::*;

use std;
use std::thread;
use std::thread::sleep;
use std::thread::JoinHandle;

use std::time::Duration;
use std::f32;
use std::sync::mpsc::{Sender, Receiver};

const MAX_VALUE: u32 = 1600;
const MIN_VALUE: u32 = 1100;

use sensors::GyroSensorData;
use sensors::start_sensors;
use sensors::SensorOutput;

use time;

use connection::Peer;

use config::Config;

use debug_server;

pub struct MotorManager {
    pub motors: Vec<u32>,
}

pub fn terminate_all_motors(debug_pipe : Sender<debug_server::Signal>) {
    println!("[Motors]: TERMINATING MOTORS!");

    debug_pipe.send(debug_server::Signal::Stop).unwrap();

    for x in Config::new().motors {
        write(x, OFF).unwrap();
    }

    terminate();
    sleep(Duration::from_secs(1));
}

impl MotorManager {
    pub fn new() -> MotorManager {
        let mm = MotorManager { motors: Vec::new() };
        mm.initialize();
        mm
    }

    fn initialize(&self) {
        initialize().unwrap();
        println!("[Motors]: Initialized Motor Manager!");
    }

    pub fn terminate(&mut self) {
        for motor in self.motors.clone() {
            stop(motor);
        }
        terminate();
        println!("[Motors]: Stopped.");
    }



    pub fn arm(&self) {
        println!("[Motors]: Arming motors.");

        let mut handles: Vec<JoinHandle<()>> = Vec::new();

        for motor in self.motors.clone() {
            handles.push(arm(motor));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("[Motors]: Motors armed.");

        println!("[Motors]: Starting motors.");

    }

    pub fn new_motor(&mut self, gpio_pin: u32) {
        initialize_motor(gpio_pin);
        self.motors.push(gpio_pin);
    }

    //PID STUFF
    pub fn start_pid_loop(&self, config: Config, peer: &mut Peer, debug_pipe : Sender<debug_server::Signal>) {

        let controller_input = peer.subscribe_input();

        let sensor_input: Receiver<SensorOutput>;
        let sensor_poll_time = config.sensor_poll_time;
        match start_sensors(sensor_poll_time, config.sea_level_pressure) {
            Ok(recv) => {
                sensor_input = recv;
            }
            Err(e) => {
                println!("[Motors]: Couldn't start sensors. Stopping. {:?}", e);
                return;
            }
        };

        let motor_1 = self.motors[0];
        let motor_2 = self.motors[1];
        let motor_3 = self.motors[2];
        let motor_4 = self.motors[3];

        if config.motors_on {
            self.arm();
        }

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

            let mut last_sample_time = time::PreciseTime::now();
            let start = time::PreciseTime::now();

            let kp = config.kp;
            let ki = config.ki;
            let kd = config.kd;

            let mut dynamic_ki: f32 = 0.98;

            let mut mid = config.hover_power as f32;

            loop {
                dynamic_ki = 0.98;
                let mut up_force = 0.0;
                match controller_input.try_recv() {
                    Ok(desired) => {
                        desired_orientation.x = desired.get_orientation().x;
                        desired_orientation.y = desired.get_orientation().y;
                        up_force = desired.get_vertical_velocity();
                        'inner: loop {
                            match controller_input.try_recv() {
                                Ok(desired) => {
                                    desired_orientation.x = desired.get_orientation().x;
                                    desired_orientation.y = desired.get_orientation().y;
                                    up_force = desired.get_vertical_velocity();
                                },
                                Err(_) => {
                                    break 'inner;
                                }
                            }
                        }
                    },
                    Err(_) => { }
                }

                mid = mid + up_force;
                let SensorOutput(mut current_orientation, mut current_altitude) = sensor_input.recv().unwrap();
                loop {
                    match sensor_input.try_recv() {
                        Ok(SensorOutput(orientation, altitude)) => {
                            current_orientation = orientation;
                            current_altitude = altitude;
                        }
                        Err(_) => {
                            break;
                        }
                    }
                }


                let t = time::PreciseTime::now();
                let dt: f32 = last_sample_time.to(t).num_microseconds().unwrap() as f32 / 1000000.0;
                last_sample_time = t;

                //Safety check
                if current_orientation.x.abs() > config.motor_cutoff {
                    println!("[Motors]: Tilted too far. {:?}", current_orientation);
                    terminate_all_motors(debug_pipe);
                    std::process::exit(0);
                }


                let proportional = desired_orientation - current_orientation;

                integral = integral + proportional * dt;
                integral = integral * dynamic_ki;
                let derivative = (proportional - last_proportional) / dt;
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

                debug_pipe.send(debug_server::Signal::Log(debug_data)).unwrap();

                let x_1 = mid + power.x;
                let x_2 = mid + power.x;
                let x_3 = mid - power.x;
                let x_4 = mid - power.x;

                let y_1 = mid - power.y;
                let y_2 = mid + power.y;
                let y_3 = mid + power.y;
                let y_4 = mid - power.y;

                let m_1 = (x_1 + y_1) / 2.0;
                let m_2 = (x_2 + y_2) / 2.0;
                let m_3 = (x_3 + y_3) / 2.0;
                let m_4 = (x_4 + y_4) / 2.0;

                if config.motors_on {
                    set_power(motor_1, m_1 as u32);
                    set_power(motor_2, m_2 as u32);
                    set_power(motor_3, m_3 as u32);
                    set_power(motor_4, m_4 as u32);
                }
            }
        });
    }
}

// Cleanflight:
//0.12029;
// 0.244381;
//0.000529;

//const MAX_RANGE: f32 = 300.0;
//const MIN_RANGE: f32 = 100.0;
//
//const MAX_MID_ACCEL: f32 = 10.0;
//const MAX_MIN_DECCEL: f32 = -10.0;

impl std::ops::Drop for MotorManager {
    fn drop(&mut self) {
        self.terminate();
    }
}


fn initialize_motor(gpio_pin: u32) -> u32 {
    let config = Config::new();
    let mut freq: u32 = ( 1000 / config.sensor_poll_time ) as u32;
    if freq < 50 {
        freq = 50;
    }
    let range: u32 = 1000000 / freq;
    set_mode(gpio_pin, OUTPUT).unwrap();
    set_pwm_range(gpio_pin, range).unwrap();
    set_pwm_frequency(gpio_pin, freq).unwrap();
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
        let handle = thread::spawn(move || {
            pwm(motor, 0).unwrap();
            sleep(Duration::from_secs(4));
//            pwm(motor, 1500).unwrap();
//            sleep(Duration::from_secs(1));
            pwm(motor, 2000).unwrap();
            sleep(Duration::from_secs(4));
            pwm(motor, 1000).unwrap();
            sleep(Duration::from_secs(4));
            write(motor, OFF).unwrap();
            sleep(Duration::from_secs(3));
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("[Motors]: Motors calibrated");
    println!("[Motors]: Shutting down");

    for motor in config.motors.clone() {
        stop(motor);
    }

    terminate();
    thread::sleep(Duration::from_secs(2));


}

fn arm(motor: u32) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        pwm(motor, 1200).unwrap();
        sleep(Duration::from_secs(1));
        pwm(motor, 1000).unwrap();
        sleep(Duration::from_secs(2));

        pwm(motor, 1250).unwrap();
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
