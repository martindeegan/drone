extern crate alga;
extern crate kiss3d;
extern crate nalgebra as na;
extern crate num;
extern crate typenum;

use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::net::{TcpListener, TcpStream};

use na::{Point3, UnitQuaternion, Vector3};
use num::traits::Zero;
use alga::linear::{ProjectiveTransformation, Transformation};

use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::camera::FirstPerson;

fn main() {
    let mut window = Window::new("Kiss3d: cube");
    let mut c = window.add_cube(1.0, 1.0, 1.0);

    c.set_color(1.0, 0.0, 0.0);
    window.set_light(Light::StickToCamera);
    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);

    let mut attitude: UnitQuaternion<f32> = UnitQuaternion::identity();
    let mut position: Point3<f32> = Point3::new(0.0, 0.0, 0.0);

    let camera_location = Point3::new(10.0, 10.0, 5.0);
    let mut camera = FirstPerson::new(camera_location, position);

    let red = Point3::new(1.0, 0.0, 0.0);
    let green = Point3::new(0.0, 1.0, 0.0);
    let blue = Point3::new(0.0, 0.0, 1.0);

    let (location_tx, location_rx): (
        Sender<(Vector3<f32>, UnitQuaternion<f32>)>,
        Receiver<(Vector3<f32>, UnitQuaternion<f32>)>,
    ) = channel();
    thread::spawn(move || {
        let port = 9898;
        let listener = TcpListener::bind("127.0.0.1:9898").unwrap();
        listener.accept();
        println!("Listening on port {}", 9898);
        loop {
            let new_position: Vector3<f32> = Vector3::new(1.0, 0.0, 0.0);
            let new_attitude: UnitQuaternion<f32> = UnitQuaternion::identity();
            location_tx.send((new_position, new_attitude));
            thread::sleep_ms(50);
        }
    });

    while window.render_with_camera(&mut camera) {
        match location_rx.try_recv() {
            Ok((pos, att)) => {
                let x = attitude.transform_vector(&Vector3::x());
                let y = attitude.transform_vector(&Vector3::y());
                let z = attitude.transform_vector(&Vector3::z());

                window.draw_line(&position, &Point3::from_coordinates(x), &red);
                window.draw_line(&position, &Point3::from_coordinates(y), &green);
                window.draw_line(&position, &Point3::from_coordinates(z), &blue);
                camera.look_at(camera_location, position);
            }
            Err(_) => {}
        }
    }
}
