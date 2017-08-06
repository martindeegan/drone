extern crate sensors;

pub fn main() {
    let errors_channel = sensors::start_sensors(50, 10.0).unwrap();
    loop {
        match errors_channel.recv() {
            Ok(components) => println!("Err x: {}, y: {}, z: {}", components.x, components.y, components.z),
            Err(_) => {
                println!("Channel closed...");
                break;
            }
        }
    }

}