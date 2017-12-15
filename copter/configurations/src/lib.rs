extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate nalgebra as na;

pub mod config;
pub mod calibrations;

pub type Config = config::Config;
pub type Calibrations = calibrations::Calibrations;
pub type Ellipsoid = calibrations::Ellipsoid;
pub type Simple = calibrations::Simple;
