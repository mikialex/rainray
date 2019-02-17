use crate::vec::Vec3;
use crate::material::*;

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

    pub fn from_point_to_point(origin: Vec3, target: Vec3)-> Ray{
        Ray{
            origin: origin.clone(),
            direction: target - origin
        }
    }
}


pub trait Intersecterable {
    fn intersect(&self, ray: Ray) -> Option<f64>;
    fn material(&self) -> Material;
    fn normal(&self, hit_point: Vec3) -> Vec3;
}


pub trait RayTrace {
    fn next_ray(&self, ray: &Ray, record: &mut RayHitRecord) -> bool;
}

pub struct RayHitRecord {
    hit_point: Vec3,
    hit_normal: Vec3,
    material: Material,
}