use std::vec::Vec;
use std::sync::mpsc::{Receiver, Sender};

use MotorCommand;
use super::kalman::State;

use na::geometry::UnitQuaternion;

type Path = Vec<Location>;

pub struct NavigationInstructions {
    desired_attitude: UnitQuaternion<f64>,
    desired_thrust: f64,
}

pub enum FlightState {
    Off,
    Powered,
    Takeoff,
    Landing,
    Navigating,
}

pub struct Location {
    lat: f64,
    lon: f64,
    alt: f64,
}

pub struct Navigator {
    state: FlightState,
    path: Path,
    motor_command: Sender<MotorCommand>,
    path_receiver: Receiver<Path>,
}

impl Navigator {
    fn new(motor_command: Sender<MotorCommand>, path_receiver: Receiver<Path>) -> Navigator {
        Navigator {
            state: FlightState::Off,
            path: Path::new(),
            motor_command: motor_command,
            path_receiver: path_receiver,
        }
    }

    pub fn update(&mut self, state: &State) -> NavigationInstructions {
        match self.path_receiver.try_recv() {
            Ok(new_path) => {
                self.path = new_path;
                match self.state {
                    FlightState::Off => {
                        self.set_power();
                    }
                    FlightState::Powered => {
                        self.state = FlightState::Takeoff;
                    }
                    FlightState::Landing => {
                        self.state = FlightState::Takeoff;
                    }
                    _ => {}
                };
            }
            Err(_) => {}
        };

        match self.state {
            FlightState::Off => self.handle_off(state),
            FlightState::Powered => self.handle_powered(state),
            FlightState::Takeoff => self.handle_takeoff(state),
            FlightState::Landing => self.handle_landing(state),
            FlightState::Navigating => self.handle_navigation(state),
        }
    }

    fn handle_off(&mut self, state: &State) -> NavigationInstructions {
        NavigationInstructions {
            desired_attitude: UnitQuaternion::identity(),
            desired_thrust: 0.0,
        }
    }

    fn handle_powered(&mut self, state: &State) -> NavigationInstructions {
        NavigationInstructions {
            desired_attitude: state.attitude,
            desired_thrust: 1000.0,
        }
    }

    fn handle_takeoff(&mut self, state: &State) -> NavigationInstructions {
        NavigationInstructions {
            desired_attitude: UnitQuaternion::identity(),
            desired_thrust: 0.0,
        }
    }

    fn handle_landing(&mut self, state: &State) -> NavigationInstructions {
        NavigationInstructions {
            desired_attitude: UnitQuaternion::identity(),
            desired_thrust: 0.0,
        }
    }

    fn handle_navigation(&mut self, state: &State) -> NavigationInstructions {
        NavigationInstructions {
            desired_attitude: UnitQuaternion::identity(),
            desired_thrust: 0.0,
        }
    }

    fn set_off(&mut self) {
        self.motor_command.send(MotorCommand::PowerDown);
        self.state = FlightState::Off;
    }

    fn set_power(&mut self) {
        self.motor_command.send(MotorCommand::Arm);
        self.state = FlightState::Powered;
    }
}
