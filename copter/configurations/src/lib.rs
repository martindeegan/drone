extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;

pub mod config;
pub mod calibrations;

pub type Config = config::Config;
pub type Calibrations = calibrations::Calibrations;
