mod altitude;
mod attitude;
mod imu;
mod navigation;

use self::altitude::Altitude;
use self::attitude::Attitude;
use self::imu::IMU;
use self::navigation::{Navigator,Destination,lat_lon_bearing,lat_lon_distance};

use time::{Duration,PreciseTime};

use std::thread;
use std::sync::mpsc::{channel,Receiver,Sender};
use std::time::Duration as _Duration;
use std::string::String;

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


        let sleep_time = _Duration::new(0, 1000000000 / 400);
        let mut last_time = PreciseTime::now();

        // Initialize all parts of control loop
        let mut imu = IMU::new();
        let mut altitude_holder = Altitude::new();
        let mut attitude_holder = Attitude::new();
        let mut navigator = Navigator::new();

        loop {
            let current_time = PreciseTime::now();
            let dt = (last_time.to(current_time).num_microseconds().unwrap() as f32) / 1000000.0;

            match mode_rx.try_recv() {
                Ok(new_mode) => {
                    match new_mode.clone() {
                        FlightMode::Off => {
                            imu.stop_tracking();
                        },
                        FlightMode::Landing => {
                            imu.start_tracking();
                        },
                        FlightMode::TakeOff => {
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
                FlightMode::Off => {},
                FlightMode::TakeOff => {
                    //Aim for 1m above the start
                    //Aim for 0 track
                }
                FlightMode::Landing => {
                    //Aim for negative climb
                    //Set flight mode off on 0 climb for extended time
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
