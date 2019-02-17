use crate::vec::Vec3;
use crate::ray::*;

pub struct Sphere{
  pub center: Vec3,
  pub radius: f32,
}

impl Intersecterable for Sphere {
  fn intersect(&self, ray: &Ray) -> Option<f64>{
    false
  }
}