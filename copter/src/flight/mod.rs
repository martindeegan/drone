mod altitude;
mod pid;
mod imu;
mod navigation;

use self::altitude::Altitude;
use self::pid::PID;
use self::imu::{Attitude,IMU};
use self::navigation::{Navigator,Destination,lat_lon_bearing,lat_lon_distance};

use hardware::motors::MotorManager;
use hardware::sensors::start_sensors;

use config::Config;

use time::{Duration,PreciseTime};

use std::thread;
use std::sync::mpsc::{channel,Receiver,Sender};
use std::time::Duration as _Duration;
use std::string::String;
use std::fmt;

#[derive(Clone,Copy)]
pub enum FlightMode {
    Off,
    TakeOff,
    Landing,
    Hold,
    Navigation(Destination)
}

pub fn test_dist_bearing() {
    //Ann Arbor
    let curr_lat = 42.2808;
    let curr_lon = -83.7430;
    //New York
    let dest_lat = 40.7128;
    let dest_lon = -74.0059;

    println!("Distance to new york: {}", lat_lon_distance(curr_lat, curr_lon, dest_lat, dest_lon));
    println!("Bearing to new york: {}", lat_lon_bearing(curr_lat, curr_lon, dest_lat, dest_lon));

}

pub fn start_flight() -> Sender<FlightMode> {
    let mut flight_mode = FlightMode::Off;
    let (mode_tx, mode_rx): (Sender<FlightMode>, Receiver<FlightMode>) = channel();

    thread::Builder::new().name("Flight Thread.".to_string()).spawn(move || {
        let motor_manager = MotorManager::new();

        let sleep_time = _Duration::new(0, 1000000000 / 400);
        let mut last_time = PreciseTime::now();

        // Initialize all parts of control loop
        let mut imu = IMU::new();
        let mut altitude_holder = Altitude::new();
        let mut pid_controller = PID::new();
        let mut navigator = Navigator::new();

        let config = Config::new();
        let start_power = config.hover_power;
        let take_off_max = config.max_motor_speed;
        let mut mid_level: f32 = 0.0;

        loop {
            let current_time = PreciseTime::now();
            let dt = (last_time.to(current_time).num_microseconds().unwrap() as f32) / 1000000.0;

            match mode_rx.try_recv() {
                Ok(new_mode) => {
                    match new_mode.clone() {
                        FlightMode::Off => {
                            motor_manager.set_powers(1000.0, 1000.0, 1000.0, 1000.0);
                            imu.stop_tracking();
                        },
                        FlightMode::Landing => {
                            imu.start_tracking();
                        },
                        FlightMode::TakeOff => {
                            mid_level = start_power as f32;
                            imu.start_tracking();
                        },
                        FlightMode::Hold => {
                            imu.start_tracking();
                        },
                        FlightMode::Navigation(desitination) => {}
                    }
                    flight_mode = new_mode;
                },
                Err(e) => {}
            }

            // Get IMU data

            match flight_mode {
                FlightMode::Off => { },
                FlightMode::TakeOff => {
                    if mid_level < take_off_max as f32 {
                        mid_level += 0.5;
                    }
                    imu.read_data();
                    let attitude = imu.get_attitude(dt);
                    let angular_rate = imu.get_angular_rate();
                    // println!("Bearing: {}", imu.get_bearing());
                    println!("Roll: {roll:+06.*}, Pitch: {pitch:+06.*}, Yaw: {yaw:+06.*}, dt: {time:.*}", 2, 2, 2, 8, roll=attitude.x, pitch=attitude.y, yaw=attitude.z, time=dt);

                    let (mut m1, mut m2, mut m3, mut m4) = pid_controller.correct_attitude(dt, attitude, angular_rate, Attitude::zeros(), mid_level);

                    motor_manager.set_powers(m1, m2, m3, m4);
                }
                FlightMode::Landing => {
                    if mid_level > 1000.0 {
                        mid_level -= 0.2;
                    } else {
                        flight_mode = FlightMode::Off;
                    }
                    imu.read_data();
                    let attitude = imu.get_attitude(dt);
                    let angular_rate = imu.get_angular_rate();

                    let (mut m1, mut m2, mut m3, mut m4) = pid_controller.correct_attitude(dt, attitude, angular_rate, Attitude::zeros(), mid_level);

                    motor_manager.set_powers(m1, m2, m3, m4);
                },
                FlightMode::Hold => {
                    //Aim for 0 climb and track
                },
                FlightMode::Navigation(desitination) => {
                    //Aim for target
                }
            }

            last_time = current_time;
            thread::sleep(sleep_time)
        }

    });

    mode_tx
}
