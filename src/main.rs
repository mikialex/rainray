mod frame;
mod material;
mod camera;
use crate::camera::*;
mod model;
mod ray;
mod sphere;
mod vec;



fn main() {
    let file_target_path = "/Users/mikialex/Desktop/rainray.png";

    let camera = Camera::new();

    let sphere = model::Model {
        geometry: sphere::Sphere {
            center: vec::Vec3 {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            radius: 1.,
        },
        material: material::Material {
            color: vec::Vec3 {
                x: 0.,
                y: 0.,
                z: 0.,
            },
        },
    };

    let mut frame = frame::Frame::new(100, 100);

    let mut color = vec::Vec3 {
        x: 1.,
        y: 2.,
        z: 3.,
    };
    frame.set_pixel(&color, 2, 2);
    println!("frame pixel count: {}", frame.pixel_count());

    color.x = 2.;
    // println!("{}", point);
}
