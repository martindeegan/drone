use i2csensors::{Accelerometer, Barometer, Gyroscope, Magnetometer, Vec3};
use i2cdev::linux::LinuxI2CError;

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
        Ok(Vec3::zeros())
    }
}

impl Accelerometer for MockSensor {
    type Error = MockSensorError;

    fn acceleration_reading(&mut self) -> Result<Vec3, Self::Error> {
        Ok(Vec3::zeros())
    }
}

impl Magnetometer for MockSensor {
    type Error = MockSensorError;

    fn magnetic_reading(&mut self) -> Result<Vec3, Self::Error> {
        Ok(Vec3::zeros())
    }
}
