use crate::camera::*;
use crate::frame::*;
use crate::scene::*;

use std::time::Instant;
use rand::prelude::*;

pub struct Renderer {
  pub frame: Frame,
}

impl Renderer {
  pub fn new() -> Renderer {
    Renderer {
      frame: Frame::new(100, 100),
    }
  }

  pub fn render(&mut self, camera: &Camera, _scene: &Scene) -> () {
    
    println!("start render");
    let now = Instant::now();
    let mut rng = rand::thread_rng();

    let frame_data = &mut self.frame.data;
    for (i, row) in frame_data.iter_mut().enumerate() {
      for (j, pixel) in row.iter_mut().enumerate() {
        let x_ratio = i as f32 / self.frame.width as f32;
        let y_ratio = j as f32 / self.frame.height as f32;
        let _ray = camera.generate_pixel_ray(x_ratio, y_ratio);
        pixel.r = rng.gen();
        pixel.g = rng.gen();
        pixel.b = rng.gen();
      }
    }
    
    let duration = now.elapsed();
    println!(
      "rendering used {} milliseconds.",
      duration.as_secs() * 1000 + u64::from(duration.subsec_millis())
    );

  }
}
