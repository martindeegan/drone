

pub struct Altitude {
    alt_kp: f32,
    alt_ki: f32,
    alt_kd: f32,
    last_altitude: f32
}

impl Altitude {
    pub fn new() -> Altitude {
        Altitude {
            alt_kp: 0.0,
            alt_ki: 0.0,
            alt_kd: 0.0,
            last_altitude: 0.0
        }
    }

    pub fn get_mid_level(&mut self, current_altitude: f32, desired_altitude: f32, climb: Option<f32>) -> f32 {
        // let proportional = desired_altitude - current_altitude;
        // let derivative = last_altitude - current_altitude;

        0.0
    }
}
