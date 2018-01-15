use std::default::Default;
use std::fs::{File, OpenOptions};
use toml;
use std::io::prelude::*;
use std::string::String;

use na::{Matrix3, Vector3};

#[derive(Debug, Deserialize, Serialize)]
pub struct Simple {
    pub offsets: Vec<f64>,
}

impl Simple {
    pub fn new(offsets: Vector3<f64>) -> Simple {
        Simple {
            offsets: offsets.as_slice().to_vec(),
        }
    }

    pub fn get_offsets(&self) -> Vector3<f64> {
        Vector3::from_column_slice(&self.offsets)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Ellipsoid {
    pub offsets: Vec<f64>,
    pub rotation: Vec<f64>,
    pub gains: Vec<f64>,
}

impl Ellipsoid {
    pub fn new(offsets: Vector3<f64>, rotation: Matrix3<f64>, gains: Vector3<f64>) -> Ellipsoid {
        Ellipsoid {
            offsets: offsets.as_slice().to_vec(),
            rotation: rotation.as_slice().to_vec(),
            gains: gains.as_slice().to_vec(),
        }
    }

    pub fn get_offsets(&self) -> Vector3<f64> {
        Vector3::from_column_slice(&self.offsets)
    }

    pub fn get_rotation(&self) -> Matrix3<f64> {
        Matrix3::from_column_slice(&self.rotation)
    }

    pub fn get_gains(&self) -> Vector3<f64> {
        Vector3::from_column_slice(&self.gains)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Calibrations {
    pub gyroscope: Option<Simple>,
    pub accelerometer: Option<Ellipsoid>,
    pub magnetometer: Option<Ellipsoid>,
}

impl Calibrations {
    pub fn new() -> Result<Calibrations, String> {
        let mut calibration_file: File = match OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("configuration/calibrations.toml")
        {
            Ok(file) => file,
            Err(e) => {
                println!("1: {:?}", e);
                return Err(e.to_string());
            }
        };

        let mut calibration_string = String::new();
        match calibration_file.read_to_string(&mut calibration_string) {
            Ok(_size) => {}
            Err(e) => {
                println!("2: {:?}", e);
                return Err(e.to_string());
            }
        };

        match toml::from_str(calibration_string.as_ref()) {
            Ok(calib) => Ok(calib),
            Err(e) => {
                println!("3: {:?}", e);
                return Err(e.to_string());
            }
        }
    }

    pub fn save(&self) -> Result<(), String> {
        let mut calibration_file: File = match OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("configuration/calibrations.toml")
        {
            Ok(file) => file,
            Err(e) => return Err(e.to_string()),
        };

        let calibration_string: String = format!(
            "# This file is automatically generated. Do not edit!\n\n{}",
            toml::to_string(self).unwrap()
        );

        calibration_file
            .write_all(calibration_string.as_bytes())
            .unwrap();
        Ok(())
    }
}

impl Default for Calibrations {
    fn default() -> Calibrations {
        Calibrations {
            gyroscope: None,
            accelerometer: None,
            magnetometer: None,
        }
    }
}
