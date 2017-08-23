use sensor_manager::MultiSensorData;

pub struct Attitude {
    pitch_kp: f32,
    pitch_ki: f32,
    pitch_kd: f32,
    pitch_i: f32,
    roll_kp: f32,
    roll_ki: f32,
    roll_kd: f32,
    roll_i: f32,
    yaw_kp: f32,
    yaw_ki: f32,
    yaw_kd: f32,
    yaw_i: f32,
}

impl Attitude {
    pub fn new() -> Attitude {

        Attitude {
            pitch_kp: 0.0,
            pitch_ki: 0.0,
            pitch_kd: 0.0,
            pitch_i: 0.0,
            roll_kp: 0.0,
            roll_ki: 0.0,
            roll_kd: 0.0,
            roll_i: 0.0,
            yaw_kp: 0.0,
            yaw_ki: 0.0,
            yaw_kd: 0.0,
            yaw_i: 0.0,
        }
    }

    pub fn get_motor_powers(&mut self, dt: f32, current_attitude: MultiSensorData, current_angular_rate: MultiSensorData,
                            desired_attitude: MultiSensorData, desired_yaw: f32, mid_level: f32) -> (f32, f32, f32, f32)
    {
        // Pitch PID
        let pitch_p = desired_attitude.y - current_attitude.y;
        self.pitch_i += pitch_p * dt;
        let pitch_d = current_angular_rate.y;

        // Roll PID
        let roll_p = desired_attitude.x - current_attitude.x;
        self.roll_i += roll_p * dt;
        let roll_d = current_angular_rate.x;

        // Yaw PID
        let yaw_p = desired_yaw - current_angular_rate.z;
        self.yaw_i += yaw_p * dt;
        let yaw_d = 0.0;
        (0.0, 0.0, 0.0, 0.0)
    }
}
