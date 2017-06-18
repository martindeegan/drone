use std::net::UdpSocket;
use std::net::SocketAddrV4;
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::borrow::BorrowMut;

use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;

use std;
use std::string::String;
use std::thread;
use std::thread::sleep;
use std::vec::Vec;
use std::collections::HashMap;

use config::Config;

use time::{Duration, PreciseTime};

use motor;

use protos::generated::controller_input::ControllerInput;
use protobuf::core::{Message, MessageStatic, parse_from_bytes};
use protobuf;

use debug_server;

//#[cfg(not(rpi))]
//const SERVER_ADDR: &str = "127.0.0.1:7070";
//#[cfg(rpi)]
const SERVER_ADDR: &str = "13.59.251.61:7070";
const LOCAL_ADDR: &str = "0.0.0.0:27136";

const INPUT_ID: u8 = 1;

pub struct Peer {
    sock: UdpSocket,
    input_sub: Sender<ControllerInput>,
}

impl Peer {
    pub fn new() -> Peer {
        let (tx, rx) = channel();
        Peer {
            sock: UdpSocket::bind(LOCAL_ADDR).unwrap(),
            input_sub: tx,
        }
    }

    pub fn connect_to_server(&self) {
        let config = Config::new();
        let server_addr = config.server_address;
        println!("[Connection]: Connecting to server");
        let msg: String = String::from("drone");
        self.sock.send_to(msg.as_bytes(), server_addr).unwrap();
        println!("[Connection]: Sent message to server. Awaiting response.");

        let mut response = String::from("                                                             ",);
        unsafe { self.sock.recv(response.as_mut_vec().borrow_mut()).unwrap() };
        println!("[Connection]: Got response: {}", response.trim());

        response = String::from("                                                             ");
        unsafe { self.sock.recv(response.as_mut_vec().borrow_mut()).unwrap() };
        println!("[Connection]: Got controller address: {}", response);
        let controller_ip = Ipv4Addr::from_str(response.trim()).unwrap();

        response = String::from("                                                             ");
        unsafe { self.sock.recv(response.as_mut_vec().borrow_mut()).unwrap() };
        println!("[Connection]: Got controller port: {:?}", response.trim());
        let controller_port: u16 = response.trim().to_string().parse::<u16>().unwrap();

        println!("[Connection]: Connecting to {:?}:{}.", controller_ip, controller_port);
        self.sock
            .connect(SocketAddrV4::new(controller_ip, controller_port))
            .unwrap();
        sleep(std::time::Duration::from_secs(1));
        println!("[Connection]: Here");

        let clone = self.sock.try_clone().unwrap();
        let (tx, rx): (Sender<bool>, Receiver<bool>) = channel();
        thread::spawn(move || {
            loop {
                let msg: String = String::from_str("Hello").unwrap();
                clone.send(msg.as_bytes());
                match rx.try_recv() {
                    Ok(stop) => {
                        break;
                    },
                    _ => { }
                }
            }
        });

        'p2ploop: loop {
            let mut buf = [0; 10];
            match self.sock.recv(&mut buf) {
                Ok(size) => {
                    println!("[Connection]: Received message from endpoint.");
                    if size > 0 {
                        tx.send(true).unwrap();
                        break 'p2ploop;
                    }
                }
                Err(e) => {
                    println!("[Connection]: Error connecting. Retrying.");
                }
            }
        }

        for i in 0..5 {
            let msg: String = String::from_str("Hello").unwrap();
            self.sock.send(msg.as_bytes());
            thread::sleep(Duration::seconds(1).to_std().unwrap());
        }

        println!("[Connection]: Successfully connected to controller.")

    }

    pub fn subscribe_input(&mut self) -> Receiver<ControllerInput> {
        let (tx, rx): (Sender<ControllerInput>, Receiver<ControllerInput>) = channel();
        self.input_sub = tx;
        rx
    }

    pub fn start_connection_loop(&self, debug_pipe : Sender<debug_server::Signal>) {
        let socket = self.sock.try_clone().unwrap();
        let input_subscriber = self.input_sub.clone();
        thread::spawn(move ||
            loop {
                let mut buf: [u8;1000] = [0; 1000];
                match socket.recv(&mut buf) {
                    Ok(size) => {
                        if size > 0 {
                            match buf[1] {
                                INPUT_ID => {
                                    match parse_from_bytes::<ControllerInput>(&buf[0 .. size]) {
                                        Ok(input) => {
                                            input_subscriber.send(input);
                                        },
                                        Err(a) => {
                                            println!("[Connection]: Something bad: {}", a);
                                        }
                                    }
                                },
                                2 => {
                                    motor::terminate_all_motors(debug_pipe);
                                    std::process::exit(0);
                                },
                                _ => {}
                            }
                        }
                    }
                    _ => {continue;}
                }
            });
    }
}
