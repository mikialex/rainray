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
    // let mut frame_data = &self.frame.data;
    // for row in frame_data {
    //   for pixel in row {
    //     pixel.r = 1.0;
    //   }
    // }

  }
}
