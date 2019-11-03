use crate::math::vec3::Vec3;
use crate::ray::*;

pub struct SolidEnvironment {
    pub intensity: Vec3,
}

pub trait Environment {
    fn sample(&self, ray: &Ray) -> Vec3;
}

impl Environment for SolidEnvironment {
    fn sample(&self, _ray: &Ray) -> Vec3 {
        self.intensity
    }
}
