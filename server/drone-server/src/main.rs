extern crate mqttc;
extern crate netopt;

use mqttc::{Client, ClientOptions, ReconnectMethod};
use std::time::Duration;
use netopt::NetworkOptions;
use std::thread;

fn main() {
    let mut opts = ClientOptions::new();
    let netopts = NetworkOptions::new();
    opts.set_reconnect(ReconnectMethod::ReconnectAfter(Duration::from_secs(5)));
    let mut sub_client = opts.connect(self, "127.0.0.1:1883", netopts).unwrap();

    sub_client.subscribe("Location").unwrap();
    sub_client.await().unwrap();

    thread::spawn(move || {
        loop {
            match sub_client.await() {
                Ok(result) => match result {
                    Some(message) => println!("{:?}", message),
                    None => println!("."),
                },
                Err(_) => continue,
            }
        }
    });
}
