extern crate protobuf;

use super::connection::Connection;

use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;

pub struct Tasker {
    tx: Sender<Vec<u8>>,
    rx: Receiver<Vec<u8>>
}

impl Tasker {

    pub fn new(conn: &Connection) -> Tasker {
        let (tx, rx) = conn.start_connection_loop();
        Tasker{ tx: tx, rx: rx }
    }

    pub fn send_object_message<'b>(&self, object: &'b protobuf::core::Message) {
        let bytes = object.write_to_bytes().unwrap();
        self.send_message(bytes);
    }

    pub fn send_message(&self, msg: Vec<u8>) {
        self.tx.send(msg);
    }
}