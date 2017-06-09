use std;
use std::net::UdpSocket;
use std::net::IpAddr;
use std::str::FromStr;
use std::borrow::BorrowMut;

use std::string::String;
use std::thread::sleep;
use std::time::Duration;

const SERVER_ADDR: &str = "10.0.0.28:7070";
const LOCAL_ADDR: &str = "0.0.0.0:0";

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
        self.sock.send_to(msg.as_bytes(), SERVER_ADDR);
        println!("Sent message to server. Awaiting response.");

        let mut response = String::from("                                                             ");
        unsafe { self.sock.recv(response.as_mut_vec().borrow_mut()) };
        println!("Got response: {}", response.trim());

        response = String::from("                                                             ");
        unsafe { self.sock.recv(response.as_mut_vec().borrow_mut()) };
        println!("Got controller address: {}", response.trim());


        println!("Connecting to {}. Waiting for message.", response.trim());
        self.sock.connect(response.trim());
        sleep(Duration::from_secs(2));

        //        let mut controller_response: Vec<u8> = Vec::new();
//        let size = self.sock.recv(&mut controller_response);

//        println!("Received '{}' from controller.", String::from_utf8(controller_response).unwrap());

        let to_controller: String = String::from("Hello");
        self.sock.send(to_controller.as_bytes());
    }

    pub fn listen_for_controller(&self) {

    }


}