use std::default::Default;
use std::fs::File;
use toml;
use std::io::prelude::*;

/*----- Flight -----*/

#[derive(Debug, Deserialize, Serialize)]
pub struct PID {
    pub p: Option<f32>,
    pub i: Option<f32>,
    pub d: Option<f32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Flight {
    pub roll: PID,
    pub pitch: PID,
    pub yaw: PID,
}

/*----- Hardware -----*/

#[derive(Debug, Deserialize, Serialize)]
pub enum SerialCommunication {
    UART, // Unused currently
    I2C,
    SPI, // Unused currently
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Sensor {
    pub name: String,
    pub update_rate: Option<i32>,    //Unused
    pub serial: SerialCommunication, // Unused
    pub slave_address: u16,          // Unused
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Motors {
    pub pins: Vec<u8>,
    pub serial_pwm: bool,
    pub serial_controller: Option<Sensor>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Battery {
    pub cells: i32,
    pub warning_voltage: f32,  // Per cell
    pub critical_voltage: f32, // Per cell
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Hardware {
    pub gps: bool,
    pub wifi_gps: bool,
    pub barometer: Sensor,
    pub gyroscope: Sensor,
    pub accelerometer: Sensor,
    pub magnetometer: Sensor,
    pub analog_converter: Option<Sensor>,
    pub motors: Motors,
    pub battery: Battery,
}

/*----- Networking -----*/

#[derive(Debug, Deserialize, Serialize)]
pub struct Networking {
    pub server_ip: String,
    pub server_port: i32,
}

/*----- Debug -----*/

#[derive(Debug, Deserialize, Serialize)]
pub struct Debug {
    pub live_debugging: bool,
    pub debug_websocket_port: i32,
    pub logging: bool,
    pub motors_off: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub flight: Flight,
    pub hardware: Hardware,
    pub networking: Networking,
    pub debug: Debug,
}

impl Config {
    pub fn new() -> Result<Config, String> {
        let mut config_file: File = match File::open("configuration/config.toml") {
            Ok(file) => file,
            Err(_) => {
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
                barometer: Sensor {
                    name: String::from("Barometer Model"),
                    update_rate: Some(100),
                    serial: SerialCommunication::I2C,
                    slave_address: 0,
                },
                gyroscope: Sensor {
                    name: String::from("Gyroscope Model"),
                    update_rate: Some(100),
                    serial: SerialCommunication::I2C,
                    slave_address: 0,
                },
                accelerometer: Sensor {
                    name: String::from("Accelerometer Model"),
                    update_rate: Some(100),
                    serial: SerialCommunication::I2C,
                    slave_address: 0,
                },
                magnetometer: Sensor {
                    name: String::from("Magnetometer Model"),
                    update_rate: Some(100),
                    serial: SerialCommunication::I2C,
                    slave_address: 0,
                },
                analog_converter: Some(Sensor {
                    name: String::from("Analog to Digital Converter"),
                    update_rate: Some(100),
                    serial: SerialCommunication::I2C,
                    slave_address: 0,
                }),
                motors: Motors {
                    pins: vec![1, 2, 3, 4],
                    serial_pwm: true,
                    serial_controller: Some(Sensor {
                        name: String::from("PWM Controller"),
                        update_rate: None,
                        serial: SerialCommunication::I2C,
                        slave_address: 0,
                    }),
                },
                battery: Battery {
                    cells: 0,
                    warning_voltage: 0.0,
                    critical_voltage: 0.0,
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
