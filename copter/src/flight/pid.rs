use hardware::sensors::MultiSensorData;
use flight::imu::Attitude;
use config::Config;

pub struct PID {
    roll_kp: f32,
    roll_ki: f32,
    roll_kd: f32,
    pitch_kp: f32,
    pitch_ki: f32,
    pitch_kd: f32,
    yaw_kp: f32,
    yaw_ki: f32,
    yaw_kd: f32,
    integral: MultiSensorData
}

impl PID {
    pub fn new() -> PID {
        let config = Config::new();

        PID {
            roll_kp: config.roll_kp,
            roll_ki: config.roll_ki,
            roll_kd: config.roll_kd,
            pitch_kp: config.pitch_kp,
            pitch_ki: config.pitch_ki,
            pitch_kd: config.pitch_kd,
            yaw_kp: config.yaw_kp,
            yaw_ki: config.yaw_ki,
            yaw_kd: config.yaw_kd,
            integral: MultiSensorData::zeros()
        }
    }

    pub fn correct_attitude(&mut self, dt: f32, current_attitude: Attitude, current_angular_rate: MultiSensorData,
                            desired_attitude: Attitude, mid_level: f32) -> (f32, f32, f32, f32)
    {
        let proportional = current_attitude - desired_attitude;
        let derivative = current_angular_rate;
        self.integral = self.integral + proportional * dt;

        let mut error = MultiSensorData::zeros();
        // x: Roll, y: Pitch PID
        error.x = proportional.x * self.roll_kp + self.integral.x * self.roll_ki + derivative.x * self.roll_kd;
        error.y = proportional.y * self.pitch_kp + self.integral.y * self.pitch_ki + derivative.y * self.pitch_kd;

        let (mut m1, mut m2, mut m3, mut m4) = (mid_level, mid_level, mid_level, mid_level);
        m1 = (0.0 + error.x - error.y) / 2.0;
        m2 = (0.0 + error.x + error.y) / 2.0;
        m3 = (0.0 - error.x + error.y) / 2.0;
        m4 = (0.0 - error.x - error.y) / 2.0;

        m1 += mid_level;
        m2 += mid_level;
        m3 += mid_level;
        m4 += mid_level;

        // z: Yaw PID is added afterwards
        error.z = proportional.z * self.yaw_kp + self.integral.z * self.yaw_ki + derivative.z * self.yaw_kd;

        (m1, m2, m3, m4)
    }
}

//PID STUFF
// pub fn start_pid_loop(&self, config: Config, controller_input: InputStream, sensor_input: Receiver<InertialMeasurement>, debug_pipe : Sender<debug_server::Signal>) {
//     let sensor_poll_time = config.sensor_sample_frequency;
//
//     let motor_1 = self.motors[0];
//     let motor_2 = self.motors[1];
//     let motor_3 = self.motors[2];
//     let motor_4 = self.motors[3];
//
//     if config.motors_on {
//         self.arm(&config);
//     }
//
//     let mut total_time = 0.0;
//     let mut last_total_time = 0.0;
//
//     //PID thread
//     thread::Builder::new().name("PID Loop".to_string()).spawn(move || {
//
//         let mut desired_orientation = MultiSensorData::zeros();
//         let mut integral = MultiSensorData::zeros();
//
//         let mut logger = Logger::new(config.logging);
//
//         let mut count = 0;
//         let mut x_arr = [0.0;40];
//         let mut y_arr = [0.0;40];
//
//         let mut last_sample_time = time::PreciseTime::now();
//         let start = time::PreciseTime::now();
//
//         let mut x_kp = config.x_kp;
//         let mut x_ki = config.x_ki;
//         let mut x_kd = config.x_kd;
//
//         let mut y_kp = config.y_kp;
//         let mut y_ki = config.y_ki;
//         let mut y_kd = config.y_kd;
//
//         let mut z_kp = config.z_kp;
//         let mut z_ki = config.z_ki;
//         let mut z_kd = config.z_kd;
//
//         let mut mid = config.hover_power as f32;
//
//         let mut current_orientation = MultiSensorData::zeros();
//
//         // Clear the sensor channel since later we expect the loop to be running fast enough to
//         // only have one signal in the queue.
//         loop {
//             match sensor_input.try_recv() {
//                 Ok(a) => { },
//                 Err(_) => {
//                     break;
//                 },
//             }
//         }
//
//         let max_motor_speed= config.max_motor_speed as f32;
//         let mut last_yaw_rate = 0.0;
//         let mut yaw_integral = 0.0;
//
//         loop {
//             let mut up_force = 0.0;
//             if mid < max_motor_speed {
//                 up_force += 0.1;
//                 if mid > max_motor_speed - 15.0 {
//                     x_ki = config.x_ki;
//                     y_ki = config.y_ki;
//                 }
//             }
//             // Get all queued updated from controller stream.
//             loop {
//                 match controller_input.try_recv() {
//                     Ok(desired) => {
//                         desired_orientation.x = desired.get_orientation().x;
//                         desired_orientation.y = desired.get_orientation().y;
//                         up_force = desired.get_vertical_velocity();
//                     },
//                     Err(_) => {
//                         break;
//                     }
//                 }
//             }
//
//             mid = mid + up_force;
//
//             let mut orientation_measurements: InertialMeasurement = sensor_input.recv().unwrap_or(InertialMeasurement {
//                 angles: MultiSensorData::zeros(),
//                 rotation_rate: MultiSensorData::zeros(),
//                 altitude: 0.0
//             });
//             loop {
//                 match sensor_input.try_recv() {
//                     Ok(a) => {
//                         orientation_measurements = a;
//                         // Consider making this a hard failure or removing this.
//                         println!("Received duplicate messages...");
//                     },
//                     Err(_) => {
//                         break;
//                     },
//                 }
//             }
//
//             current_orientation = orientation_measurements.angles - MultiSensorData { x: config.angle_offset_x, y: config.angle_offset_y, z: 0.0 };
//             let mut derivative = orientation_measurements.rotation_rate;
//
//             let t = time::PreciseTime::now();
//             let dt: f32 = last_sample_time.to(t).num_microseconds().unwrap() as f32 / 1000000.0;
//             total_time += dt;
//
//             let a = dt / config.derivative_sampling;
// //                if total_time - last_total_time > 1.0 {
// //                    let c = Config::new();
// //                    x_kp = c.x_kp;
// //                    pki = c.x_ki;
// //                    pkd = c.x_kd;
// //
// //                    rkp = c.y_kp;
// //                    rki = c.y_ki;
// //                    rkd = c.y_kd;
// //                    if desired_orientation.y != c.desired_angle {
// //                        integral = MultiSensorData::zeros();
// //                    }
// //                    desired_orientation.y = c.desired_angle;
// //                    mid = c.hover_power as f32;
// //                    last_total_time = total_time;
// //                }
//             last_sample_time = t;
//
//             //Safety check
//             if current_orientation.x.abs() > config.motor_cutoff {
//                 println!("[Motors]: Tilted too far. {:?}", current_orientation);
//                 terminate_all_motors(debug_pipe);
//                 std::process::exit(0);
//             }
//
// //                println!("{}", desired_orientation.x);
//             let mut proportional = desired_orientation - current_orientation;
//
//             integral = integral + proportional * dt;
//
//             let range = 1.0;
//
//             proportional.x *= x_kp;
//             proportional.y *= y_kp;
//
//             if proportional.x.abs() > config.integral_bandwidth {
//                 integral.x = 0.0;
//             }
//             if proportional.y.abs() > config.integral_bandwidth {
//                 integral.y = 0.0;
//             }
//
//             integral.x *= x_ki;
//             integral.y *= y_ki;
//
//             derivative.x *= x_kd;
//             derivative.y *= y_kd;
//
//             let u: MultiSensorData = proportional + integral + derivative;
//             let power = u * range;
//
//             if config.real_time_debugging {
//                 let debug_data = debug_server::DebugInfo {
//                     time: start
//                         .to(time::PreciseTime::now())
//                         .num_microseconds()
//                         .unwrap(),
//                     pidaxes: debug_server::Axis {
//                         power: 0.0,
//                         p: current_orientation.x,
//                         i: current_orientation.y,
//                         d: current_orientation.z,
//                         power_y: power.y,
//                         p_y: proportional.y,
//                         i_y: 0.0,
//                         d_y: derivative.y,
//                     },
//                     power: mid,
//                 };
//
//                 match debug_pipe.send(debug_server::Signal::Log(debug_data)) {
//                     Ok(o) => {},
//                     Err(e) => {
//                         return;
//                     }
//                 }
//
//                 x_arr[count % 40] = current_orientation.x;
//                 y_arr[count % 40] = current_orientation.y;
//                 let (x_avg, x_std) = stat(x_arr);
//                 let (y_avg, y_std) = stat(y_arr);
//                 println!("CA: x: {}, y: {}", format!("{:.*}", 2, current_orientation.x), format!("{:.*}", 2, current_orientation.y));
//                 println!("AA: x: {}, y: {}", format!("{:.*}", 2, x_avg), format!("{:.*}", 2, y_avg));
//                 println!("STD: x: {}, y: {}", format!("{:.*}", 3, x_std), format!("{:.*}", 3, y_std));
//             }
//
//             let x_1 = mid + power.x;
//             let x_2 = mid + power.x;
//             let x_3 = mid - power.x;
//             let x_4 = mid - power.x;
//
//             let y_1 = mid - power.y;
//             let y_2 = mid + power.y;
//             let y_3 = mid + power.y;
//             let y_4 = mid - power.y;
//
//             let mut m_1 = (x_1 + y_1) / 2.0;
//             let mut m_2 = (x_2 + y_2) / 2.0;
//             let mut m_3 = (x_3 + y_3) / 2.0;
//             let mut m_4 = (x_4 + y_4) / 2.0;
//
//             let desired_z_rate = 0.0;
//
//             let current_yaw_rate = orientation_measurements.rotation_rate.z;
//             let yaw_proportional = desired_z_rate - current_yaw_rate;
//             let yaw_derivative = (current_yaw_rate - last_yaw_rate) / dt;
//             yaw_integral = current_yaw_rate * dt;
//
//             let yaw_error = yaw_proportional * z_kp + yaw_integral * z_ki + yaw_derivative * z_kd;
//             println!("Curr Yaw: {}dps", current_yaw_rate);
//
//             m_1 += yaw_error;
//             m_2 -= yaw_error;
//             m_3 += yaw_error;
//             m_4 -= yaw_error;
//
//             if config.motors_on && total_time > 2.0 {
//                 set_power(motor_1, m_1 as u32);
//                 set_power(motor_2, m_2 as u32);
//                 set_power(motor_3, m_3 as u32);
//                 set_power(motor_4, m_4 as u32);
//             }
//
//             if config.logging {
//                 if (count as i32) % config.logging_freq == 0 {
//                     match logger.send(Log {
//                         t: start.to(time::PreciseTime::now())
//                             .num_microseconds()
//                             .unwrap(),
//                         m1: m_1 as u32,
//                         m2: m_2 as u32,
//                         m3: m_3 as u32,
//                         m4: m_4 as u32,
//                         x_ang: current_orientation.x,
//                         y_ang: current_orientation.y,
//                         z_ang: current_orientation.z,
//                         x_p: proportional.x,
//                         x_i: integral.x,
//                         x_d: derivative.x,
//                         y_p: proportional.y,
//                         y_i: integral.y,
//                         y_d: derivative.y
//                     }) {
//                         Ok(o) => {},
//                         Err(e) => {}
//                     }
//                 }
//             }
//
//             count += 1;
//         }
//     });
// }
