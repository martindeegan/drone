extern crate config;
extern crate toml;

use config::Config;
use std::fs::{File, OpenOptions};
use std::string::String;
use std::io::Write;

fn main() {
    let config = Config::default();
    let mut default_file: File = OpenOptions::new()
        .create(true)
        .write(true)
        .open("../configuration/config_default.toml")
        .unwrap();
    let config_str: String = toml::to_string(&config).unwrap();
    default_file.write_all(config_str.as_bytes()).unwrap();
}
