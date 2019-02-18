use crate::vec::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn point_at_direction(&self, distance: f32) -> Vec3 {
        self.origin + self.direction * distance
    }

    #[allow(dead_code)]
    pub fn new() -> Ray {
        Ray {
            origin: Vec3 {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            direction: Vec3 {
                x: 0.,
                y: 0.,
                z: 1.,
            },
        }
    }

    pub fn from_point_to_point(origin: Vec3, target: Vec3) -> Ray {
        Ray {
            origin: origin.clone(),
            direction: target - origin,
        }
    }
}

pub trait Intersecterable {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}

pub struct Intersection {
    pub distance: f32,
    pub hit_position: Vec3,
    pub hit_normal: Vec3,
}
