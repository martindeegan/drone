use i2csensors::{Accelerometer, Barometer, Gyroscope, Magnetometer, Vec3};
use i2cdev::linux::LinuxI2CError;
use super::motors::{MotorCommand, MotorManager};

pub type MockSensorError = LinuxI2CError;

pub struct MockSensor {}

impl MockSensor {
    pub fn new() -> MockSensor {
        MockSensor {}
    }
}

impl Barometer for MockSensor {
    type Error = MockSensorError;
    fn pressure_kpa(&mut self) -> Result<f32, Self::Error> {
        Ok(0.0)
    }
}

impl Gyroscope for MockSensor {
    type Error = MockSensorError;

    fn angular_rate_reading(&mut self) -> Result<Vec3, Self::Error> {
        Ok(Vec3 {
            x: -1.05,
            y: 0.3,
            z: -0.0,
        })
    }
}

impl Accelerometer for MockSensor {
    type Error = MockSensorError;

    fn acceleration_reading(&mut self) -> Result<Vec3, Self::Error> {
        Ok(Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        })
    }
}

impl Magnetometer for MockSensor {
    type Error = MockSensorError;

    fn magnetic_reading(&mut self) -> Result<Vec3, Self::Error> {
        Ok(Vec3::zeros())
    }
}

impl MotorManager for MockSensor {
    fn arm(&mut self) {}
    fn terminate(&mut self) {}
    fn set_powers(&mut self, powers: [f64; 4]) {}
    fn process_command(&mut self, command: MotorCommand) {}
    fn calibrate(&mut self) {}
}
