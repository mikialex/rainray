use crate::vec::Vec3;
use crate::frame::Frame;

pub struct Camera {
    frame: Frame,
    eye_position: Vec3,
    film_width: f32,
    film_height: f32,
    film_distance: f32,
    look_at_direction: Vec3
}

impl Camera {
  pub fn new()->Camera{
    Camera{
      frame: Frame::new(200, 100),
      eye_position: Vec3::new(0.,0.,0.),
      film_width: 2.,
      film_height: 1.,
      film_distance: 1.,
      look_at_direction:Vec3::new(0.,0.,-1.)
    }
  }
}