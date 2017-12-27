use std::sync::mpsc::Receiver;
use std::default::Default;

use na::geometry::{Quaternion, UnitQuaternion};
use na::{Matrix3, Matrix4, MatrixMN, MatrixN, Unit, Vector3, Vector4, VectorN};
use alga::linear::{ProjectiveTransformation, Transformation};
use na::{U10, U3, U4, U6, U9};
use num::traits::Zero;

use hardware::{PredictionReading, UpdateReading};
use hardware::GPSData;

use logger::ModuleLogger;

const G_TO_MPSPS: f32 = 9.80665;

type StateVector = VectorN<f32, U4>;
type ErrorStateVector = VectorN<f32, U3>;
type ErrorStateJacobian = MatrixN<f32, U3>;
type CovarianceMatrix = MatrixN<f32, U3>;

// Keep track of Location: (lat, lon, altitude), Velocity: (track, climb), Attitude(w, i, j, k)
pub struct State {
    // pub position: Vector3<f32>,
    // pub velocity: Vector3<f32>,
    pub attitude: UnitQuaternion<f32>,
}

impl Default for State {
    fn default() -> State {
        State {
            // position: Vector3::zero(),
            // velocity: Vector3::zero(),
            attitude: UnitQuaternion::identity(),
        }
    }
}

pub struct KalmanFilter {
    prediction_rx: Receiver<PredictionReading>,
    update_rx: Receiver<UpdateReading>,
    pub state: StateVector,
    covariance: CovarianceMatrix,
    error_state: ErrorStateVector,
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
            state: StateVector::new(1.0, 0.0, 0.0, 0.0),
            covariance: CovarianceMatrix::zero(),
            error_state: ErrorStateVector::zero(),
            previous_control: PredictionReading::default(),
            previous_update: UpdateReading::default(),
        }
    }

    fn sqew_matrix(a: &Vector3<f32>) -> Matrix3<f32> {
        #[rustfmt_skip]
        Matrix3::new(0.0, -a.z, a.y,
                     a.z, 0.0, -a.x,
                     -a.y, a.x, 0.0)
    }

    pub fn predict(&mut self, dt: f32) {
        // let current_state = self.get_state();
        let control = self.prediction_rx.recv().unwrap();

        //-------- First Order Gyroscope Integrator ---------//
        let prev_attitude = Quaternion::from_vector(self.state);
        let prev_angular_rate = self.previous_control.angular_rate;
        let avg_angular_rate = (prev_angular_rate + control.angular_rate) / 2.0;
        let w: Quaternion<f32> = Quaternion::from_parts(0.0, avg_angular_rate);

        let w_dot: Quaternion<f32> =
            Quaternion::from_parts(0.0, (control.angular_rate - prev_angular_rate) / dt);

        let first_term = 0.5 * dt * w * prev_attitude;
        let second_term =
            (1.0 / 4.0 * w * w * prev_attitude + 1.0 / 2.0 * w_dot * prev_attitude) * dt * dt;

        let attitude_p = UnitQuaternion::from_quaternion(prev_attitude + first_term + second_term);

        let skew = KalmanFilter::sqew_matrix(&avg_angular_rate);
        let F = Matrix3::identity() - dt * skew + dt * dt / 2.0 * skew * skew;
        let gyroscope_variance = 0.01;
        let Q = Matrix3::identity() * gyroscope_variance;

        self.covariance = F * self.covariance * F.transpose() + Q;

        self.state = attitude_p.coords;
        self.previous_control = control;
        self.error_state = F * self.error_state;

        // //--------- Predict acceleration -------------//
        // let prev_position = self.previous_state.position;
        // let prev_velocity = self.previous_state.velocity;
        // let prev_acceleration = self.previous_control.acceleration;

        // let acceleration_avg = (prev_acceleration + control.acceleration) / 2.0;
        // let gravity = Vector3::new(0.0, 0.0, G_TO_MPSPS);
        // let global_acceleration = attitude_p.transform_vector(&acceleration_avg) - gravity;

        // // Predict velocity
        // let velocity_p = prev_velocity + global_acceleration * dt;

        // // Predict position
        // let position_p = prev_position + velocity_p * dt + 0.5 * global_acceleration * dt * dt;


        let state = State {
            // position: position_p,
            // velocity: velocity_p,
            attitude: attitude_p,
        };

        {
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
        }
    }

    pub fn update(&mut self, dt: f32) {
        let update = self.update_rx.recv().unwrap();

        // Compute absolute attitude from accelerometer
        let acceleration = update.acceleration;
        let gravity = Vector3::new(0.0, 0.0, G_TO_MPSPS);

        let attitude_a = KalmanFilter::get_absolue_attitude(acceleration, gravity);


        let H: Matrix3<f32> = Matrix3::identity();
        let accelerometer_variance = 0.25;
        let R: Matrix3<f32> = Matrix3::from_element(1.0) - Matrix3::identity()
            + Matrix3::identity() * accelerometer_variance;
        let S = H * self.covariance * H.transpose() + R;
        let K = (self.covariance * H.transpose()).component_div(&S);

        println!("covariance: {:?}", K.data);

        let correction = K * self.error_state;
        let attitude_correction =
            UnitQuaternion::from_quaternion(Quaternion::from_parts(1.0, 0.5 * correction));

        let updated_attitude = attitude_correction
            * UnitQuaternion::from_quaternion(Quaternion::from_vector(self.state));

        self.state = updated_attitude.coords;

        // println!("covariance: {:?}", self.covariance.data);

        // self.covariance = (Matrix3::identity() - K * H) * self.covariance
        //     * (Matrix3::identity() - K * H).transpose()
        //     + K * R * K.transpose();

        // let vec = attitude_a.inverse_transform_vector(&Vector3::new(0.0, 0.0, 1.0));
        // println!(
        //     "attitude: x:{:+.2},y:{:+.2},z:{:+.2}",
        //     vec.data[0], vec.data[1], vec.data[2],
        // );

        // Compute absolute attitude from magnetometer
        // if update.magnetic_reading.is_some() {
        //     let magnetic_reading = update.magnetic_reading.unwrap();
        //     // Ann Arbor magnetic field
        //     let magnetic_field = Vector3::new(18924.2, -2318.0, 50104.5).normalize();

        //     let attitude_m = KalmanFilter::get_absolue_attitude(magnetic_reading, magnetic_field);
        // }

        // // Compute absolute position and velocity
        // if update.gps_information.is_some() {
        //     // GPS Shit
        // }

        self.previous_update = update;
    }

    pub fn update_motors(&mut self, m1: f32, m2: f32, m3: f32, m4: f32) {}

    fn get_absolue_attitude(reading: Vector3<f32>, field: Vector3<f32>) -> UnitQuaternion<f32> {
        let theta = field.angle(&reading);
        if theta != 0.0 {
            let u = field.cross(&reading);
            UnitQuaternion::from_axis_angle(&Unit::new_normalize(u), theta)
        } else {
            UnitQuaternion::identity()
        }
    }

    // pub fn get_state(&self) -> State {
    //     State {
    //         position: Vector3::new(self.state.data[0], self.state.data[1], self.state.data[2]),
    //         velocity: Vector3::new(self.state.data[3], self.state.data[4], self.state.data[5]),
    //         attitude: UnitQuaternion::from_quaternion(Quaternion::from_vector(Vector4::new(self.state.data[6], self.state.data[7], self.state.data[8], self.state.data[9])));
    //     }
    // }
}
