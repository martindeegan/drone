use std::sync::mpsc::Receiver;
use std::default::Default;

use na::geometry::{Quaternion, UnitQuaternion};
use na::{Matrix4, MatrixN, Unit, Vector3, Vector4, VectorN};
use alga::linear::{ProjectiveTransformation, Transformation};
use na::U10;
use num::traits::Zero;

type StateVector = VectorN<f32, U10>;

use hardware::{PredictionReading, UpdateReading};
use hardware::GPSData;

use logger::ModuleLogger;

const G_TO_MPSPS: f32 = 9.80665;

// Keep track of Location: (lat, lon, altitude), Velocity: (track, climb), Attitude(w, i, j, k)
pub struct State {
    position: Vector3<f32>,
    velocity: Vector3<f32>,
    attitude: UnitQuaternion<f32>,
}

impl Default for State {
    fn default() -> State {
        State {
            position: Vector3::zero(),
            velocity: Vector3::zero(),
            attitude: UnitQuaternion::identity(),
        }
    }
}

pub struct KalmanFilter {
    prediction_rx: Receiver<PredictionReading>,
    update_rx: Receiver<UpdateReading>,
    previous_state: State,
    previous_control: PredictionReading,
    previous_update: UpdateReading,
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
            previous_state: State::default(),
            previous_control: PredictionReading::default(),
            previous_update: UpdateReading::default(),
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
        let control = self.prediction_rx.recv().unwrap();

        //-------- First Order Gyroscope Integrator ---------//
        let prev_attitude = self.previous_state.attitude;
        let prev_angular_rate = self.previous_control.angular_rate;

        let w: Quaternion<f32> =
            Quaternion::from_parts(0.0, (prev_angular_rate + control.angular_rate) / 2.0);

        let w_dot: Quaternion<f32> =
            Quaternion::from_parts(0.0, (control.angular_rate - prev_angular_rate) / dt);

        let first_term = 0.5 * dt * w * prev_attitude.quaternion();
        let second_term = (1.0 / 4.0 * w * w * prev_attitude.quaternion()
            + 1.0 / 2.0 * w_dot * prev_attitude.quaternion()) * dt * dt;

        let attitude_p =
            UnitQuaternion::from_quaternion(prev_attitude.quaternion() + first_term + second_term);

        //--------- Predict acceleration -------------//
        let prev_position = self.previous_state.position;
        let prev_velocity = self.previous_state.velocity;
        let prev_acceleration = self.previous_control.acceleration;

        let acceleration_avg = (prev_acceleration + control.acceleration) / 2.0;
        let gravity = Vector3::new(0.0, 0.0, G_TO_MPSPS);
        let global_acceleration = attitude_p.transform_vector(&acceleration_avg) - gravity;

        // Predict velocity
        let velocity_p = prev_velocity + global_acceleration * dt;

        // Predict position
        let position_p = prev_position + velocity_p * dt + 0.5 * global_acceleration * dt * dt;


        let state = State {
            position: position_p,
            velocity: velocity_p,
            attitude: attitude_p,
        };

        // println!(
        //     "acceleration: x:{:+.2},y:{:+.2},z:{:+.2}",
        //     global_acceleration.data[0], global_acceleration.data[1], global_acceleration.data[2],
        // );
        // println!(
        //     "position: x:{:+.2},y:{:+.2},z:{:+.2}",
        //     state.position.data[0], state.position.data[1], state.position.data[2],
        // );
        // println!(
        //     "velocity: x:{:+.2},y:{:+.2},z:{:+.2}",
        //     state.velocity.data[0], state.velocity.data[1], state.velocity.data[2],
        // );

        self.previous_state = state;
        self.previous_control = control;
    }

    pub fn update(&mut self, dt: f32) {
        let update = self.update_rx.recv().unwrap();

        // Compute attitude from accelerometer
        let acceleration = update.acceleration;
        let gravity = Vector3::new(0.0, 0.0, G_TO_MPSPS);

        let x = Vector3::new(1.0, 0.0, 0.0);
        let absolute_attitude = UnitQuaternion::new_observer_frame(&gravity, &acceleration);
        let transform = absolute_attitude.inverse_transform_vector(&x);
        println!(
            "orientation: x:{:+.2},y:{:+.2},z:{:+.2}",
            transform.data[0], transform.data[1], transform.data[2],
        );

        // Compute heading from magnetometer
        if update.magnetic_reading.is_some() {
            let magnetic_reading = update.magnetic_reading.unwrap();
        }
    }

    // pub fn get_state(&self) -> State {
    // State::from_state_vector(&self.current_state)
    // }

    pub fn update_motors(&mut self, m1: f32, m2: f32, m3: f32, m4: f32) {}
}
