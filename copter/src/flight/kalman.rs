use std::sync::mpsc::Receiver;

use na::geometry::{Quaternion, UnitQuaternion};
use na::{Matrix4, MatrixN, Unit, Vector3, Vector4, VectorN};
use alga::linear::Transformation;
use na::U10;
use num::traits::Zero;

type StateVector = VectorN<f32, U10>;

use hardware::{PredictionReading, UpdateReading};
use hardware::GPSData;

use logger::ModuleLogger;

// Keep track of Location: (lat, lon, altitude), Velocity: (track, climb), Attitude(w, i, j, k)
pub struct State {
    location: Vector3<f32>,
    attitude: Quaternion<f32>,
}

impl State {
    pub fn from_state_vector(state: &StateVector) -> State {
        State {
            location: Vector3::new(state.data[0], state.data[1], state.data[2]),
            attitude: Quaternion::new(state.data[6], state.data[7], state.data[8], state.data[9]),
        }
    }
}

pub struct KalmanFilter {
    prediction_rx: Receiver<PredictionReading>,
    update_rx: Receiver<UpdateReading>,
    previous_angular_rate: Vector3<f32>,
    attitude: UnitQuaternion<f32>,
    // current_state: StateVector,
    // previous_state: StateVector,
    // previous_prediction: PredictionReading,
    // logger: ModuleLogger,
}

impl KalmanFilter {
    pub fn new(
        pred_rx: Receiver<PredictionReading>,
        update_rx: Receiver<UpdateReading>,
    ) -> KalmanFilter {
        KalmanFilter {
            prediction_rx: pred_rx,
            update_rx: update_rx,
            previous_angular_rate: Vector3::zero(),
            attitude: UnitQuaternion::identity(),
            // current_state: StateVector::zero(),
            // previous_state: StateVector::zero(),
            // previous_prediction: PredictionReading {
            //     angular_rate: Vector3::zero(),
            //     acceleration: Vector3::zero(),
            //     gps_information: None,
            // },
            // logger: ModuleLogger::new("Kalman", None),
        }
    }

    pub fn predict(&mut self, dt: f32) {
        // let current_state = self.get_state();
        let prediction = self.prediction_rx.recv().unwrap();

        // println!(
        //     "x: {}, y: {}, z: {}",
        //     prediction.angular_rate.data[0],
        //     prediction.angular_rate.data[1],
        //     prediction.angular_rate.data[2]
        // );
        //-------- First Order Integrater ---------//
        let w: Quaternion<f32> = Quaternion::from_parts(
            0.0,
            (self.previous_angular_rate + prediction.angular_rate) / 2.0,
        );

        let w_dot: Quaternion<f32> = Quaternion::from_parts(
            0.0,
            (prediction.angular_rate - self.previous_angular_rate) / dt,
        );

        let first_term = 0.5 * dt * w * self.attitude.quaternion();
        let second_term = (1.0 / 4.0 * w * w * self.attitude.quaternion()
            + 1.0 / 2.0 * w_dot * self.attitude.quaternion()) * dt * dt;

        #[rustfmt_skip]
        let attitude_p = UnitQuaternion::from_quaternion(
             self.attitude.quaternion() + first_term + second_term
        );

        self.attitude = attitude_p;

        let transform = self.attitude.transform_vector(&Vector3::new(0.0, 0.0, 1.0));
        println!(
            "x:{:+.2},y:{:+.2},z:{:+.2}",
            transform.data[0], transform.data[1], transform.data[2],
        );

        self.previous_angular_rate = prediction.angular_rate;
    }

    pub fn update(&mut self, dt: f32) {
        let update = self.update_rx.recv().unwrap();
    }

    // pub fn get_state(&self) -> State {
    // State::from_state_vector(&self.current_state)
    // }

    pub fn update_motors(&mut self, m1: f32, m2: f32, m3: f32, m4: f32) {}
}
