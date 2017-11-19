use std::fs::{File, OpenOptions};
use time;

pub struct FlightLogger {
    log_file: File,
}

impl FlightLogger {
    pub fn new() -> FlightLogger {
        let current_time = time::now();
        let log_name = format!("{}.log", current_time.rfc3339());

        let file = OpenOptions::new().create(true).write(true).open(log_name).unwrap();

        FlightLogger {
            log_file: file,
        }
    }

    pub fn log(&self) {
        let current_time = time::now();
        let log_msg = format!("[{}:{}:{}], {}", current_time.tm_hour, current_time.tm_min, current_time.tm_sec, "L O G G");
    }
}
