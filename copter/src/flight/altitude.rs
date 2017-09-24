use config::Config;

const MAX_DESCEND_RATE: f32 = -0.03;
const MAX_ASCEND_RATE: f32 = 0.25;

pub struct Altitude {
    alt_kp: f32,
    alt_ki: f32,
    alt_kd: f32,
    last_altitude: f32,
    integral: f32,
    pub last_mid_level: f32
}

impl Altitude {
    pub fn new(altitude: f32) -> Altitude {
        let config = Config::new();

        Altitude {
            alt_kp: config.alt_kp,
            alt_ki: config.alt_ki,
            alt_kd: config.alt_kd,
            last_altitude: altitude,
            integral: 0.0,
            last_mid_level: 0.0
        }
    }

    pub fn get_mid_level(&mut self, current_altitude: f32, desired_altitude: f32, climb: Option<f32>, dt: f32) -> f32 {
        let mut proportional = desired_altitude - current_altitude;
        proportional  = proportional * proportional * proportional;
        self.integral += proportional * dt;
        let derivative = (self.last_altitude - current_altitude) / dt;

        let error = proportional * self.alt_kp + self.integral * self.alt_ki + derivative * self.alt_kd;

        if error < MAX_DESCEND_RATE {
            self.last_mid_level += MAX_DESCEND_RATE;
            self.integral -= proportional * dt;
        } else {
            self.last_mid_level += error;
        }

        self.last_mid_level
    }
}
