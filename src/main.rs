mod camera;
mod frame;
mod light;
mod material;
mod model;
mod ray;
mod renderer;
mod scene;
mod geometry;
mod vec;
mod environment;

use crate::environment::*;
use crate::camera::*;
use crate::frame::*;
use crate::light::*;
use crate::renderer::*;
use crate::scene::*;
use crate::geometry::*;
use crate::vec::*;

use std::env;

fn main() {
    let renderer = Renderer::new();
    let camera = Camera::new();
    let mut frame = Frame::new(500, 500);
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
                diffuse_color: Color::new(0.8, 0.8, 0.8),
            },
        }],
        point_lights: vec![PointLight {
            position: Vec3 {
                x: -200.,
                y: 0.,
                z: 200.,
            },
            color: Vec3::new(1.0, 1.0, 1.0),
        }],
        env: Box::new(SolidEnvironment{
            color: Color::new(0.8, 0.8, 0.8),
        })
    };

    let mut current_path = env::current_dir().unwrap();
    println!("working dir {}", current_path.display());
    renderer.render(&camera, &scene, &mut frame);
    current_path.push("result.png");
    let file_target_path = current_path.into_os_string().into_string().unwrap();

    println!("writing file to path: {}", file_target_path);
    frame.write_to_file(&file_target_path);
}
