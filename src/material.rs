
use crate::frame::*;
use crate::light::*;
use crate::vec::*;
use crate::ray::*;
use crate::camera::*;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub diffuse_color: Color
}

impl Material {
    pub fn shade(&self, camera: &Camera, intersection: &Intersection, light_list: &Vec<PointLight>) -> Vec3 {
        let mut light_e = Vec3::new(0.0,0.0,0.0);
        for light in light_list {
            let mut light_direction = light.position - intersection.hit_position;
            let mut hit_to_eye_direction = camera.eye_position - intersection.hit_position;
            let mut center_direction = light_direction + hit_to_eye_direction;
            light_direction.normalize();
            hit_to_eye_direction.normalize();
            center_direction.normalize();

            // diffuse
            light_e += light.color * Vec3::dot(&intersection.hit_normal, &light_direction).max(0.0);
        }
        Vec3::new(
            self.diffuse_color.r * light_e.x,
            self.diffuse_color.g * light_e.y,
            self.diffuse_color.b * light_e.z,
        )
    }
}