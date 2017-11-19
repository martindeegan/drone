extern crate ansi_term;
extern crate time;

mod flight_logger;
mod module_logger;

pub type ModuleLogger = module_logger::ModuleLogger;
pub type FlightLogger = flight_logger::FlightLogger;
