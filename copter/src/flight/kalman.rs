use std::sync::mpsc::Receiver;
use std::default::Default;
use std::f32;

use na::geometry::{Quaternion, UnitQuaternion};
use na::{Matrix, Matrix3, Matrix3x4, Matrix4, Matrix4x3, MatrixMN, MatrixN, SliceStorage, Unit,
         Vector3, Vector4, VectorN};
use alga::linear::{ProjectiveTransformation, Transformation};
use na::Id;
use na::{U10, U12, U18, U19, U3, U4, U6, U9};
use num::traits::Zero;

use hardware::{PredictionReading, UpdateReading};
use hardware::GPSData;

use logger::ModuleLogger;

const G_TO_MPSPS: f64 = 9.80665;

type TransitionJacobian = MatrixN<f64, U18>;
type CovarianceMatrix = MatrixN<f64, U18>;
type StateJacobian = MatrixMN<f64, U19, U18>;

// Keep track of Location: (lat, lon, altitude), Velocity: (track, climb), Attitude(w, i, j, k), gyro bias, accel bias, magnetic field
#[derive(Debug)]
pub struct State {
    pub position: Vector3<f64>,
    pub velocity: Vector3<f64>,
    pub attitude: UnitQuaternion<f64>,
    pub gyro_bias: Vector3<f64>,
    pub acc_bias: Vector3<f64>,
    pub magnetic_field: Vector3<f64>,
}

impl Default for State {
    fn default() -> State {
        State {
            position: Vector3::zero(),
            velocity: Vector3::zero(),
            attitude: UnitQuaternion::identity(),
            gyro_bias: Vector3::zero(),
            acc_bias: Vector3::zero(),
            magnetic_field: Vector3::new(18924.2, -2318.0, 50104.5).normalize(),
        }
    }
}

pub struct KalmanFilter {
    prediction_rx: Receiver<PredictionReading>,
    update_rx: Receiver<UpdateReading>,
    pub x: State,
    P: CovarianceMatrix,
    u_p: PredictionReading,
    state_jacobian: StateJacobian,
    F: TransitionJacobian,
    Q: CovarianceMatrix,
    H_field: MatrixMN<f64, U3, U19>,
    thrust: f64,
}

impl KalmanFilter {
    pub fn new(
        pred_rx: Receiver<PredictionReading>,
        update_rx: Receiver<UpdateReading>,
    ) -> KalmanFilter {
        let mut X_dx = StateJacobian::zero();
        {
            let mut top_left = X_dx.fixed_slice_mut::<U6, U6>(0, 0);
            top_left.fill_with_identity();
        }
        {
            let mut bottom_right = X_dx.fixed_slice_mut::<U9, U9>(8, 9);
            bottom_right.fill_with_identity();
        }

        let mut F_i: MatrixMN<f64, U18, U12> = MatrixMN::zero();
        F_i.fixed_slice_mut::<U12, U12>(3, 0).fill_with_identity();

        let acc_noise = (0.5 as f64);
        let gyro_noise = (0.1 as f64).to_radians();
        let acc_bias_walk = (0.0 as f64);
        let gyro_bias_walk = (0.0 as f64);
        let mut Q_i: MatrixN<f64, U12> = MatrixN::zero();
        {
            let mut acc_noise_mat = Q_i.fixed_slice_mut::<U3, U3>(0, 0);
            acc_noise_mat.fill_with_identity();
            acc_noise_mat *= acc_noise;
        }
        {
            let mut gyro_noise_mat = Q_i.fixed_slice_mut::<U3, U3>(3, 3);
            gyro_noise_mat.fill_with_identity();
            gyro_noise_mat *= gyro_noise;
        }
        {
            let mut acc_bias_walk_mat = Q_i.fixed_slice_mut::<U3, U3>(6, 6);
            acc_bias_walk_mat.fill_with_identity();
            acc_bias_walk_mat *= acc_bias_walk;
        }
        {
            let mut gyro_bias_walk_mat = Q_i.fixed_slice_mut::<U3, U3>(9, 9);
            gyro_bias_walk_mat.fill_with_identity();
            gyro_bias_walk_mat *= gyro_bias_walk;
        }

        // println!("Q: {:?}", F_i * Q_i * F_i.transpose());

        let F = TransitionJacobian::identity();
        KalmanFilter {
            prediction_rx: pred_rx,
            update_rx: update_rx,
            x: State::default(),
            P: CovarianceMatrix::zero(),
            u_p: PredictionReading::default(),
            state_jacobian: X_dx,
            F: F,
            Q: F_i * Q_i * F_i.transpose(),
            H_field: MatrixMN::zero(),
            thrust: 0.0,
        }
    }

    fn update_state_jacobian(&mut self) {
        let q: &Vector4<f64> = &self.x.attitude.coords;

        let mut q_jacob = self.state_jacobian.fixed_slice_mut::<U4, U3>(6, 6);

        #[rustfmt_skip]
        let update = (0.5) * Matrix4x3::new(q.data[3], -q.data[2], q.data[1],
                                            q.data[2], q.data[3], -q.data[0],
                                            -q.data[1], q.data[0], q.data[3],
                                            -q.data[0], -q.data[1], -q.data[2]);

        q_jacob.copy_from(&update);
    }

    fn omega_matrix(w: &Vector3<f64>) -> Matrix4<f64> {
        #[rustfmt_skip]
        Matrix4::new(0.0, w.z, -w.y, w.x,
                     -w.z, 0.0, w.x, w.y,
                     w.y, -w.x, 0.0, w.z,
                     -w.x, -w.y, -w.z, 0.0)
    }

    fn sqew_matrix(a: &Vector3<f64>) -> Matrix3<f64> {
        #[rustfmt_skip]
        Matrix3::new(0.0, -a.z, a.y,
                     a.z, 0.0, -a.x,
                     -a.y, a.x, 0.0)
    }

    fn quaternion_rate(w: Vector3<f64>) -> UnitQuaternion<f64> {
        UnitQuaternion::from_axis_angle(&Unit::new_normalize(w), w.norm())
    }

    pub fn predict(&mut self, dt: f64) {
        // let current_state = self.get_state();
        let mut u = self.prediction_rx.recv().unwrap();
        u.angular_rate -= self.x.gyro_bias;
        u.acceleration -= self.x.acc_bias;

        //-------- First Order Gyroscope Integrator ---------//
        let prev_attitude = self.x.attitude;
        let prev_angular_rate = self.u_p.angular_rate;
        let next_angular_rate = u.angular_rate;
        let w_avg = (next_angular_rate + prev_angular_rate) / 2.0;

        let omega_mean = 0.5 * dt * KalmanFilter::omega_matrix(&w_avg);
        let omega_prev = KalmanFilter::omega_matrix(&prev_angular_rate);
        let omega_next = KalmanFilter::omega_matrix(&next_angular_rate);

        // 3 axis angular rate integration
        let taylor_order: i32 = 3;
        let mut Theta: Matrix4<f64> = Matrix4::identity();
        let mut fac = 1.0;
        let mut omega = omega_mean;
        for i in 1..taylor_order {
            Theta = Theta + (1.0 / fac) * omega;

            omega *= omega_mean;
            fac *= i as f64;
        }
        Theta += (1.0 / 48.0 * dt * dt) * (omega_next * omega_prev - omega_prev * omega_next);
        let attitude_p =
            UnitQuaternion::from_quaternion(Quaternion::from_vector(Theta * prev_attitude.coords));

        let acceleration_avg = (u.acceleration + self.u_p.acceleration) / 2.0;
        let gravity = Vector3::new(0.0, 0.0, G_TO_MPSPS);
        let acceleration_w = attitude_p.transform_vector(&acceleration_avg) - gravity;

        let velocity_p = self.x.velocity + acceleration_w * dt;
        let position_p = self.x.position + self.x.velocity * dt;

        let skew_w = KalmanFilter::sqew_matrix(&w_avg);
        let skew_a = KalmanFilter::sqew_matrix(&acceleration_avg);
        let R = self.x.attitude.to_rotation_matrix().unwrap();
        let dtI = Matrix3::identity() * dt;

        let dqddt = Matrix3::identity() - dt * skew_w + dt * dt / 2.0 * skew_w * skew_w;
        let dqdgb = Matrix3::identity() * dt + (dt * dt / 2.0) * skew_w
            - (dt * dt * dt / 2.0 / 3.0) * skew_w * skew_w;
        let dpdv = dtI;
        let dvdq = -R * skew_a * dt;
        let dvdab = -R * dt;

        self.F.fixed_slice_mut::<U3, U3>(0, 3).copy_from(&dtI);
        self.F.fixed_slice_mut::<U3, U3>(3, 6).copy_from(&dvdq);
        self.F.fixed_slice_mut::<U3, U3>(3, 9).copy_from(&dvdab);
        self.F.fixed_slice_mut::<U3, U3>(6, 6).copy_from(&dqddt);
        self.F.fixed_slice_mut::<U3, U3>(6, 12).copy_from(&dqdgb);
        self.F.fixed_slice_mut::<U3, U3>(3, 9).copy_from(&(-dtI));

        self.P = self.F * self.P * self.F.transpose() + self.Q;
        self.u_p = u;


        self.x = State {
            position: position_p,
            velocity: velocity_p,
            attitude: attitude_p,
            gyro_bias: self.x.gyro_bias,
            acc_bias: self.x.acc_bias,
            magnetic_field: self.x.magnetic_field,
        };
    }

    fn add_error_state(&mut self, error_state: VectorN<f64, U18>) {
        let dp = Vector3::new(
            error_state.data[0],
            error_state.data[1],
            error_state.data[2],
        );
        let dv = Vector3::new(
            error_state.data[3],
            error_state.data[4],
            error_state.data[5],
        );
        let dt = Vector3::new(
            error_state.data[6] * 0.5,
            error_state.data[7] * 0.5,
            error_state.data[8] * 0.5,
        );
        let dab = Vector3::new(
            error_state.data[9],
            error_state.data[10],
            error_state.data[11],
        );
        let dgb = Vector3::new(
            error_state.data[12],
            error_state.data[13],
            error_state.data[14],
        );
        let dm = Vector3::new(
            error_state.data[15],
            error_state.data[16],
            error_state.data[17],
        );

        self.x.position += dp;
        self.x.velocity += dv;
        self.x.acc_bias += dab;
        self.x.gyro_bias += dgb;
        self.x.magnetic_field += dm;


        let dq = UnitQuaternion::from_quaternion(Quaternion::from_parts(1.0, dt));
        self.x.attitude = self.x.attitude * dq;

        // let mut G = CovarianceMatrix::identity();
        // G.fixed_slice_mut::<U3, U3>(6, 6)
        //     .copy_from(&(Matrix3::identity() - KalmanFilter::sqew_matrix(&dt)));

        // self.P = G * self.P * G.transpose();
    }

    fn correct_field_reading(&mut self, field: Vector3<f64>, measurement: Vector3<f64>) {
        let predicted_measurement = self.x.attitude.transform_vector(&field);
        let z = measurement - predicted_measurement;

        let acc_noise = (2.0 as f64);
        let rot = self.x.attitude.to_rotation_matrix().unwrap();
        let R = Matrix3::identity() * acc_noise;
        let V = rot * R * rot.transpose();

        self.update_state_jacobian();

        let fx = field.data[0];
        let fy = field.data[1];
        let fz = field.data[2] - self.thrust;
        let qx = self.x.attitude.coords.data[0];
        let qy = self.x.attitude.coords.data[1];
        let qz = self.x.attitude.coords.data[2];
        let qw = self.x.attitude.coords.data[3];
        #[rustfmt_skip]
        let H_x: Matrix3x4<f64> =
            Matrix3x4::new(          2.0*(fy*qy+qz*fz), 2.0*(fy*qx-2.0*fx*qy+qw*fz), 2.0*(qx*fz-2.0*fx*qz-fy*qw), 2.0*(qy*fz-fy*qz),
                           2.0*(fx*qy-2.0*fy*qx-qw*fz),           2.0*(fx*qx+qz*fz), 2.0*(fx*qw-2.0*fy*qz+qy*fz), 2.0*(fx*qz-qx*fz),
                           2.0*(fy*qw+fx*qz-2.0*qx*fz), 2.0*(fy*qz-fx*qw-2.0*qy*fz),           2.0*(fx*qx+fy*qy), 2.0*(fy*qx-fx*qy));
        self.H_field.fixed_slice_mut::<U3, U4>(0, 6).copy_from(&H_x);
        let H = self.H_field * self.state_jacobian;

        let S = H * self.P * H.transpose() + V;
        match S.try_inverse() {
            Some(S_inv) => {
                let K = self.P * H.transpose() * S_inv;
                let error_state = K * z;
                self.add_error_state(error_state);

                self.P = (CovarianceMatrix::identity() - K * H) * self.P
                    * (CovarianceMatrix::identity() - K * H).transpose()
                    + K * V * K.transpose();
            }
            None => {}
        }
        println!("P: {:?}", self.P);
    }

    fn update_accelerometer(&mut self, acceleration: Vector3<f64>, dt: f64) {
        let gravity = Vector3::new(0.0, 0.0, G_TO_MPSPS);
        self.correct_field_reading(gravity, acceleration);
    }

    fn update_magnetometer(&mut self, magnetic_reading: Vector3<f64>) {}

    fn update_gps(&mut self, gps_measurement: GPSData) {
        let mut ned_measurement = Vector3::new(
            gps_measurement.latitude,
            gps_measurement.longitude,
            self.x.position.data[2] as f64,
        );
        match gps_measurement.altitude {
            Some(alt) => {
                ned_measurement.data[2] = alt;
            }
            None => {}
        };

        let z = ned_measurement - self.x.position;
    }

    // fn update_magnetometer(&mut self, magnetic_reading: Vector3<f32>, dt: f32) {
    //     // Ann Arbor magnetic field
    //     // let magnetic_field = Vector3::new(18924.2, -2318.0, 50104.5).normalize();

    //     // let (error_angle, error_axis) = self.get_attitude_error(magnetic_reading, magnetic_field);
    //     // let alpha = 0.15;

    //     // let attitude_error: UnitQuaternion<f32> =
    //     //     UnitQuaternion::from_axis_angle(&error_axis, error_angle * alpha);

    //     // let attitude_correction = self.state.attitude * attitude_error;

    //     // self.state.attitude = attitude_correction;
    // }

    // fn update_gps(&mut self, dt: f32) {}

    pub fn update(&mut self, dt: f64) {
        let update = self.update_rx.recv().unwrap();

        self.update_accelerometer(update.acceleration, dt);
        // if update.magnetic_reading.is_some() {
        //     self.update_magnetometer(update.magnetic_reading.unwrap(), dt);
        // }

        // // Compute absolute position and velocity
        // if update.gps_information.is_some() {
        //     self.update_gps(dt);
        // }
    }

    // pub fn update_motors(&mut self, m1: f32, m2: f32, m3: f32, m4: f32) {}

    // // pub fn get_state(&self) -> State {
    // //     State {
    // //         position: Vector3::new(self.state.data[0], self.state.data[1], self.state.data[2]),
    // //         velocity: Vector3::new(self.state.data[3], self.state.data[4], self.state.data[5]),
    // //         attitude: UnitQuaternion::from_quaternion(Quaternion::from_vector(Vector4::new(self.state.data[6], self.state.data[7], self.state.data[8], self.state.data[9])));
    // //     }
    // // }
}
