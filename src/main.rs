use std::io::stdin;
use std::string::String;

mod motor;

use motor::MotorManager;



const MOTOR_1 : u32 = 19;
const MOTOR_2 : u32 = 20;
const MOTOR_3 : u32 = 21;
const MOTOR_4 : u32 = 26;

fn main() {
    let mut manager = MotorManager::new();
//    manager.new_motor(MOTOR_1);
    manager.new_motor(MOTOR_2);
//    manager.new_motor(MOTOR_3);
    manager.new_motor(MOTOR_4);

    'question: loop {
        println!("arm or calibrate");
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Error");

        match input.trim() {
            "arm" => {
                manager.arm();
                break 'question;
            },
            "calibrate" => {
                manager.calibrate();
            },
            _ => { }
        }
    }

    'input: loop {
        println!("Enter power between 1000-2000 or 'stop'");
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Error");

        match input.trim() {
            "stop" => {
                manager.terminate();
                break 'input
            },
            _ => {
                let x: u32 = input.trim().parse().unwrap_or(1100);

                manager.set_power(0, x);
                manager.set_power(1, x);
//                manager.set_power(2, x);
//                manager.set_power(3 , x);
            }
        }
    }


}

