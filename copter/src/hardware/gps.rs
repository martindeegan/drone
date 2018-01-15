use unbounded_gpsd::GpsdConnection;
use unbounded_gpsd::types::{Response, TpvResponse};
use wifilocation::{get_api_key_from_file, get_towers, WifiGPS};

use logger::{FlightLogger, ModuleLogger};
use configurations::Config;

use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::ops::Drop;
use std::io::Result;
use std::string::String;
use std::cmp::PartialEq;

use std::time::{Duration, Instant};

//Add wifi location?
//https://crates.io/crates/wifilocation

#[derive(Debug, Clone, Copy)]
pub struct GPSData {
    pub latitude: f64,
    pub lat_err: Option<f64>,
    pub longitude: f64,
    pub lon_err: Option<f64>,
    pub altitude: Option<f64>,
    pub alt_err: Option<f64>,
    pub speed: Option<f64>,
    pub speed_err: Option<f64>,
    pub climb: Option<f64>,
    pub climb_err: Option<f64>,
    pub track: Option<f64>,
    pub track_err: Option<f64>,
}

impl GPSData {
    pub fn zeros() -> GPSData {
        GPSData {
            latitude: 0.0,
            lat_err: None,
            longitude: 0.0,
            lon_err: None,
            altitude: None,
            alt_err: None,
            speed: None,
            speed_err: None,
            climb: None,
            climb_err: None,
            track: None,
            track_err: None,
        }
    }
}

impl PartialEq for GPSData {
    fn eq(&self, other: &GPSData) -> bool {
        self.latitude == other.latitude && self.longitude == other.longitude
            && self.altitude == other.altitude && self.speed == other.speed
            && self.climb == other.climb && self.track == other.track
    }
}

pub fn get_gps() -> Receiver<GPSData> {
    let (gps_tx, gps_rx): (Sender<GPSData>, Receiver<GPSData>) = channel();
    let logger = ModuleLogger::new("GPS", None);

    #[cfg(target_arch = "arm")]
    thread::Builder::new()
        .name("GPS Thread".to_string())
        .spawn(move || {
            let gps_logger = ModuleLogger::new("GPS", None);
            let config = Config::new().unwrap();
            gps_logger.log("Initializing GPS");

            let mut gps_connection = GpsdConnection::new("localhost:2947").unwrap();
            gps_connection.watch(true).unwrap();
            gps_connection.set_read_timeout(None).unwrap();
            // let mut wifi_gps = WifiGPS::new(get_api_key_from_file("./geolocation_api_key.key").unwrap());

            loop {
                let mut data = GPSData::zeros();
                match get_location(&mut gps_connection) {
                    Some(gps_data) => {
                        data = gps_data;
                    }
                    None => {}
                }

                // let towers = get_towers();
                // match wifi_gps.get_location(towers) {
                //     Ok(wifi_data) => if wifi_data.accuracy < 10.0 {
                //         data.latitude = wifi_data.location.lat;
                //         data.longitude = wifi_data.location.lng;
                //     },
                //     Err(e) => {}
                // }

                if data != GPSData::zeros() {
                    gps_tx.send(data);
                }
                thread::sleep_ms(100);
            }
        });

    /*--------- Mock GPS -----------*/
    // Would be nice to move this into mock.rs

    #[cfg(not(target_arch = "arm"))]
    thread::spawn(move || {
        let gps_logger = ModuleLogger::new("GPS", None);
        gps_logger.log("Initializing GPS");
        gps_tx.send(GPSData::zeros());
        loop {
            thread::sleep(Duration::from_secs(20));
            gps_tx.send(GPSData::zeros());
        }
    });

    /*--------- Check that GPS is tracking -----------*/
    // match gps_rx.recv_timeout(Duration::from_secs(60)) {
    //     Ok(_) => logger.log("GPS check."),
    //     Err(err) => {
    //         logger.error("GPS failed to respond in time. Check that GPSD is running. Check that your GPS is running correctly. Check GPS fix.");
    //         panic!("{:?}", err);
    //     }
    // }

    gps_rx
}

fn get_location(gps_connection: &mut GpsdConnection) -> Option<GPSData> {
    match gps_connection.get_response() {
        Ok(response) => {
            return process_gps_response(response);
        }
        Err(e) => {
            return None;
        }
    }
}

fn process_gps_response(response: Response) -> Option<GPSData> {
    match response {
        Response::Tpv(tpv_response) => process_tpv_response(tpv_response),
        _ => None,
    }
}

fn process_tpv_response(response: TpvResponse) -> Option<GPSData> {
    match response {
        TpvResponse::Fix3D {
            device,
            time,
            mode,
            time_err,
            lat,
            lat_err,
            lon,
            lon_err,
            alt,
            alt_err,
            track,
            track_err,
            speed,
            speed_err,
            climb,
            climb_err,
        } => Some(GPSData {
            latitude: lat,
            lat_err: lat_err,
            longitude: lon,
            lon_err: lon_err,
            altitude: Some(alt),
            alt_err: alt_err,
            speed: Some(speed),
            speed_err: speed_err,
            climb: Some(climb),
            climb_err: climb_err,
            track: track,
            track_err: track_err,
        }),
        TpvResponse::Fix2D {
            device,
            time,
            mode,
            time_err,
            lat,
            lat_err,
            lon,
            lon_err,
            track,
            track_err,
            speed,
            speed_err,
        } => Some(GPSData {
            latitude: lat,
            lat_err: lat_err,
            longitude: lon,
            lon_err: lon_err,
            altitude: None,
            alt_err: None,
            speed: Some(speed),
            speed_err: speed_err,
            climb: None,
            climb_err: None,
            track: track,
            track_err: track_err,
        }),
        TpvResponse::LatLonOnly {
            device,
            time,
            mode,
            time_err,
            lat,
            lat_err,
            lon,
            lon_err,
            alt,
            alt_err,
            track,
            track_err,
            speed,
            speed_err,
            climb,
            climb_err,
        } => Some(GPSData {
            latitude: lat,
            lat_err: lat_err,
            longitude: lon,
            lon_err: lon_err,
            altitude: alt,
            alt_err: alt_err,
            speed: speed,
            speed_err: speed_err,
            climb: climb,
            climb_err: climb_err,
            track: track,
            track_err: track_err,
        }),
        _ => None,
    }
}
