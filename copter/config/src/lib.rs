extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::vec::Vec;
use std::fs::File;
use std::io::prelude::*;
use std::string::String;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub pkp: f32,
    pub pki: f32,
    pub pkd: f32,
    pub rkp: f32,
    pub rki: f32,
    pub rkd: f32,
    pub desired_angle: f32,
    pub motors: Vec<u32>,
    pub motor_cutoff: f32,
    pub sensor_poll_time: i64,
    pub motors_on: bool,
    pub integral_decay_time: f32,
    pub server_address: String,
    pub hover_power: u32,
    pub max_motor_speed: u32,
    pub debug_websocket_port: i32,
    pub sea_level_pressure: f32,
    pub derivative_sampling: f32,
    pub integral_decay: f32,
    pub sensors: Vec<String>
}

impl Config {
    pub fn new() -> Config {
        let mut file = File::open("config.json").expect("failed to open config.json");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("failed to read config.json");
        // Parse the string of data into serde_json::Value.
        serde_json::from_str(&contents).expect("failed to desrialize json")
    }
}
