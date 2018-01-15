// mod pid;
mod navigation;
mod kalman;

// use self::altitude::Altitude;
// use self::pid::PID;
// use self::imu::{Attitude, IMU};
// use self::navigation::{lat_lon_bearing, lat_lon_distance, Destination, Navigator};
use self::kalman::{KalmanFilter, State};
use hardware::{MotorCommand, PredictionReading, UpdateReading};

use na::geometry::{Quaternion, UnitQuaternion};
use na::Vector3;

use configurations::Config;
use logger::ModuleLogger;
use debug_server::{DebugInfo, Logger, Signal};
use time::{Duration, PreciseTime};

use std::thread;
use std::thread::{sleep, Builder};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Duration as _Duration;
use std::string::String;
use std::fmt;
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};
use std::io::prelude::*;

const MICROSECONDS_PER_SECOND: f64 = 1000000.0;

#[derive(Clone, Copy)]
pub enum FlightMode {
    Shutdown,
    Off,
    TakeOff,
    Landing,
    Hold,
    // Navigation(Destination),
}


pub fn start_flight_controller(
    pred_rx: Receiver<PredictionReading>,
    update_rx: Receiver<UpdateReading>,
    motor_tx: Sender<MotorCommand>,
) {
    let logger = ModuleLogger::new("Flight", None);
    logger.log("Initializing flight controller.");

    let (mode_tx, mode_rx): (Sender<FlightMode>, Receiver<FlightMode>) = channel();
    let mut kalman_filter = KalmanFilter::new(pred_rx, update_rx);

    Builder::new()
        .name(String::from("Control thread"))
        .spawn(move || {
            control_loop(kalman_filter, motor_tx, mode_rx);
        });
}

fn control_loop(
    mut kalman_filter: KalmanFilter,
    motor_tx: Sender<MotorCommand>,
    mode_rx: Receiver<FlightMode>,
) {
    let logger = ModuleLogger::new("Flight", None);
    let config = Config::new().unwrap();
    let addr = format!(
        "{}:{}",
        config.networking.server_ip, config.networking.server_port
    );
    let local = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 0);
    let mut client = UdpSocket::bind(local).unwrap();

    logger.log("Control loop started.");
    let mut prev_time = PreciseTime::now();
    sleep(Duration::milliseconds(10).to_std().unwrap());
    let mut count = 0;
    'control: loop {
        let current_time = PreciseTime::now();
        let diff = prev_time.to(current_time);
        let dt = (diff.num_microseconds().unwrap() as f64) / MICROSECONDS_PER_SECOND;
        kalman_filter.predict(dt);
        kalman_filter.update(dt);
        prev_time = current_time;

        println!("{:?}", kalman_filter.x);

        motor_tx.send(MotorCommand::SetPower(0.0, 0.0, 0.0, 0.0));

        let att_vec = kalman_filter.x.attitude.coords;
        if count % 3 == 0 {
            let msg: String = format!(
                "{{ \"position\": [{}, {}, {}], \"attitude\": [{}, {}, {}, {}] }}",
                0.0, 0.0, 0.0, att_vec.data[0], att_vec.data[1], att_vec.data[2], att_vec.data[3]
            );
            client.send_to(msg.as_bytes(), &addr).unwrap();
        }
        count += 1;
    }
}
// pub fn start_flight() -> (Sender<FlightMode>, thread::JoinHandle<()>) {
//     let (mode_tx, mode_rx): (Sender<FlightMode>, Receiver<FlightMode>) = channel();

//     let config = Config::new();
//     let start_power = config.hover_power;
//     let take_off_max = config.max_motor_speed;
//     let mut mid_level: f64 = 1000.0;
//     let mut start_altitude = 0.0;
//     let logging = config.logging;
//     let loop_time = 1000000.0 / config.sensor_sample_frequency as f64;
//     let power_diff = take_off_max - start_power;
//     let take_off_time = config.take_off_time * 1000000.0;
//     let loops_to_run = take_off_time / loop_time;
//     let ramp_amount = power_diff / loops_to_run;

//     let control_thread = thread::Builder::new()
//         .name("Flight Thread.".to_string())
//         .spawn(move || {
//             println!("[Flight]: Starting control loop");
//             control_loop(
//                 &mode_rx,
//                 logging,
//                 take_off_max as f64,
//                 start_power,
//                 ramp_amount,
//             );
//         });

//     (mode_tx, control_thread.unwrap())
// }

// fn control_loop(
//     mode_rx: &Receiver<FlightMode>,
//     logging: bool,
//     take_off_max: f64,
//     take_off_start: f64,
//     ramp_amount: f64,
// ) {
//     let mut flight_mode = FlightMode::Off;

//     // Initialize all parts of control loop
//     let mut motor_manager = MotorManager::new();
//     let mut imu = IMU::new(start_sensors());
//     let mut altitude_holder = Altitude::new(imu.last_altitude);
//     let mut pid_controller = PID::new();
//     let mut navigator = Navigator::new();

//     let log_pipe = Logger::new(logging);
//     let mut time = 0.0;

//     let mut mid_level = take_off_start;

//     let sleep_time = _Duration::new(0, 1000000000 / 400);
//     let mut last_time = PreciseTime::now();

//     imu.read_data();

//     'control: loop {
//         let current_time = PreciseTime::now();
//         let dt = (last_time.to(current_time).num_microseconds().unwrap() as f64) / 1000000.0;

//         check_flight_mode(
//             &mode_rx,
//             &mut flight_mode,
//             &mut imu,
//             &mut motor_manager,
//             &mut altitude_holder,
//         );

//         // Get IMU data
//         imu.read_data();
//         imu.compute_intertial_reference(dt);
//         navigator.get_location();

//         println!(
//             "Roll: {roll:+06.*}, Pitch: {pitch:+06.*}, Yaw: {yaw:+06.*}, dt: {time:.*}",
//             2,
//             2,
//             2,
//             8,
//             roll = imu.last_attitude.x,
//             pitch = imu.last_attitude.y,
//             yaw = imu.last_attitude.z,
//             time = dt
//         );
//         // println!("Alt: {:.*}, {}", 2, imu.last_altitude, imu.start_altitude);
//         match flight_mode {
//             FlightMode::Shutdown => {
//                 break 'control;
//             }
//             FlightMode::Off => {}
//             FlightMode::TakeOff => {
//                 handle_take_off(
//                     &imu,
//                     &mut flight_mode,
//                     &mut motor_manager,
//                     &mut pid_controller,
//                     &mut altitude_holder,
//                     take_off_max,
//                     ramp_amount,
//                     dt,
//                 );
//             }
//             FlightMode::Landing => {
//                 handle_landing(
//                     &imu,
//                     &mut flight_mode,
//                     &mut motor_manager,
//                     &mut pid_controller,
//                     &mut altitude_holder,
//                     dt,
//                 );
//             }
//             FlightMode::Hold => {
//                 handle_hold();
//             }
//             FlightMode::Navigation(desitination) => {
//                 handle_navigation();
//             }
//         }

//         if logging {
//             time += dt;
//             send_log_data(&log_pipe, &imu, &motor_manager, time);
//         }

//         last_time = current_time;
//     }
// }

// fn handle_take_off(
//     imu: &IMU,
//     flight_mode: &mut FlightMode,
//     motor_manager: &mut MotorManager,
//     pid_controller: &mut PID,
//     altitude_holder: &mut Altitude,
//     take_off_max: f64,
//     ramp_amount: f64,
//     dt: f64,
// ) {
//     let mid_level = altitude_holder.get_mid_level(imu.last_altitude, 1.5, None, dt);

//     let attitude = imu.last_attitude;
//     let angular_rate = imu.last_angular_rate;

//     let (mut m1, mut m2, mut m3, mut m4) =
//         pid_controller.correct_attitude(dt, attitude, angular_rate, Attitude::zeros(), mid_level);

//     motor_manager.set_powers(m1, m2, m3, m4);
// }

// fn handle_landing(
//     imu: &IMU,
//     flight_mode: &mut FlightMode,
//     motor_manager: &mut MotorManager,
//     pid_controller: &mut PID,
//     altitude_holder: &mut Altitude,
//     dt: f64,
// ) {
//     let mid_level = altitude_holder.get_mid_level(imu.last_altitude, -1.0, None, dt);

//     let attitude = imu.last_attitude;
//     let angular_rate = imu.last_angular_rate;

//     let (mut m1, mut m2, mut m3, mut m4) =
//         pid_controller.correct_attitude(dt, attitude, angular_rate, Attitude::zeros(), mid_level);

//     motor_manager.set_powers(m1, m2, m3, m4);
// }

// fn handle_hold() {}

// fn handle_navigation() {}

// fn check_flight_mode(
//     mode_rx: &Receiver<FlightMode>,
//     flight_mode: &mut FlightMode,
//     imu: &mut IMU,
//     motor_manager: &mut MotorManager,
//     altitude_holder: &mut Altitude,
// ) {
//     match mode_rx.try_recv() {
//         Ok(new_mode) => {
//             match new_mode.clone() {
//                 FlightMode::Shutdown => {
//                     println!("[Flight]: Shutting down control loop.");
//                 }
//                 FlightMode::Off => {
//                     println!("[Flight]: Set Off Mode.");
//                     motor_manager.set_powers(1000.0, 1000.0, 1000.0, 1000.0);
//                 }
//                 FlightMode::Landing => {
//                     println!("[Flight]: Set Landing Mode.");
//                 }
//                 FlightMode::TakeOff => {
//                     println!("[Flight]: Set Take Off Mode.");
//                     altitude_holder.last_mid_level = 1100.0;
//                     imu.yaw_offset = imu.last_attitude.z;
//                     imu.start_altitude = imu.last_altitude;
//                 }
//                 FlightMode::Hold => {
//                     println!("[Flight]: Set Hold Mode.");
//                 }
//                 FlightMode::Navigation(desitination) => {
//                     println!("[Flight]: Set Navigation Mode.");
//                 }
//             }
//             *flight_mode = new_mode;
//         }
//         Err(e) => {}
//     }
// }

// fn send_log_data(logger: &Sender<Signal>, imu: &IMU, motor_manager: &MotorManager, time: f64) {
//     let data = DebugInfo {
//         t: time,
//         m1: motor_manager.last_m1,
//         m2: motor_manager.last_m2,
//         m3: motor_manager.last_m3,
//         m4: motor_manager.last_m4,
//         x_ang: imu.last_attitude.x,
//         y_ang: imu.last_attitude.y,
//         z_ang: imu.last_attitude.z,
//         x_p: 0.0,
//         x_i: 0.0,
//         x_d: 0.0,
//         y_p: 0.0,
//         y_i: 0.0,
//         y_d: 0.0,
//         x_ang_rate: imu.last_angular_rate.x,
//         y_ang_rate: imu.last_angular_rate.y,
//         z_ang_rate: imu.last_angular_rate.z,
//         x_accel: imu.last_acceleration.x,
//         y_accel: imu.last_acceleration.y,
//         z_accel: imu.last_acceleration.z,
//         x_mag: imu.last_magnetic_reading.x,
//         y_mag: imu.last_magnetic_reading.y,
//         z_mag: imu.last_magnetic_reading.z,
//     };

//     logger.send(Signal::Log(data));
// }

// pub fn test_dist_bearing() {
//     //Ann Arbor
//     let curr_lat = 42.2808;
//     let curr_lon = -83.7430;
//     //New York
//     let dest_lat = 40.7128;
//     let dest_lon = -74.0059;

//     println!(
//         "Distance to new york: {}",
//         lat_lon_distance(curr_lat, curr_lon, dest_lat, dest_lon)
//     );
//     println!(
//         "Bearing to new york: {}",
//         lat_lon_bearing(curr_lat, curr_lon, dest_lat, dest_lon)
//     );
// }
