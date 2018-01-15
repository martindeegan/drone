use hardware::sensors::MultiSensorData;
use flight::imu::Attitude;
use configurations::Config;

use na::Vector3;
use na::UnitQuaternion;
use num::traits::Zero;

use time::{Duration, PreciseTime};

const MICROSECONDS_PER_SECOND: f64 = 1000000.0;

pub struct PID {
    kp: f64,
    ki: f64,
    kd: f64,
    integral: Vector3<f64>,
    last_update: PreciseTime,
}

impl PID {
    pub fn new() -> PID {
        PID {
            kp: 0.0,
            ki: 0.0,
            kd: 1.0,
            integral: Vector3::zero(),
            last_update: PreciseTime::now(),
        }
    }

    // Thank you to Jaeyoon Kim for helping formulate this PID
    // Compute the torque to put on the quadcopter in order to fix the attitude
    fn compute_torque(&mut self, error: Vector3<f64>, angular_rate: Vector3<f64>) -> Vector3<f64> {
        let now = PreciseTime::now();
        let diff = self.last_update.to(now);
        let dt = (diff.num_microseconds() as f64) / MICROSECONDS_PER_SECOND;

        self.integral = self.integral + error * dt
        let torque = -self.kp * error - self.kd * angular_rate - self.ki * self.integral;

        self.last_update = diff;

        torque
    }

    // Output motor speeds to correct attitude
    // Thank you to Jaeyoon Kim for helping formulate this PID
    pub fn control(
        &mut self,
        attitude: UnitQuaternion<f64>,
        angular_rate: UnitQuaternion<f64>,
        desired_attitude: UnitQuaternion<f64>,
    ) -> (f64, f64, f64, f64) {
        let torque = self.compute_torque(attitude.scaled_axis(), angular_rate.scaled_axis());
    }
}
