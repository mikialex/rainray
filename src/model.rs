use crate::sphere::Sphere;
use crate::ray::*;
use crate::material::Material;

pub struct Model<T:RayIntersect>{
    pub geometry: T,
    pub material: Material,
}

impl RayTrace for Model<Sphere>{
    fn next_ray(&self, ray: &Ray, record: &mut RayHitRecord) -> bool{
        false
    }
}