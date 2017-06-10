
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

use time::{Duration, PreciseTime};

#[cfg(not(rpi))]
const SERVER_ADDR: &str = "127.0.0.1:7070";
#[cfg(rpi)]
const SERVER_ADDR: &str = "13.59.251.61:7070";
const LOCAL_ADDR: &str = "0.0.0.0:27136";

pub struct Connection {
    sock: UdpSocket,
}

impl Connection {
    pub fn new() -> Connection {
        Connection{ sock: UdpSocket::bind(LOCAL_ADDR).unwrap() }
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

        println!("Connecting to {:?}:{}. Waiting for message.", controller_ip, controller_port);
        self.sock.connect(SocketAddrV4::new(controller_ip, controller_port)).unwrap();
        sleep(std::time::Duration::from_secs(2));

        let to_controller: String = String::from("Hello");
        self.sock.send(to_controller.as_bytes()).unwrap();
        println!("Sent something to controller.");

        let mut controller_response: Vec<u8> = Vec::new();
        self.sock.recv(&mut controller_response).unwrap();

        println!("Received '{}' from controller.", String::from_utf8(controller_response).unwrap());
        println!("Successfully connected to controller.");

    }

    pub fn start_connection_loop(&self) -> (Sender<Vec<u8>>,Receiver<Vec<u8>>) {
        let (sender, rx) = channel::<Vec<u8>>();
        let (tx, receiver) = channel::<Vec<u8>>();
        let socket = self.sock.try_clone().unwrap();
        thread::spawn(move || {
            let mut time_since_last_byte = PreciseTime::now();
            let mut time = PreciseTime::now();
            loop {
                if time_since_last_byte.to(PreciseTime::now()) > Duration::seconds(45) {
                    let arr: [u8;1] = [0;1];
                    socket.send(&arr).unwrap();
                }
                if time.to(PreciseTime::now()) > Duration::milliseconds(1) {
                    match rx.try_recv() {
                        Ok(bytes) => {
                            socket.send(bytes.as_ref());
                        },
                        Err(e) => {},
                    }
                }
            }
        });
        (sender, receiver)
    }
}