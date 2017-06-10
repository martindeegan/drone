extern crate rust_pigpio;

use self::rust_pigpio::*;
use self::rust_pigpio::pwm::*;
use self::rust_pigpio::constants::*;

use std;
use std::thread;
use std::thread::sleep;
use std::sync::mpsc::channel;

use std::collections::HashSet;
use std::string::String;
use std::time::Duration;


pub struct MotorManager {
    motors: Vec<Motor>
}

impl MotorManager {

    pub fn new() -> MotorManager {
        let mm = MotorManager{ motors: Vec::new() };
        mm.initialize();
        mm
    }

    pub fn initialize(&self) {
        initialize();
        println!("Initialized Motor Manager!");
    }

    pub fn terminate(&mut self) {
        for x in 0..self.motors.capacity() {
            self.motors[x].stop();
        }
        terminate();
        println!("Stopped.");
    }

    pub fn calibrate(&mut self) {
        println!("Calibrating. This may take a moment");

        let motor_1 = self.motors[0].calibrate();
        let motor_2 = self.motors[1].calibrate();
        let motor_3 = self.motors[2].calibrate();
        let motor_4 = self.motors[3].calibrate();

        sleep(Duration::from_secs(10));
        println!("Weird eh! Special tone eh? WHAT DID I SAID!!!");

        motor_1.join().unwrap();
        motor_2.join().unwrap();
        motor_3.join().unwrap();
        motor_4.join().unwrap();

        println!("Done calibrating. Restarting.");
        self.terminate();
        sleep(Duration::from_secs(10));

        self.initialize();
    }

    pub fn arm(&mut self) {
        let num_motors = self.motors.capacity();
        println!("Arming motors.");

        let motor_1 = self.motors[0].arm();
        let motor_2 = self.motors[1].arm();
        let motor_3 = self.motors[2].arm();
        let motor_4 = self.motors[3].arm();

        motor_1.join().unwrap();
        motor_2.join().unwrap();
        motor_3.join().unwrap();
        motor_4.join().unwrap();

        println!("Motors armed.");

        println!("Starting motors.");

        for m in 0..num_motors {
            self.motors[m].set_power(1250);
        }
    }

    pub fn new_motor(&mut self, gpio_pin: u32) {
        //        if !initialized {
        //            println!("Error: Not initialized.");
        //            return;
        //        }
        //        if MotorManager::used_pins.contains(gpio_pin) {
        //            println!("Error: Pin in use.");
        //            return;
        //        }
        let motor = Motor::new(gpio_pin);
        self.motors.push(motor);
        //        used_pins.insert(gpio_pin);
    }

    pub fn set_power(&mut self, motor_num: usize, power: u32) {
        self.motors[motor_num].set_power(power);
    }
}

impl std::ops::Drop for MotorManager {
    fn drop(&mut self) {
        self.terminate();
    }
}

pub struct Motor {
    pin: u32,
    current_power: u32
}

impl Motor {

    pub fn new(gpio_pin: u32) -> Motor {
        set_mode(gpio_pin, OUTPUT);
        set_pwm_range(gpio_pin, 2000);
        set_pwm_frequency(gpio_pin, 500);
        Motor { pin: gpio_pin, current_power: 0 }
    }

    pub fn calibrate(&mut self) -> thread::JoinHandle<()> {
        let gpio = self.pin;
        thread::spawn(move || {
            servo(gpio, 0);
            sleep(Duration::from_secs(4));
            servo(gpio, 2000);
            sleep(Duration::from_secs(4));
            servo(gpio, 1000);
            sleep(Duration::from_secs(8));
            write(gpio, OFF);
            sleep(Duration::from_secs(8));
        })
    }

    pub fn arm(&mut self) -> thread::JoinHandle<()> {
        let gpio = self.pin;
        thread::spawn(move || {
            pwm(gpio, 1000);
            sleep(Duration::from_secs(2));

            pwm(gpio, 1100);
        })
    }

    pub fn set_power(&mut self, power: u32) {
        pwm(self.pin, power);
        self.current_power = power
    }

    pub fn get_power(&self) -> u32 {
        self.current_power
    }

    pub fn stop(&mut self)  {
        write(self.pin, OFF);
        self.current_power = 0;
    }
}

