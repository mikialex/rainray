use super::sphere::Sphere;
use super::material::Material;
use super::ray::RayIntersect;

pub struct Model<T:RayIntersect>{
  pub geometry: T,
  pub material: Material,
}