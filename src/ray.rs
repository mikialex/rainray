use crate::vec::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn point_at_direction(self, distance:f32) -> Vec3 {
        self.origin + self.direction * distance
    }

    pub fn new() -> Ray{
        Ray{
            origin: Vec3 {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            direction: Vec3 {
                x: 0.,
                y: 0.,
                z: 1.,
            }
        }
    }
}


pub trait RayIntersect {
    fn ray_intersect(&self, ray: &Ray) -> bool;
}


pub trait RayTrace {
    fn next_ray(&self, ray: &Ray) -> Ray;
}