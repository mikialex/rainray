use crate::vec::Vec3;
use crate::ray::*;

pub struct Sphere{
  pub center: Vec3,
  pub radius: f32,
}

impl RayIntersect for Sphere {
  fn ray_intersect(&self, ray: &Ray) -> bool{
    false
  }
}