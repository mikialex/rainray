mod camera;
mod frame;
mod material;
mod model;
mod ray;
mod sphere;
mod vec;
mod renderer;
mod scene;

use crate::camera::*;
use crate::renderer::*;
use crate::sphere::*;
use crate::scene::*;

fn main() {
    let mut renderer = Renderer::new();
    let camera = Camera::new();
    let scene = Scene {
        models: vec![model::Model {
        geometry: Box::new(Sphere {
            center: vec::Vec3 {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            radius: 1.,
        }),
        material: material::Material {
            color: vec::Vec3 {
                x: 0.,
                y: 0.,
                z: 0.,
            },
        },
    }]
    };


    println!("start render");
    renderer.render(&camera, &scene);
    let file_target_path = "/Users/mikialex/Desktop/rainray.png";
    println!("writing file to path: {}", file_target_path);
    renderer.frame.write_to_file(file_target_path);
}
