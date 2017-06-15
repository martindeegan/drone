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

use time::{Duration, PreciseTime};

use protos::generated::position::Position;
use protobuf::core::{Message,MessageStatic, parse_from_bytes};
use protobuf;

//#[cfg(not(rpi))]
//const SERVER_ADDR: &str = "127.0.0.1:7070";
//#[cfg(rpi)]
const SERVER_ADDR: &str = "13.59.251.61:7070";
const LOCAL_ADDR: &str = "0.0.0.0:27136";

const POSITION_ID: u8 = 1;

pub struct Peer {
    sock: UdpSocket,
    position_sub: Sender<Position>,
}

impl Peer {
    pub fn new() -> Peer {
        let (tx, rx) = channel();
        Peer { sock: UdpSocket::bind(LOCAL_ADDR).unwrap(), position_sub: tx }
    }

    pub fn connect_to_server(&self) {
        println!("Connecting to server");
        //        self.sock.connect(SERVER_ADDR).unwrap();
        let msg : String = String::from("drone");
        self.sock.send_to(msg.as_bytes(), SERVER_ADDR).unwrap();
        println!("Sent message to server. Awaiting response.");

        let mut response = String::from("                                                             ");
        unsafe { self.sock.recv(response.as_mut_vec().borrow_mut()).unwrap() };
        println!("Got response: {}", response.trim());

        response = String::from("                                                             ");
        unsafe { self.sock.recv(response.as_mut_vec().borrow_mut()).unwrap() };
        let controller_ip = Ipv4Addr::from_str(response.trim()).unwrap();
        println!("Got controller address: {}", response.trim());

        response = String::from("                                                             ");
        unsafe { self.sock.recv(response.as_mut_vec().borrow_mut()).unwrap() };
        println!("Got controller port: {:?}", response.trim());
        let controller_port: u16 = response.trim().to_string().parse::<u16>().unwrap();

        println!("Connecting to {:?}:{}.", controller_ip, controller_port);
        self.sock.connect(SocketAddrV4::new(controller_ip, controller_port)).unwrap();
        sleep(std::time::Duration::from_secs(1));
        println!("Here");

        'p2ploop: loop {
            println!(".");

            let msg: String = String::from_str("Hello").unwrap();
            self.sock.send(msg.as_bytes());

            let mut response: Vec<u8> = Vec::new();
            match self.sock.recv(response.as_mut()) {
                Ok(size) => {
                    if size > 0 {
                        println!("Successfully connected to controller.");
                        break 'p2ploop;
                    }
                },
                Err(e) => {}
            }
            thread::sleep(Duration::seconds(1).to_std().unwrap());
        }

        for i in 0..5 {
            let msg: String = String::from_str("Hello").unwrap();
            self.sock.send(msg.as_bytes());
            thread::sleep(Duration::seconds(1).to_std().unwrap());
        }

        let msg: String = String::from_str("Connected").unwrap();
        self.sock.send(msg.as_bytes());

        println!("Successfully connected to controller.");

    }

    pub fn subscribe_position(&mut self) -> Receiver<Position> {
        let (tx, rx): (Sender<Position>, Receiver<Position>) = channel();
        self.position_sub = tx;
        rx
    }

    pub fn send_position(&self, object: Position) {
        let bytes = object.write_to_bytes().unwrap();
        self.sock.send(bytes.as_ref()).unwrap();
    }

    pub fn start_connection_loop(&self) {
        let socket = self.sock.try_clone().unwrap();
        let pos_sub = self.position_sub.clone();
        thread::spawn(move || {
            loop {
                let mut bytes: Vec<u8> = Vec::new();
                socket.recv(&mut bytes);
                if bytes.capacity() <= 0 {
                    continue;
                }

                match bytes[0] {
                    POSITION_ID => {
                        let pos: Position =  parse_from_bytes(bytes.as_ref()).unwrap();
                        pos_sub.send(pos);
                    }
                    _ => {}
                }
            }
        });
    }
}