extern crate sensors;

pub fn main() {
    let errors_channel = sensors::start().unwrap();
    loop {
        match errors_channel.recv() {
            Ok(components) => println!("Err y: {}", components.y),
            Err(_) => {
                println!("Channel closed...");
                break;
            }
        }

    }

}