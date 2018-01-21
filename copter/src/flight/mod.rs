// mod altitude;
// mod pid;
// mod imu;
// mod navigation;
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

const MICROSECONDS_PER_SECOND: f32 = 1000000.0;

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
        })
        .unwrap();
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
    let mut t = 0.0;
    'control: loop {
        let current_time = PreciseTime::now();
        let diff = prev_time.to(current_time);
        let dt = (diff.num_microseconds().unwrap() as f32) / MICROSECONDS_PER_SECOND;
        t += dt;
        kalman_filter.predict(dt);
        kalman_filter.update(dt);
        prev_time = current_time;

        motor_tx.send(MotorCommand::SetPower(0.0, 0.0, 0.0, 0.0));

        let att_vec = kalman_filter.x.attitude.coords;
        if count % 3 == 0 {
            let msg: String = format!(
                "{{ \"position\": [{}, {}, {}], \"attitude\": [{}, {}, {}, {}], \"power\": [{}, {}, {}, {}]}}",
                0.0, 0.0, 0.0, att_vec[0], att_vec[1], att_vec[2], att_vec[3], 1400.0 + 200.0 * t.sin(), 1400.0 + 200.0 * t.cos(), 1400.0 + 200.0 * -t.sin(), 1400.0 + 200.0 * -t.cos()
            );
            client.send_to(msg.as_bytes(), &addr).unwrap();
        }
        count += 1;
    }
}
