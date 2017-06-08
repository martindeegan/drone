use std;
use std::net::UdpSocket;

use std::string::String;

const SERVER_ADDR: &str = "13.59.251.61:7070";
const LOCAL_ADDR: &str = "0.0.0.0:0";

pub struct Connection {
    sock: UdpSocket,
}

impl Connection {
    pub fn new() -> Connection {
        Connection{ sock: UdpSocket::bind(LOCAL_ADDR).unwrap() }
    }

    pub fn connect_to_server(&self) {
        self.sock.connect(SERVER_ADDR).unwrap();
        let msg: String = String::parse("Drone");
        self.sock.send(msg);

        let mut response = String::new();
        self.sock.recv(&mut response);

        loop {
            self.sock.send(msg);
            self.sock.peek(&mut String::new());
        }
    }

    pub fn listen_for_controller(&self) {

    }


}