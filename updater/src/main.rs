use std::process::Command;
extern crate daemonize;

use daemonize::Daemonize;
use std::thread::sleep;
use std::time::Duration;
use std::process::exit;

/* Run this process as a daemon */

fn main() {
    let daemonize = Daemonize::new()
        .pid_file("/tmp/test.pid") // Every method except `new` and `start`
        .chown_pid_file(true)      // is optional, see `Daemonize` documentation
        .working_directory("/tmp") // for default behaviour.
        .user("nobody")
        .group("daemon") // Group name
        .group(2)        // Or group id
        .privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
        Ok(_) => println!("Success, daemonized"),
        Err(e) => println!("{}", e),
    }

    let mut i = 0;
    loop {
        i += 1;
        if i > 1000 {
            i = 0;
            exit(0);
        }
        sleep(Duration::from_secs(1));
    }
}

fn update() {}

fn start_flight_controller() {}

fn stop_flight_controller() {}
