use crate::camera::*;
use crate::frame::*;
use crate::material::*;
use crate::math::*;
use crate::ray::*;
use crate::scene::*;

use indicatif::ProgressBar;
use std::time::Instant;

pub struct Renderer {
    super_sample_rate: u64,
    exposure_upper_bound: f64,
    gamma: f64,

    trace_fix_sample_count: u64,
    bounce_time_limit: u64,
}

fn test_intersection_is_visible_to_point(
    _scene: &Scene,
    _intersection: &Intersection,
    _point: &Vec3,
) -> bool {
    
    // let mut light_direction = light.position - intersection.hit_position;
    // light_direction.normalize();
    // true
    false
}

impl Renderer {
    pub fn new() -> Renderer {
        let super_sample_rate = 1;
        Renderer {
            super_sample_rate,
            exposure_upper_bound: 1.0,
            gamma: 2.2,
            bounce_time_limit: 3,
            trace_fix_sample_count: 15,
        }
    }

    pub fn path_trace(&self, ray: &Ray, scene: &Scene, _camera: &Camera) -> Vec3 {
        let mut energy_acc = Vec3::new(0., 0., 0.);

        let mut current_ray = *ray;
        // let mut start_material: Option<Material> = None;

        for _depth in 0..self.bounce_time_limit {
            let mut min_distance = std::f64::INFINITY;
            let mut material: Option<Material> = None;
            let mut min_distance_intersection: Option<Intersection> = None;

            // get min hit point
            for model in &scene.models {
                if let Some(intersection) = model.geometry.intersect(&current_ray) {
                    if intersection.distance < min_distance {
                        min_distance = intersection.distance;
                        material = Some(model.material);
                        min_distance_intersection = Some(intersection);
                    }
                }
            }

            if material.is_none() {
                energy_acc += scene.env.sample(&current_ray);
                break;
            }
            let material = material.unwrap();
            let min_distance_intersection = min_distance_intersection.unwrap();
            let diff_absorb = material.absorb_rate(&current_ray, &min_distance_intersection);

            // collect energy
            for light in &scene.point_lights {
                if test_intersection_is_visible_to_point(
                    &scene,
                    &min_distance_intersection,
                    &light.position,
                ) {
                    energy_acc += material.shade(&min_distance_intersection, &light) * diff_absorb;
                }
            }
            let next = material.next_ray(&current_ray, &min_distance_intersection);
            current_ray.copy_from(&next);
        }

        energy_acc
    }

    pub fn render(&self, camera: &Camera, scene: &Scene, frame: &mut Frame) {
        println!("start render");
        let now = Instant::now();
        let mut render_frame = Frame::new(
            frame.width * self.super_sample_rate,
            frame.height * self.super_sample_rate,
        );

        let x_ratio_unit = 1.0 / render_frame.width as f64;
        let y_ratio_unit = 1.0 / render_frame.width as f64;
        let energy_div = self.trace_fix_sample_count as f64 * self.exposure_upper_bound;

        let progress_bar = ProgressBar::new(100);
        let bar_inv = (render_frame.width as f64 / 100.).ceil() as usize;

        for (i, row) in render_frame.data.iter_mut().enumerate() {
            for (j, pixel) in row.iter_mut().enumerate() {
                let x_ratio = i as f64 * x_ratio_unit;
                let y_ratio = j as f64 * y_ratio_unit;
                let ray = camera.generate_pixel_ray(x_ratio, y_ratio);

                let mut energy_acc = Vec3::new(0., 0., 0.);

                for _sample in 0..self.trace_fix_sample_count {
                    energy_acc += self.path_trace(&ray, scene, camera);
                }
                pixel.r = energy_acc.x / energy_div;
                pixel.g = energy_acc.y / energy_div;
                pixel.b = energy_acc.z / energy_div;
            }
            if i % bar_inv == 0 {
                progress_bar.inc(1);
            }
        }
        progress_bar.finish_and_clear();
        println!("frame data render finished.");

        println!("start super sample down sample and gamma corration");

        let result_data = &mut frame.data;
        let super_sample_rate = self.super_sample_rate as usize;
        for (i, row) in result_data.iter_mut().enumerate() {
            for (j, pixel) in row.iter_mut().enumerate() {
                let super_sample_count =
                    self.super_sample_rate as f64 * self.super_sample_rate as f64;
                let mut r_all = 0.0;
                let mut g_all = 0.0;
                let mut b_all = 0.0;
                for k in 0..super_sample_rate {
                    for l in 0..super_sample_rate {
                        let sample_pix =
                            render_frame.data[i * super_sample_rate + k][j * super_sample_rate + l];
                        let gammared_pix = sample_pix.gamma_rgb(self.gamma);
                        r_all += gammared_pix.r;
                        g_all += gammared_pix.g;
                        b_all += gammared_pix.b;
                    }
                }
                pixel.r = r_all / super_sample_count;
                pixel.g = g_all / super_sample_count;
                pixel.b = b_all / super_sample_count;
            }
        }

        let duration = now.elapsed();
        println!(
            "rendering used {} milliseconds.",
            duration.as_secs() * 1000 + u64::from(duration.subsec_millis())
        );
    }
}
