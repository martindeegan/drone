use super::unbounded_gpsd::GpsdConnection;
use super::unbounded_gpsd::types::TpvResponse;
use std::ops::Drop;
use std::io::Result;
use std::string::String;


pub struct GPSData {
    latitude: f64,
    longitude: f64,
    altitude: f64,
    speed: f64,
    climb: f64,
    track: f64
}

pub enum GPSResponse {
    Nothing,
    Data(GPSData),
}

pub struct GPS {
    connection: GpsdConnection
}

impl GPS {
    pub fn new() -> GPS {
        let mut connection = GpsdConnection::new("localhost:2947").unwrap();
        connection.watch(true).unwrap();
        connection.set_read_timeout(None).unwrap();
        GPS {
            connection: connection
        }
    }

    // fn process_tpv_response(response: TpvResponse) -> Result<GPSData> {
    //     match response {
    //         TpvResponse::Fix3D {
    //             device: device,
    //             time: time,
    //             mode: mode,
    //             time_err: time_err,
    //             lat: lat,
    //             lat_err: lat_err,
    //             lon: lon,
    //             lon_err: lon_err,
    //             alt: alt,
    //             alt_err: alt_err,
    //             track: track,
    //             track_err: track_err,
    //             speed: speed,
    //             speed_err: speed_err,
    //             climb: climb,
    //             climb_err: climb_err,
    //         } => {
    //             let data = GPSData {
    //                 latitude: lat,
    //                 longitude: lon,
    //                 altitude: alt,
    //                 speed: speed,
    //                 climb: climb,
    //                 track: track
    //             };
    //
    //             return Ok(data);
    //         },
    //         TpvResponse::Fix2D {
    //             device: device,
    //             time: time,
    //             mode: mode,
    //             time_err: time_err,
    //             lat: lat,
    //             lat_err: lat_err,
    //             lon: lon,
    //             lon_err: lon_err,
    //             track: track,
    //             track_err: track_err,
    //             speed: speed,
    //             speed_err: speed_err,
    //         } => {
    //             let data = GPSData {
    //                 latitude: lat,
    //                 longitude: lon,
    //                 altitude: 0.0,
    //                 speed: speed,
    //                 climb: 0.0,
    //                 track: 0.0
    //             };
    //
    //             return Ok(data);
    //         },
    //         TpvResponse::LatLonOnly {
    //             device: device,
    //             time: time,
    //             mode: mode,
    //             time_err: time_err,
    //             lat: lat,
    //             lat_err: lat_err,
    //             lon: lon,
    //             lon_err: lon_err,
    //             alt: alt,
    //             alt_err: alt_err,
    //             track: track,
    //             track_err: track_err,
    //             speed: speed,
    //             speed_err: speed_err,
    //             climb: climb,
    //             climb_err: climb_err,
    //         } => {
    //             let data = GPSData {
    //                 latitude: lat,
    //                 longitude: lon,
    //                 altitude: 0.0,
    //                 speed: 0.0,
    //                 climb: 0.0,
    //                 track: 0.0
    //             };
    //
    //             return Ok(data);
    //         },
    //         _ => Err("No data")
    //     }
    // }

    pub fn get_location(&mut self) {
        self.connection.poll().unwrap();
        match self.connection.get_response() {
            Ok(response) => {
                println!("{:?}", response);
            },
            Err(e) => {}
        }
    }
    //     match self.connection.poll() {
    //         Ok(()) => {
    //             match self.connection.get_response() {
    //                 Ok(response) => {
    //                     match response {
    //                         Tpv(response) => {
    //                             match process_tpv_response(response) {
    //                                 Ok(data) => {
    //                                     return Ok(data)
    //                                 },
    //                                 Err(e) => {
    //                                     return Err(e)
    //                                 }
    //                             }
    //                         },
    //                         Poll(response) => {
    //
    //                         },
    //                         _ => {
    //                             Err("")
    //                         }
    //                     }
    //                 },
    //                 Err(e) => {
    //                     return Err(e);
    //                 }
    //             }
    //         },
    //         Err(e) => Err(e)
    //     }
    // }
}

impl Drop for GPS {
    fn drop(&mut self) {
        self.connection.watch(false).unwrap();
    }
}
