
struct PID {
    p: Option<f32>,
    i: Option<f32>,
    d: Option<f32>,
}

struct Flight {
    roll: PID,
    pitch: PID,
    yaw: PID,
}

struct Hardware {
    gps: bool,
    internet_gps: bool,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    flight: Flight,
}

impl Config {
    pub fn new() -> Config {}
}
