mod camera;
mod frame;
mod material;
mod model;
mod ray;
mod sphere;
mod vec;
mod renderer;
mod scene;
mod light;

use crate::camera::*;
use crate::renderer::*;
use crate::sphere::*;
use crate::scene::*;
use crate::frame::*;
use crate::vec::*;
use crate::light::*;

use std::env;

fn main() {

    let mut renderer = Renderer::new();
    let camera = Camera::new();
    let scene = Scene {
        models: vec![model::Model {
        geometry: Box::new(Sphere {
            center: Vec3 {
                x: 0.,
                y: 0.,
                z: -5.,
            },
            radius: 2.,
        }),
        material: material::Material {
            diffuse_color: Color::new(0.8,0.8,0.8),
        },
    }],
    lights: vec![Box::new(PointLight {
            position: Vec3 {
                x: 0.,
                y: 0.,
                z: -5.,
            },
            color: Color::new(0.8,0.8,0.8),
        })
    ],
    };

    let mut current_path = env::current_dir().unwrap();
    println!("working dir {}", current_path.display());
    renderer.render(&camera, &scene);
    current_path.push("result.png");
    let file_target_path = current_path.into_os_string()
    .into_string().unwrap();
    println!("writing file to path: {}", file_target_path);
    renderer.frame.write_to_file(&file_target_path);
}
