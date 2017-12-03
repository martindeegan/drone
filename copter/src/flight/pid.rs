use hardware::sensors::MultiSensorData;
use flight::imu::Attitude;
use configurations::Config;

pub struct PID {
    integral_on: bool,
    roll_kp: f32,
    roll_ki: f32,
    roll_kd: f32,
    pitch_kp: f32,
    pitch_ki: f32,
    pitch_kd: f32,
    yaw_kp: f32,
    yaw_kd: f32,
    integral: MultiSensorData
}

impl PID {
    pub fn new() -> PID {
        let config = Config::new();

        PID {
            integral_on: false,
            roll_kp: config.roll_kp,
            roll_ki: config.roll_ki,
            roll_kd: config.roll_kd,
            pitch_kp: config.pitch_kp,
            pitch_ki: config.pitch_ki,
            pitch_kd: config.pitch_kd,
            yaw_kp: config.yaw_kp,
            yaw_kd: config.yaw_kd,
            integral: MultiSensorData::zeros()
        }
    }

    pub fn correct_attitude(&mut self, dt: f32, current_attitude: Attitude, current_angular_rate: MultiSensorData,
                            desired_attitude: Attitude, mid_level: f32) -> (f32, f32, f32, f32)
    {
        // println!("derivative: {:?}", current_angular_rate);

        let proportional = current_attitude - desired_attitude;
        let derivative = current_angular_rate;
        self.integral = self.integral + proportional * dt;

        let mut error = MultiSensorData::zeros();

        let ki = 0.0;

        // x: Roll, y: Pitch PID
        error.x = proportional.x * self.roll_kp + self.integral.x * self.roll_ki + derivative.x * self.roll_kd;
        error.y = proportional.y * self.pitch_kp + self.integral.y * self.pitch_ki + derivative.y * self.pitch_kd;

        let (mut m1, mut m2, mut m3, mut m4) = (mid_level, mid_level, mid_level, mid_level);
        m1 = (0.0 - error.x + error.y) / 2.0;
        m2 = (0.0 - error.x - error.y) / 2.0;
        m3 = (0.0 + error.x - error.y) / 2.0;
        m4 = (0.0 + error.x + error.y) / 2.0;

        m1 += mid_level;
        m2 += mid_level;
        m3 += mid_level;
        m4 += mid_level;

        // z: Yaw PID is added afterwards
        // Need to compute error both ways and use minimum error
        let yaw_p_1 = proportional.z;
        let mut yaw_p_2 = 0.0;
        if current_attitude.z < desired_attitude.z {
            yaw_p_2 = yaw_p_1 - 360.0;
        } else if current_attitude.z > desired_attitude.z {
            yaw_p_2 = yaw_p_1 + 360.0;
        }
        let mut yaw_p = 0.0;
        if yaw_p_1.abs() < yaw_p_2.abs() {
            yaw_p = yaw_p_1;
        } else {
            yaw_p = yaw_p_2;
        }

        println!("Yaw: {:.*}, Yaw error: {:.*}", 3, current_attitude.z, 3, yaw_p);

        error.z = yaw_p * self.yaw_kp + derivative.z * self.yaw_kd;
        // println!("Yaw err: {}", yaw_p);

        m1 -= error.z;
        m2 += error.z;
        m3 -= error.z;
        m4 += error.z;

        (m1, m2, m3, m4)
    }
}
