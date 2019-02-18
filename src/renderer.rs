use crate::camera::*;
use crate::frame::*;
use crate::scene::*;

pub struct Renderer {
  pub frame: Frame,
}

impl Renderer {
  pub fn new() -> Renderer {
    Renderer {
      frame: Frame::new(100, 100),
    }
  }

  pub fn render(&mut self, camera: &Camera, scene: &Scene) -> () {
    let frame_data = &mut self.frame.data;
    for (i, row) in frame_data.iter_mut().enumerate() {
      for (j, pixel) in row.iter_mut().enumerate() {
        let x_ratio = i as f32 / self.frame.width as f32;
        let y_ratio = j as f32 / self.frame.height as f32;
        let ray = camera.generate_pixel_ray(x_ratio, y_ratio);
        pixel.r = 0.5;
      }
    }

  }
}
