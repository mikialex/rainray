use crate::camera::*;
use crate::frame::*;
use crate::scene::*;
use crate::ray::*;
use crate::material::*;

use std::time::Instant;
use rand::prelude::*;

pub struct Renderer {
    pub frame: Frame,
}

impl Renderer {
    pub fn new() -> Renderer {
        let mut renderer = Renderer {
          frame: Frame::new(500, 500),
        };
        renderer.frame.clear(&Color::new(0.0, 0.0, 0.0));
        renderer
    }

    pub fn path_trace(ray: &Ray, scene: &Scene, camera: &Camera) -> Color {
        let mut min_distance = std::f64::INFINITY;
        let mut material: Option<Material> = None;
        let mut min_distance_intersection: Option<Intersection> = None;

        // get min hit point
        for model in &scene.models {
            if let Some(intersection) = model.geometry.intersect(ray) {
                if intersection.distance < min_distance {
                    min_distance = intersection.distance;
                    material = Some(model.material);
                    min_distance_intersection = Some(intersection);
                }
            }
        }
        if material.is_none() {
            Color::new(0.7,0.7,0.7)
        } else {
            material.unwrap().shade(camera, &min_distance_intersection.unwrap(), &scene.lights)
        }
    }

    pub fn render(&mut self, camera: &Camera, scene: &Scene) -> () {
      
      println!("start render");
      let now = Instant::now();
      let mut rng = rand::thread_rng();

      let frame_data = &mut self.frame.data;
      for (i, row) in frame_data.iter_mut().enumerate() {
          for (j, pixel) in row.iter_mut().enumerate() {
                let x_ratio = i as f64 / self.frame.width as f64;
                let y_ratio = j as f64 / self.frame.height as f64;
                let ray = camera.generate_pixel_ray(x_ratio, y_ratio);
                pixel.r = rng.gen();
                pixel.g = rng.gen();
                pixel.b = rng.gen();

                let color = Renderer::path_trace(&ray, scene, camera);
                pixel.r = color.r;
                pixel.g = color.g;
                pixel.b = color.b;
          }
      }
      
      let duration = now.elapsed();
      println!(
          "rendering used {} milliseconds.",
          duration.as_secs() * 1000 + u64::from(duration.subsec_millis())
      );

    }
}
