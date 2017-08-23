use std::f32::consts::PI;

#[derive(Clone,Copy)]
pub struct Destination {
    lat: Option<f32>,
    lon: Option<f32>,
    altitude: Option<f32>,
    relative_x: Option<f32>,
    relative_y: Option<f32>
}

const EARTH_RADIUS: f32 = 6371000.0; // Meters
const DEGREES_TO_RADIANS: f32 = PI / 180.0;
const RADIANS_TO_DEGREES: f32 = 180.0 / PI;

//Distance
pub fn lat_lon_distance(curr_lat: f32, curr_lon: f32, dest_lat: f32, dest_lon: f32) -> f32 {
    // Haversine formula
    let phi1 = curr_lat * DEGREES_TO_RADIANS;
    let phi2 = dest_lat * DEGREES_TO_RADIANS;
    let dphi = (dest_lat - curr_lat) * DEGREES_TO_RADIANS;
    let dlambda = (dest_lon - curr_lon) * DEGREES_TO_RADIANS;

    let a = (dphi / 2.0).sin() * (dphi / 2.0).sin() +
            phi1.cos() * phi2.cos() *
            (dlambda / 2.0).sin() * (dlambda / 2.0).sin();

    let c = 2.0 * (a.sqrt().atan2((1.0 - a).sqrt()));

    EARTH_RADIUS * c
}

//Initial bearing
pub fn lat_lon_bearing(mut curr_lat: f32, mut curr_lon: f32, mut dest_lat: f32, mut dest_lon: f32) -> f32 {
    curr_lat *= DEGREES_TO_RADIANS; curr_lon *= DEGREES_TO_RADIANS; dest_lat *= DEGREES_TO_RADIANS; dest_lon *= DEGREES_TO_RADIANS;
    let y = (dest_lon - curr_lon).sin() * dest_lat.cos();
    let x = curr_lat.cos() * dest_lat.sin() -
            curr_lat.sin() * dest_lat.cos() * (dest_lon - curr_lon).cos();

    y.atan2(x) * RADIANS_TO_DEGREES
}

pub struct Navigator {

}

impl Navigator {
    pub fn new() -> Navigator {
        Navigator {}
    }
}
