extern crate alga;
extern crate json;
extern crate kiss3d;
extern crate nalgebra as na;
extern crate num;
extern crate typenum;

use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::net::UdpSocket;
use std::f32;
use std::path::Path;

use na::{Matrix3, Matrix3x4, Point3, Quaternion, Unit, UnitQuaternion, Vector3, Vector4};
use num::traits::Zero;
use alga::linear::{ProjectiveTransformation, Transformation};

use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::camera::{ArcBall, FirstPerson};
use kiss3d::resource::MeshManager;

use std::f32::consts::PI;

struct State {
    position: Point3<f32>,
    attitude: UnitQuaternion<f32>,
    motor_powers: Vector4<f32>,
}

fn main() {
    let mut window = Window::new("Quadcopter visualizer");

    window.set_light(Light::StickToCamera);

    let mut state = State {
        position: Point3::new(0.0, 0.0, 0.0),
        attitude: UnitQuaternion::identity(),
        motor_powers: Vector4::zero(),
    };

    let r45 = UnitQuaternion::from_axis_angle(
        &Unit::new_normalize(Vector3::new(0.0, 0.0, 1.0)),
        45.0_f32.to_radians(),
    );
    let r90 = r45 * r45;
    let mut arm = r45.to_rotation_matrix() * Vector3::new(0.275, 0.0, 0.0);
    let mut motor_points: Matrix3x4<f32> = Matrix3x4::zero();
    for i in 0..4 {
        motor_points.set_column(i, &arm);
        arm = r90.to_rotation_matrix() * arm;
    }


    let camera_location = Point3::new(-5.0, 3.0, -3.0);
    let mut camera = ArcBall::new(camera_location, Point3::new(0.0, 0.0, 0.0));
    let camera_rotation =
        UnitQuaternion::from_axis_angle(&Unit::new_normalize(Vector3::x()), PI / 2.0);

    let red = Point3::new(1.0, 0.0, 0.0);
    let green = Point3::new(0.0, 1.0, 0.0);
    let blue = Point3::new(0.0, 0.0, 1.0);
    let yellow = Point3::new(0.0, 1.0, 1.0);

    let obj_path = Path::new("models/quadcopter.obj");
    let mtl_path = Path::new("models/quadcopter.mtl");
    let mut quad_model = window.add_obj(&obj_path, &mtl_path, Vector3::new(1.0, 1.0, 1.0)); //window.add_obj(&obj_path, &mtl_path, Vector3::new(0.01, 0.01, 0.01));

    let (state_tx, state_rx): (Sender<State>, Receiver<State>) = channel();

    thread::spawn(move || {
        match (UdpSocket::bind("0.0.0.0:9898")) {
            Ok(sock) => {
                println!("Bound to socket");
                loop {
                    // let numFloats = 6;
                    // let numBytes = numFloats * 4;
                    let mut buf: [u8; 1000] = [0; 1000];
                    let (amt, src) = sock.recv_from(&mut buf).expect("failed to read from sock");
                    let result = String::from_utf8(Vec::from(&buf[0..amt])).unwrap();
                    // println!("{}", result);
                    let j = json::parse(result.as_str()).unwrap();

                    // let new_position: Vector3<f32> = Vector3::new(j["position"][0].as_f32().unwrap(), j["position"][1].as_f32().unwrap(), j["position"][2].as_f32().unwrap());
                    let new_attitude: UnitQuaternion<f32> =
                        UnitQuaternion::from_quaternion(Quaternion::new(
                            j["attitude"][3].as_f32().unwrap(),
                            j["attitude"][0].as_f32().unwrap(),
                            j["attitude"][1].as_f32().unwrap(),
                            j["attitude"][2].as_f32().unwrap(),
                        ));
                    let new_position = Point3::new(
                        j["position"][0].as_f32().unwrap(),
                        j["position"][1].as_f32().unwrap(),
                        j["position"][2].as_f32().unwrap(),
                    );
                    let new_powers = Vector4::new(
                        j["power"][0].as_f32().unwrap(),
                        j["power"][1].as_f32().unwrap(),
                        j["power"][2].as_f32().unwrap(),
                        j["power"][3].as_f32().unwrap(),
                    );
                    let offset = Vector4::from_element(1000.0);
                    let new_state = State {
                        position: new_position,
                        attitude: new_attitude,
                        motor_powers: (new_powers - offset) / 2000.0,
                    };
                    state_tx.send(new_state);
                }
            }
            Err(err) => panic!("Could not bind: {}", err),
        }
        // let port = 9898;
        // let listener = TcpListener::bind("127.0.0.1:9898").unwrap();
        // listener.accept();
        // println!("Listening on port {}", 9898);
        // loop {
        //     let new_position: Vector3<f32> = Vector3::new(1.0, 0.0, 0.0);
        //     let new_attitude: UnitQuaternion<f32> = UnitQuaternion::identity();
        //     location_tx.send((new_position, new_attitude));
        //     thread::sleep_ms(50);
        // }
    });

    while window.render_with_camera(&mut camera) {
        match state_rx.try_recv() {
            Ok(s) => {
                state = s;
            }
            Err(_) => {}
        }



        let rot = state.attitude.to_rotation_matrix();
        let transformed_motor_positions = (rot * motor_points);
        let mut thrusts: Matrix3x4<f32> = Matrix3x4::zero();
        thrusts.set_row(2, &state.motor_powers.transpose());
        thrusts = rot * thrusts;
        thrusts = transformed_motor_positions - thrusts;

        for i in 0..4 {
            let motor_location = transformed_motor_positions.column(i);
            let pt1: Point3<f32> = change_axes(Point3::new(
                motor_location[0],
                motor_location[1],
                motor_location[2],
            ));
            let thrust = thrusts.column(i);
            let pt2: Point3<f32> = change_axes(Point3::new(thrust[0], thrust[1], thrust[2]));
            window.draw_line(&pt1, &pt2, &yellow);
        }


        println!("{:?}", state.attitude);
        quad_model.set_local_rotation(state.attitude);


        // camera.look_at(camera_location, position);

        let x = change_axes_vector(state.attitude.transform_vector(&Vector3::x()));
        let y = change_axes_vector(state.attitude.transform_vector(&Vector3::y()));
        let z = change_axes_vector(state.attitude.transform_vector(&Vector3::z()));
        // let x = Vector3::x();
        // let y = Vector3::y();
        // let z = Vector3::z();

        draw_grid(&mut window);


        let position_changed = change_axes(state.position);
        window.draw_line(&position_changed, &(position_changed + x), &red);
        window.draw_line(&position_changed, &(position_changed + y), &green);
        window.draw_line(&position_changed, &(position_changed + z), &blue);

        thread::sleep_ms(5);
    }
}

fn change_axes(pt: Point3<f32>) -> Point3<f32> {
    let rotation = UnitQuaternion::from_axis_angle(&Unit::new_normalize(Vector3::x()), PI / 2.0);
    rotation.transform_point(&pt)
}

fn change_axes_vector(vec: Vector3<f32>) -> Vector3<f32> {
    let rotation = UnitQuaternion::from_axis_angle(&Unit::new_normalize(Vector3::x()), PI / 2.0);
    rotation.transform_vector(&vec)
}

fn draw_grid(window: &mut Window) {
    let white = Point3::new(0.2, 0.2, 0.2);

    for i in 0..21 {
        let x = (i - 10) as f32;
        window.draw_line(
            &Point3::new(x, 0.0, -10.0),
            &Point3::new(x, 0.0, 10.0),
            &white,
        );
        window.draw_line(
            &Point3::new(-10.0, 0.0, x),
            &Point3::new(10.0, 0.0, x),
            &white,
        );
    }
}
