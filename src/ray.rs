use super::vec::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

pub fn point_at_direction(ray: Ray, distance:f32) -> Vec3 {
    ray.origin + ray.direction * distance
}

pub trait RayIntersect {
    fn ray_intersect(&self, ray: &Ray) -> bool;
}