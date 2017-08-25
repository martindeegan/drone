use super::unbounded_gpsd::GpsdConnection;
use super::unbounded_gpsd::types::{Response,TpvResponse};

use std::sync::mpsc::{Sender,Receiver,channel};
use std::thread;
use std::ops::Drop;
use std::io::Result;
use std::string::String;

//Add wifi location?
//https://crates.io/crates/wifilocation

#[derive(Debug,Clone,Copy)]
pub struct GPSData {
    latitude: f64,
    longitude: f64,
    altitude: Option<f64>,
    speed: Option<f64>,
    climb: Option<f64>,
    track: Option<f64>
}

impl GPSData {
    pub fn zeros() -> GPSData {
        GPSData {
            latitude: 0.0,
            longitude: 0.0,
            altitude: None,
            speed: None,
            climb: None,
            track: None
        }
    }
}

pub fn get_gps() -> Receiver<GPSData> {
    // let data = GPSData::zeros();

    let (gps_tx, gps_rx): (Sender<GPSData>, Receiver<GPSData>) = channel();
    thread::Builder::new().name("GPS Thread".to_string()).spawn(move || {
        let mut gps_connection = GpsdConnection::new("localhost:2947").unwrap();
        gps_connection.watch(true).unwrap();
        gps_connection.set_read_timeout(None).unwrap();

        loop {
            match get_location(&mut gps_connection) {
                Some(data) => {
                    gps_tx.send(data);
                },
                None => {}
            }
        }
    });
    gps_rx
}

fn get_location(gps_connection: &mut GpsdConnection) -> Option<GPSData> {
    match gps_connection.get_response() {
        Ok(response) => {
            return process_gps_response(response);
        },
        Err(e) => None
    }
}

fn process_gps_response(response: Response) -> Option<GPSData> {
    match response {
        Response::Tpv(tpv_response) => process_tpv_response(tpv_response),
        _ => None
    }
}

fn process_tpv_response(response: TpvResponse) -> Option<GPSData> {
    match response {
        TpvResponse::Fix3D {
            device: device,
            time: time,
            mode: mode,
            time_err: time_err,
            lat: lat,
            lat_err: lat_err,
            lon: lon,
            lon_err: lon_err,
            alt: alt,
            alt_err: alt_err,
            track: track,
            track_err: track_err,
            speed: speed,
            speed_err: speed_err,
            climb: climb,
            climb_err: climb_err,
        } => {
            Some(GPSData {
                latitude: lat,
                longitude: lon,
                altitude: Some(alt),
                speed: Some(speed),
                climb: Some(climb),
                track: track
            })
        },
        TpvResponse::Fix2D {
            device: device,
            time: time,
            mode: mode,
            time_err: time_err,
            lat: lat,
            lat_err: lat_err,
            lon: lon,
            lon_err: lon_err,
            track: track,
            track_err: track_err,
            speed: speed,
            speed_err: speed_err,
        } => {
            Some(GPSData {
                latitude: lat,
                longitude: lon,
                altitude: None,
                speed: Some(speed),
                climb: None,
                track: track
            })
        },
        TpvResponse::LatLonOnly {
            device: device,
            time: time,
            mode: mode,
            time_err: time_err,
            lat: lat,
            lat_err: lat_err,
            lon: lon,
            lon_err: lon_err,
            alt: alt,
            alt_err: alt_err,
            track: track,
            track_err: track_err,
            speed: speed,
            speed_err: speed_err,
            climb: climb,
            climb_err: climb_err,
        } => {
            Some(GPSData {
                latitude: lat,
                longitude: lon,
                altitude: alt,
                speed: speed,
                climb: climb,
                track: track
            })
        },
        _ => None
    }
}
