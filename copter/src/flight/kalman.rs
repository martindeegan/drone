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
    pub state: State,
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
            state: State::default(),
            covariance: CovarianceMatrix::zero(),
            error_state: ErrorStateVector::zero(),
            previous_control: PredictionReading::default(),
            previous_update: UpdateReading::default(),
        }
    }

    fn omega_matrix(w: &Vector3<f32>) -> Matrix4<f32> {
        #[rustfmt_skip]
        Matrix4::new(0.0, w.z, -w.y, w.x,
                     -w.z, 0.0, w.x, w.y,
                     w.y, -w.x, 0.0, w.z,
                     -w.x, -w.y, -w.z, 0.0)
    }

    fn sqew_matrix(a: &Vector3<f32>) -> Matrix3<f32> {
        #[rustfmt_skip]
        Matrix3::new(0.0, -a.z, a.y,
                     a.z, 0.0, -a.x,
                     -a.y, a.x, 0.0)
    }

    fn quaternion_rate(w: Vector3<f32>) -> UnitQuaternion<f32> {
        UnitQuaternion::from_axis_angle(&Unit::new_normalize(w), w.norm())
    }

    pub fn predict(&mut self, dt: f32) {
        // let current_state = self.get_state();
        let control = self.prediction_rx.recv().unwrap();

        //-------- First Order Gyroscope Integrator ---------//
        let prev_attitude = self.state.attitude;
        let prev_angular_rate = self.previous_control.angular_rate;
        let w = (prev_angular_rate + control.angular_rate) / 2.0;

        // let q_dot = Quaternion::from_parts(0.0, w) * prev_attitude * dt;

        let omega_mean = 0.5 * dt * KalmanFilter::omega_matrix(&w);
        let omega_prev = KalmanFilter::omega_matrix(&prev_angular_rate);
        let omega_now = KalmanFilter::omega_matrix(&control.angular_rate);

        #[rustfmt_skip]
        let Theta = Matrix4::identity() + omega_mean + 0.5 * omega_mean * omega_mean;
        let q_dot =
            Theta + 1.0 / 48.0 * dt * dt * (omega_now * omega_prev - omega_prev * omega_now);

        let q_k_1 = Quaternion::from_vector(q_dot * prev_attitude.coords);

        let attitude_p = UnitQuaternion::from_quaternion(q_k_1);

        let skew = KalmanFilter::sqew_matrix(&w);
        let F = Matrix3::identity() - dt * skew + dt * dt / 2.0 * skew * skew;
        let gyroscope_variance = 0.2750;
        // let Q = Matrix3::identity() * gyroscope_variance;
        #[rustfmt_skip]
        let Q = Matrix3::new(0.1463, 0.0, 0.0,
                             0.0, 0.2880, 0.0,
                             0.0, 0.0, 0.3908);
        let impulses_vector = Vector3::new(0.1463, 0.2880, 0.3908);

        self.covariance = F * self.covariance * F.transpose() + Q;

        self.state.attitude = attitude_p;
        self.previous_control = control;
        self.error_state = F * self.error_state + impulses_vector;

        // println!("Error state: {:?}", self.error_state);
        // println!("Covariance:  {:?}", self.covariance);

        {
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
        }

        let state = State {
            // position: position_p,
            // velocity: velocity_p,
            attitude: attitude_p,
        };
    }

    #[rustfmt_skip]
    fn get_attitude_error(&self, reading: Vector3<f32>, field: Vector3<f32>) -> (f32, Unit<Vector3<f32>>) {

        let reading_world = self.state.attitude.inverse_transform_vector(&reading);
        // println!(
        //     "reading_world: {:.02}, {:.02}, {:.02}",
        //     reading_world.data[0], reading_world.data[1], reading_world.data[2]
        // );

        let correction_rotation: UnitQuaternion<f32> =
            UnitQuaternion::rotation_between(&field, &reading_world).unwrap();
        let phi = correction_rotation.angle();
        if phi != 0.0 {
            let u = reading_world.cross(&field);
            (phi, correction_rotation.axis().unwrap())
        } else {
            (0.0, Unit::new_normalize(Vector3::x()))
        }
    }

    fn update_accelerometer(&mut self, acceleration: Vector3<f32>, dt: f32) {
        let gravity = Vector3::new(0.0, 0.0, G_TO_MPSPS);
        let (error_angle, error_axis) = self.get_attitude_error(acceleration, gravity);

        println!(
            "Error angle: {:.02}, Error axis: {:.02}, {:.02}, {:.02}",
            error_angle, error_axis.data[0], error_axis.data[1], error_axis.data[2]
        );

        let alpha = 0.15;
        let attitude_error: UnitQuaternion<f32> =
            UnitQuaternion::from_axis_angle(&error_axis, error_angle * alpha);

        let attitude_correction = self.state.attitude * attitude_error;

        self.state.attitude = attitude_correction;

        // let H: Matrix3<f32> = Matrix3::identity();
        // let accelerometer_variance = 0.25;
        // let R: Matrix3<f32> = Matrix3::from_element(1.0) - Matrix3::identity()
        //     + Matrix3::identity() * accelerometer_variance;
        // let S = H * self.covariance * H.transpose() + R;
        // let K = (self.covariance * H.transpose()).component_div(&S);

        // let correction = K * self.error_state;
        // let attitude_correction =
        //     UnitQuaternion::from_quaternion(Quaternion::from_parts(1.0, 0.5 * correction));

        // let updated_attitude = attitude_correction * self.state.attitude;

        // self.state.attitude = updated_attitude;

        // // println!("covariance: {:?}", self.covariance.data);

        // // self.covariance = (Matrix3::identity() - K * H) * self.covariance
        // //     * (Matrix3::identity() - K * H).transpose()
        // //     + K * R * K.transpose();
    }

    fn update_magnetometer(&mut self, magnetic_reading: Vector3<f32>, dt: f32) {
        // Ann Arbor magnetic field
        // let magnetic_field = Vector3::new(18924.2, -2318.0, 50104.5).normalize();

        // let (error_angle, error_axis) = self.get_attitude_error(magnetic_reading, magnetic_field);
        // let alpha = 0.15;

        // let attitude_error: UnitQuaternion<f32> =
        //     UnitQuaternion::from_axis_angle(&error_axis, error_angle * alpha);

        // let attitude_correction = self.state.attitude * attitude_error;

        // self.state.attitude = attitude_correction;
    }

    fn update_gps(&mut self, dt: f32) {}

    pub fn update(&mut self, dt: f32) {
        let update = self.update_rx.recv().unwrap();

        self.update_accelerometer(update.acceleration, dt);
        if update.magnetic_reading.is_some() {
            self.update_magnetometer(update.magnetic_reading.unwrap(), dt);
        }

        // Compute absolute position and velocity
        if update.gps_information.is_some() {
            self.update_gps(dt);
        }
    }

    pub fn update_motors(&mut self, m1: f32, m2: f32, m3: f32, m4: f32) {}

    // pub fn get_state(&self) -> State {
    //     State {
    //         position: Vector3::new(self.state.data[0], self.state.data[1], self.state.data[2]),
    //         velocity: Vector3::new(self.state.data[3], self.state.data[4], self.state.data[5]),
    //         attitude: UnitQuaternion::from_quaternion(Quaternion::from_vector(Vector4::new(self.state.data[6], self.state.data[7], self.state.data[8], self.state.data[9])));
    //     }
    // }
}
