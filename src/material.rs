use crate::camera::*;
use crate::frame::*;
use crate::light::*;
use crate::ray::*;
use crate::vec::*;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub diffuse_color: Color,
}

impl Material {
    pub fn brdf(&self, into_ray: &Ray, intersection: &Intersection, random: f64) -> Ray {
        unimplemented!();
    }

    pub fn absorb_rate(&self, into_ray: &Ray, intersection: &Intersection) -> Vec3 {
        Vec3::new(
            self.diffuse_color.r,
            self.diffuse_color.g,
            self.diffuse_color.b,
        )
    }

    pub fn shade(
        &self,
        intersection: &Intersection,
        light: &PointLight,
    ) -> Vec3 {
        let mut light_e = Vec3::new(0.0, 0.0, 0.0);
        let mut light_direction = light.position - intersection.hit_position;
        light_direction.normalize();
        light_e = light.color * Vec3::dot(&intersection.hit_normal, &light_direction).max(0.0);
        
        Vec3::new(
            self.diffuse_color.r * light_e.x,
            self.diffuse_color.g * light_e.y,
            self.diffuse_color.b * light_e.z,
        )
    }
}
