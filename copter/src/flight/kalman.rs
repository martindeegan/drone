use std::sync::mpsc::Receiver;

use na::geometry::UnitQuaternion;
use na::{MatrixN, VectorN};

use hardware::{PredictionReading, UpdateReading};

// Keep track of Latitude, Longitute, Altitude, Attitude, Track, Climb
pub struct State {}

pub struct KalmanFilter {
    prediction_rx: Receiver<PredictionReading>,
    update_rx: Receiver<UpdateReading>,
}

impl KalmanFilter {
    pub fn new(
        pred_rx: Receiver<PredictionReading>,
        update_rx: Receiver<UpdateReading>,
    ) -> KalmanFilter {
        KalmanFilter {
            prediction_rx: pred_rx,
            update_rx: update_rx,
        }
    }

    pub fn predict(&mut self) {
        match self.prediction_rx.recv() {
            Ok(pred) => {}
            Err(_) => {}
        };
    }

    pub fn update(&mut self) {
        match self.update_rx.recv() {
            Ok(update) => {}
            Err(_) => {}
        };
    }

    pub fn get_state(&self) -> State {
        State {}
    }

    pub fn update_motors(&mut self) {}
}
