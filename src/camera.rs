use crate::ray::*;
use crate::vec::Vec3;

pub struct Camera {
    pub eye_position: Vec3,
    pub up_direction: Vec3,

    pub film_width: f64,
    pub film_height: f64,
    pub film_center: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            eye_position: Vec3::new(0., 0., 0.),
            up_direction: Vec3::new(0., 1., 0.),
            film_width: 2.,
            film_height: 2.,
            film_center: Vec3::new(0., 0., -1.),
        }
    }

    pub fn get_pixel_world_position(&self, x_ratio: f64, y_ratio: f64) -> Vec3 {
        let clamped_x_ratio = x_ratio.max(0.0).min(1.0) - 0.5;
        let clamped_y_ratio = y_ratio.max(0.0).min(1.0) - 0.5;
        let center_direction = self.film_center - self.eye_position;
        let x_axis = *Vec3::cross(&self.up_direction, &center_direction).normalize();
        let y_axis = *Vec3::cross(&x_axis, &center_direction).normalize();
        let mut film_position = self.film_center + x_axis * self.film_width * clamped_x_ratio;
        film_position = film_position + y_axis * self.film_height * clamped_y_ratio;
        return film_position
    }

    pub fn generate_pixel_ray(&self, x_ratio: f64, y_ratio: f64) -> Ray {
        let mut ray = Ray::from_point_to_point(
            self.eye_position, 
            self.get_pixel_world_position(x_ratio, y_ratio)
        );
        ray.direction.normalize();
        ray
    }
}
