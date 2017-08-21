use rust_pigpio::*;
use rust_pigpio::pwm::*;

use std;
use std::thread;
use std::thread::sleep;
use std::thread::JoinHandle;

use std::time::Duration;
use std::f32;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};
use std::collections::VecDeque;

use std::io::stdin;
use ansi_term::Colour::*;
use time;

const MAX_VALUE: u32 = 2000;
const MIN_VALUE: u32 = 1000;

use sensors::GyroSensorData;
use sensors::start_sensors;
use sensors::SensorOutput;

use connection::InputStream;
use config::Config;
use debug_server;

use std::fs::File;
use std::io::Write;
use std::fs::OpenOptions;

use sensor_manager::{InertialMeasurement,MultiSensorData};

fn stat(values: [f32;40]) -> (f32, f32) {
    let mut average = 0.0;
    for i in 0..40 {
        average += values[i];
    }
    average /= 40.0;

    let mut std = 0.0;
    for i in 0..40 {
        std += (values[i] - average).powi(2);
    }
    std /= 40.0;

    (average, std)
}

struct Log {
    pub t: i64,
    pub m1: u32,
    pub m2: u32,
    pub m3: u32,
    pub m4: u32,
    pub x_ang: f32,
    pub y_ang: f32,
    pub z_ang: f32,
    pub x_p: f32,
    pub x_i: f32,
    pub x_d: f32,
    pub y_p: f32,
    pub y_i: f32,
    pub y_d: f32
}

struct Logger {}

impl Logger {
    pub fn new(on: bool) -> Sender<Log> {
        let (tx,rx): (Sender<Log>,Receiver<Log>) = channel();
        if on {
            thread::spawn(move || {
                let log_file_name = format!("logs/{}_{}_{}_{}_{}_{}_data.csv",
                                            time::now().tm_year, time::now().tm_mon, time::now().tm_yday,
                                            time::now().tm_hour, time::now().tm_min, time::now().tm_sec);
                let mut log_file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .open(log_file_name).expect("Couldn't open or create new file");
                loop {
                    match rx.try_recv() {
                        Ok(log) => {
                            let out = format!("{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n", log.t,
                                              log.m1, log.m2, log.m3, log.m4,
                                              format!("{:.*}", 2, log.x_ang), format!("{:.*}", 2, log.y_ang), format!("{:.*}", 2, log.z_ang),
                                              format!("{:.*}", 2, log.x_p), format!("{:.*}", 2, log.x_i), format!("{:.*}", 2, log.x_d),
                                              format!("{:.*}", 2, log.y_p), format!("{:.*}", 2, log.y_i), format!("{:.*}", 2, log.y_d));
                            log_file.write_all(out.as_bytes());
                        },
                        Err(e) => {}
                    }
                }
            });
        }
        tx
    }
}



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

    pub fn arm(&self, config: &Config) {

        println!("[Motors]: Arming motors.");

        let mut handles: Vec<JoinHandle<()>> = Vec::new();

        for motor in self.motors.clone() {
            handles.push(arm(motor, config.hover_power));
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
    pub fn start_pid_loop(&self, config: Config, controller_input: InputStream, sensor_input: Receiver<InertialMeasurement>, debug_pipe : Sender<debug_server::Signal>) {
        let sensor_poll_time = config.sensor_sample_frequency;

        let motor_1 = self.motors[0];
        let motor_2 = self.motors[1];
        let motor_3 = self.motors[2];
        let motor_4 = self.motors[3];

        if config.motors_on {
            self.arm(&config);
        }

        let mut total_time = 0.0;
        let mut last_total_time = 0.0;

        //PID thread
        thread::Builder::new().name("PID Loop".to_string()).spawn(move || {

            let mut desired_orientation = MultiSensorData::zeros();
            let mut integral = MultiSensorData::zeros();

            let mut logger = Logger::new(config.logging);

            let mut count = 0;
            let mut x_arr = [0.0;40];
            let mut y_arr = [0.0;40];

            let mut last_sample_time = time::PreciseTime::now();
            let start = time::PreciseTime::now();

            let mut x_kp = config.x_kp;
            let mut x_ki = 0.0;
            let mut x_kd = config.x_kd;

            let mut y_kp = config.y_kp;
            let mut y_ki = 0.0;
            let mut y_kd = config.y_kd;

            let mut mid = config.hover_power as f32;

            let mut current_orientation = MultiSensorData::zeros();

            // Clear the sensor channel since later we expect the loop to be running fast enough to
            // only have one signal in the queue.
            loop {
                match sensor_input.try_recv() {
                    Ok(a) => { },
                    Err(_) => {
                        break;
                    },
                }
            }

            let max_motor_speed= config.max_motor_speed as f32;
            loop {
                let mut up_force = 0.0;
                if mid < max_motor_speed {
                    up_force += 0.1;
                    if mid > max_motor_speed - 15.0 {
                        x_ki = config.x_ki;
                        y_ki = config.y_ki;
                    }
                }
                // Get all queued updated from controller stream.
                loop {
                    match controller_input.try_recv() {
                        Ok(desired) => {
                            desired_orientation.x = desired.get_orientation().x;
                            desired_orientation.y = desired.get_orientation().y;
                            up_force = desired.get_vertical_velocity();
                        },
                        Err(_) => {
                            break;
                        }
                    }
                }

                mid = mid + up_force;

                let mut orientation_measurements: InertialMeasurement = sensor_input.recv().unwrap_or(InertialMeasurement {
                    angles: MultiSensorData::zeros(),
                    rotation_rate: MultiSensorData::zeros(),
                    altitude: 0.0
                });
                loop {
                    match sensor_input.try_recv() {
                        Ok(a) => {
                            orientation_measurements = a;
                            // Consider making this a hard failure or removing this.
                            println!("Received duplicate messages...");
                        },
                        Err(_) => {
                            break;
                        },
                    }
                }

                current_orientation = orientation_measurements.angles - MultiSensorData { x: config.angle_offset_x, y: config.angle_offset_y, z: 0.0 };
                let mut derivative = orientation_measurements.rotation_rate;

                let t = time::PreciseTime::now();
                let dt: f32 = last_sample_time.to(t).num_microseconds().unwrap() as f32 / 1000000.0;
                total_time += dt;

                let a = dt / config.derivative_sampling;
//                if total_time - last_total_time > 1.0 {
//                    let c = Config::new();
//                    x_kp = c.x_kp;
//                    pki = c.x_ki;
//                    pkd = c.x_kd;
//
//                    rkp = c.y_kp;
//                    rki = c.y_ki;
//                    rkd = c.y_kd;
//                    if desired_orientation.y != c.desired_angle {
//                        integral = MultiSensorData::zeros();
//                    }
//                    desired_orientation.y = c.desired_angle;
//                    mid = c.hover_power as f32;
//                    last_total_time = total_time;
//                }
                last_sample_time = t;

                //Safety check
                if current_orientation.x.abs() > config.motor_cutoff {
                    println!("[Motors]: Tilted too far. {:?}", current_orientation);
                    terminate_all_motors(debug_pipe);
                    std::process::exit(0);
                }

//                println!("{}", desired_orientation.x);
                let mut proportional = desired_orientation - current_orientation;

                integral = integral + proportional * dt;

                let range = 1.0;

                proportional.x *= x_kp;
                proportional.y *= y_kp;

                if proportional.x.abs() > config.integral_bandwidth {
                    integral.x = 0.0;
                }
                if proportional.y.abs() > config.integral_bandwidth {
                    integral.y = 0.0;
                }

                integral.x *= x_ki;
                integral.y *= y_ki;

                derivative.x *= x_kd;
                derivative.y *= y_kd;

                let u: MultiSensorData = proportional + integral + derivative;
                let power = u * range;

                if config.real_time_debugging {
                    let debug_data = debug_server::DebugInfo {
                        time: start
                            .to(time::PreciseTime::now())
                            .num_microseconds()
                            .unwrap(),
                        pidaxes: debug_server::Axis {
                            power: 0.0,
                            p: current_orientation.x,
                            i: current_orientation.y,
                            d: current_orientation.z,
                            power_y: power.y,
                            p_y: proportional.y,
                            i_y: 0.0,
                            d_y: derivative.y,
                        },
                        power: mid,
                    };

                    match debug_pipe.send(debug_server::Signal::Log(debug_data)) {
                        Ok(o) => {},
                        Err(e) => {
                            return;
                        }
                    }

                    x_arr[count % 40] = current_orientation.x;
                    y_arr[count % 40] = current_orientation.y;
                    let (x_avg, x_std) = stat(x_arr);
                    let (y_avg, y_std) = stat(y_arr);
                    println!("CA: x: {}, y: {}", format!("{:.*}", 2, current_orientation.x), format!("{:.*}", 2, current_orientation.y));
                    println!("AA: x: {}, y: {}", format!("{:.*}", 2, x_avg), format!("{:.*}", 2, y_avg));
                    println!("STD: x: {}, y: {}", format!("{:.*}", 3, x_std), format!("{:.*}", 3, y_std));
                }

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

                if config.motors_on && total_time > 2.0 {
                    set_power(motor_1, m_1 as u32);
                    set_power(motor_2, m_2 as u32);
                    set_power(motor_3, m_3 as u32);
                    set_power(motor_4, m_4 as u32);
                }

                if config.logging {
                    if (count as i32) % config.logging_freq == 0 {
                        match logger.send(Log {
                            t: start.to(time::PreciseTime::now())
                                .num_microseconds()
                                .unwrap(),
                            m1: m_1 as u32,
                            m2: m_2 as u32,
                            m3: m_3 as u32,
                            m4: m_4 as u32,
                            x_ang: current_orientation.x,
                            y_ang: current_orientation.y,
                            z_ang: current_orientation.z,
                            x_p: proportional.x,
                            x_i: integral.x,
                            x_d: derivative.x,
                            y_p: proportional.y,
                            y_i: integral.y,
                            y_d: derivative.y
                        }) {
                            Ok(o) => {},
                            Err(e) => {}
                        }
                    }
                }

                count += 1;
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
//        pwm(motor, 2000).unwrap();
    }

    println!("{}", Green.paint("[Motors]: Plug in the battery now. Then press enter."));
    input = String::new();
    stdin().read_line(&mut input).expect("Error");

    println!("{}", Yellow.paint("[Motors]: Wait until the rising tones finish. Then press enter."));
    input = String::new();
    stdin().read_line(&mut input).expect("Error");
    for motor in config.motors.clone() {
//        pwm(motor, 1000).unwrap();
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

fn arm(motor: u32, starting_power: u32) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        pwm(motor, 1200).unwrap();
        sleep(Duration::from_secs(1));
        pwm(motor, 1000).unwrap();
        sleep(Duration::from_secs(2));
        pwm(motor, starting_power).unwrap();
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
