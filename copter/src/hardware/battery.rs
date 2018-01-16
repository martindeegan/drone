use configurations::Config;
use logger::ModuleLogger;

use ads111x::*;

pub enum BatteryStatus {
    Full,
    Low,
    Critical,
}

pub struct BatteryMonitor {
    monitor: ADS111X,
    warning_voltage: f32,
    critical_voltage: f32,
    logger: ModuleLogger,
}

impl BatteryMonitor {
    pub fn new() -> Result<BatteryMonitor, ()> {
        let logger = ModuleLogger::new("Battery", None);
        let config = Config::new().unwrap();

        logger.log("Initializing analog to digital converter.");

        let ads1115_config = ADS111XConfig {
            multiplexer_config: ADS1115MultiplexerConfig::AIN0_GND,
            gain_amplifier: ADS11145GainAmplifier::FS_6_144V,
            operating_mode: ADS111XOperatingMode::SingleShot,
            data_rate: ADS111XDataRate::DR_128SPS,
        };

        let device = LinuxI2CDevice::new("/dev/i2c-1", DEFAULT_ADS1115_SLAVE_ADDRESS).unwrap();
        let ads1115 = match ADS111X::new(device, ads1115_config) {
            Ok(analog_to_digital_converter) => analog_to_digital_converter,
            Err(_) => {
                logger.error(
                    "Couldn't start the analog to digital device. Can't monitor battery voltage.",
                );
                return Err(());
            }
        };

        let warning_voltage =
            (config.hardware.battery.cells as f32) * config.hardware.battery.warning_voltage;
        let critical_voltage =
            (config.hardware.battery.cells as f32) * config.hardware.battery.critical_voltage;

        Ok(BatteryMonitor {
            monitor: ads1115,
            warning_voltage: warning_voltage,
            critical_voltage: critical_voltage,
            logger: logger,
        })
    }

    pub fn check_battery(&mut self) -> Result<BatteryStatus, ()> {
        match self.monitor.read_voltage() {
            Ok(voltage) => {
                if voltage < self.critical_voltage {
                    return Ok(BatteryStatus::Critical);
                } else if voltage < self.warning_voltage {
                    return Ok(BatteryStatus::Low);
                } else {
                    return Ok(BatteryStatus::Full);
                }
            }
            Err(_) => Err(()),
        }
    }
}
