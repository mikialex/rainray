use crate::frame::*;
use crate::light::*;
use crate::math::*;
use crate::ray::*;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub diffuse_color: Color,
}

impl Material {
    pub fn next_ray(&self, into_ray: &Ray, intersection: &Intersection) -> Ray {
        // Ray::new(
        //     intersection.hit_position,
        //     cosine_sample_hemisphere(&intersection.hit_normal),
        // )
        // Ray::new(
        //     intersection.hit_position,
        //     Vec3::reflect(&intersection.hit_normal, &into_ray.direction),
        // )

        Ray::from_point_to_point(
            &intersection.hit_position,
            &(intersection.hit_position + intersection.hit_normal + rand_point_in_unit_sphere()),
        )
    }

    pub fn absorb_rate(&self, _into_ray: &Ray, _intersection: &Intersection) -> Vec3 {
        Vec3::new(
            self.diffuse_color.r,
            self.diffuse_color.g,
            self.diffuse_color.b,
        )
    }

    pub fn shade(&self, intersection: &Intersection, light: &PointLight) -> Vec3 {
        let mut light_direction = light.position - intersection.hit_position;
        light_direction.normalize();
        let light_e = light.color * Vec3::dot(&intersection.hit_normal, &light_direction).max(0.0);
        // let light_e = light.color;

        Vec3::new(
            self.diffuse_color.r * light_e.x,
            self.diffuse_color.g * light_e.y,
            self.diffuse_color.b * light_e.z,
        )
    }
}
