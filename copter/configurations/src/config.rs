use std::default::Default;
use std::fs::File;
use toml;
use std::io::prelude::*;

/*----- Flight -----*/

#[derive(Debug, Deserialize, Serialize)]
pub struct PID {
    p: Option<f32>,
    i: Option<f32>,
    d: Option<f32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Flight {
    roll: PID,
    pitch: PID,
    yaw: PID,
}

/*----- Hardware -----*/

#[derive(Debug, Deserialize, Serialize)]
enum SerialCommunication {
    UART,
    I2C,
    SPI,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Sensor {
    name: String,
    update_rate: Option<i32>,
    serial: SerialCommunication,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Motors {
    pins: Vec<u32>,
    serial_pwm: bool,
    serial: Option<SerialCommunication>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Hardware {
    gps: bool,
    wifi_gps: bool,
    barometer: Option<Sensor>,
    gyroscope: Sensor,
    accelerometer: Sensor,
    magnetometer: Option<Sensor>,
    analog_converter: Option<Sensor>,
    motors: Motors,
}

/*----- Networking -----*/

#[derive(Debug, Deserialize, Serialize)]
pub struct Networking {
    server_ip: String,
    server_port: i32,
}

/*----- Debug -----*/

#[derive(Debug, Deserialize, Serialize)]
pub struct Debug {
    live_debugging: bool,
    debug_websocket_port: i32,
    logging: bool,
    motors_off: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    flight: Flight,
    hardware: Hardware,
    networking: Networking,
    debug: Debug,
}

impl Config {
    pub fn new() -> Result<Config, String> {
        let mut config_file: File = match File::open("configuration/config.toml") {
            Ok(file) => file,
            Err(e) => {
                println!("Couldn't open config.toml! Opening config_default.toml.");
                File::open("configuration/config_default.toml").unwrap()
            }
        };

        let mut config_string = String::new();
        match config_file.read_to_string(&mut config_string) {
            Ok(_size) => {}
            Err(e) => return Err(e.to_string()),
        };

        match toml::from_str(config_string.as_ref()) {
            Ok(cfg) => Ok(cfg),
            Err(e) => Err(e.to_string()),
        }
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            flight: Flight {
                roll: PID {
                    p: Some(0.0),
                    i: Some(0.0),
                    d: Some(0.0),
                },
                pitch: PID {
                    p: Some(0.0),
                    i: Some(0.0),
                    d: Some(0.0),
                },
                yaw: PID {
                    p: Some(0.0),
                    i: None,
                    d: Some(0.0),
                },
            },
            hardware: Hardware {
                gps: false,
                wifi_gps: false,
                barometer: Some(Sensor {
                    name: String::from("Barometer Model"),
                    update_rate: Some(100),
                    serial: SerialCommunication::I2C,
                }),
                gyroscope: Sensor {
                    name: String::from("Gyroscope Model"),
                    update_rate: Some(100),
                    serial: SerialCommunication::I2C,
                },
                accelerometer: Sensor {
                    name: String::from("Accelerometer Model"),
                    update_rate: Some(100),
                    serial: SerialCommunication::I2C,
                },
                magnetometer: Some(Sensor {
                    name: String::from("Magnetometer Model"),
                    update_rate: Some(100),
                    serial: SerialCommunication::I2C,
                }),
                analog_converter: Some(Sensor {
                    name: String::from("Analog to Digital Converter"),
                    update_rate: Some(100),
                    serial: SerialCommunication::I2C,
                }),
                motors: Motors {
                    pins: vec![1, 2, 3, 4],
                    serial_pwm: true,
                    serial: Some(SerialCommunication::I2C),
                },
            },
            networking: Networking {
                server_ip: String::from("0.0.0.0"),
                server_port: 0000,
            },
            debug: Debug {
                live_debugging: false,
                debug_websocket_port: 0000,
                logging: true,
                motors_off: false,
            },
        }
    }
}
