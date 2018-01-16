extern crate alga;
extern crate json;
extern crate kiss3d;
extern crate nalgebra as na;
extern crate num;
extern crate typenum;

use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::net::{TcpListener, TcpStream, UdpSocket};

use na::{Point3, Quaternion, Unit, UnitQuaternion, Vector3};
use num::traits::Zero;
use alga::linear::{ProjectiveTransformation, Transformation};

use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::camera::{ArcBall, FirstPerson};

use std::f32::consts::PI;

fn main() {
    let mut window = Window::new("Quadcopter visualizer");

    window.set_light(Light::StickToCamera);

    let mut attitude: UnitQuaternion<f32> = UnitQuaternion::identity();
    let mut position: Point3<f32> = Point3::new(0.0, 0.0, 0.0);

    let camera_location = Point3::new(-10.0, 7.0, -7.0);
    let mut camera = ArcBall::new(camera_location, position);
    let camera_rotation =
        UnitQuaternion::from_axis_angle(&Unit::new_normalize(Vector3::x()), PI / 2.0);

    let red = Point3::new(1.0, 0.0, 0.0);
    let green = Point3::new(0.0, 1.0, 0.0);
    let blue = Point3::new(0.0, 0.0, 1.0);

    let (location_tx, location_rx): (
        Sender<(Vector3<f32>, UnitQuaternion<f32>)>,
        Receiver<(Vector3<f32>, UnitQuaternion<f32>)>,
    ) = channel();

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
                    let j = json::parse(result.as_str()).expect("failed to read json");
                    // println!("{:?}", j);
                    // let new_position: Vector3<f32> = Vector3::new(j["position"][0].as_f32().unwrap(), j["position"][1].as_f32().unwrap(), j["position"][2].as_f32().unwrap());
                    let new_attitude: UnitQuaternion<f32> =
                        UnitQuaternion::from_quaternion(Quaternion::new(
                            j["attitude"][3].as_f32().unwrap(),
                            j["attitude"][0].as_f32().unwrap(),
                            j["attitude"][1].as_f32().unwrap(),
                            j["attitude"][2].as_f32().unwrap(),
                        ));
                    let new_position: Vector3<f32> = Vector3::new(0.0, 0.0, 0.0);
                    location_tx.send((new_position, new_attitude));
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
        match location_rx.try_recv() {
            Ok((pos, att)) => {
                attitude = att;
            }
            Err(_) => {}
        }


        // camera.look_at(camera_location, position);

        let x = change_axes_vector(attitude.transform_vector(&Vector3::x()));
        let y = change_axes_vector(attitude.transform_vector(&Vector3::y()));
        let z = change_axes_vector(attitude.transform_vector(&Vector3::z()));
        // let x = Vector3::x();
        // let y = Vector3::y();
        // let z = Vector3::z();

        draw_grid(&mut window);


        let position_changed = change_axes(position);
        window.draw_line(&position_changed, &(position_changed + x), &red);
        window.draw_line(&position_changed, &(position_changed + y), &green);
        window.draw_line(&position_changed, &(position_changed + z), &blue);

        // thread::sleep_ms(5);
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
