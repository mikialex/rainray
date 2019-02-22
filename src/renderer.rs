use crate::camera::*;
use crate::frame::*;
use crate::material::*;
use crate::ray::*;
use crate::scene::*;
use crate::vec::*;

use indicatif::ProgressBar;
use rand::prelude::*;
use std::time::Instant;

pub struct Renderer {
    pub frame: Frame,
    pub render_frame: Frame,
    super_sample_rate: u64,
    exposure_upper_bound: f64,
    gamma: f64
}

impl Renderer {
    pub fn new() -> Renderer {
        let width = 500;
        let height = 500;
        let super_sample_rate = 4;

        let mut renderer = Renderer {
            render_frame: Frame::new(
                width * super_sample_rate,
                height * super_sample_rate,
            ),
            frame: Frame::new(width, height),
            super_sample_rate,
            exposure_upper_bound: 1.0,
            gamma: 2.2
        };
        renderer.frame.clear(&Color::new(0.0, 0.0, 0.0));
        renderer
    }

    pub fn path_trace(ray: &Ray, scene: &Scene, camera: &Camera) -> Vec3 {
        let mut min_distance = std::f64::INFINITY;
        let mut material: Option<Material> = None;
        let mut min_distance_intersection: Option<Intersection> = None;

        // get min hit point
        for model in &scene.models {
            if let Some(intersection) = model.geometry.intersect(ray) {
                if intersection.distance < min_distance {
                    min_distance = intersection.distance;
                    material = Some(model.material);
                    min_distance_intersection = Some(intersection);
                }
            }
        }
        if material.is_none() {
            Vec3::new(0., 0., 0.)
        } else {
            material
                .unwrap()
                .shade(camera, &min_distance_intersection.unwrap(), &scene.lights)
        }
    }

    pub fn render(&mut self, camera: &Camera, scene: &Scene) -> () {
        println!("start render");
        let now = Instant::now();
        let mut rng = rand::thread_rng();

        let frame_data = &mut self.render_frame.data;
        let x_ratio_unit = 1.0 / self.render_frame.width as f64;
        let y_ratio_unit = 1.0 / self.render_frame.width as f64;

        let bar = ProgressBar::new(100);
        let bar_inv = (self.render_frame.width as f64 / 100.).ceil() as usize;

        for (i, row) in frame_data.iter_mut().enumerate() {
            for (j, pixel) in row.iter_mut().enumerate() {
                let x_ratio = i as f64 * x_ratio_unit;
                let y_ratio = j as f64 * y_ratio_unit;
                let ray = camera.generate_pixel_ray(x_ratio, y_ratio);
                // pixel.r = rng.gen();
                // pixel.g = rng.gen();
                // pixel.b = rng.gen();

                let energy = Renderer::path_trace(&ray, scene, camera);
                pixel.r = energy.x / self.exposure_upper_bound;
                pixel.g = energy.y / self.exposure_upper_bound;
                pixel.b = energy.z / self.exposure_upper_bound;
            }
            if i % bar_inv == 0 {
                bar.inc(1);
            }
        }
        bar.finish_and_clear();
        println!("frame data render finished.");

        println!("start super sample down sample and gamma corration");

        let result_data = &mut self.frame.data;
        let super_sample_rate = self.super_sample_rate as usize;
        for (i, row) in result_data.iter_mut().enumerate() {
            for (j, pixel) in row.iter_mut().enumerate() {
                let super_sample_count = self.super_sample_rate as f64 * self.super_sample_rate as f64;
                let mut r_all = 0.0;
                let mut g_all = 0.0;
                let mut b_all = 0.0;
                for k in 0..super_sample_rate{
                    for l in 0..super_sample_rate {
                        let sample_pix = frame_data[i * super_sample_rate + k][j * super_sample_rate + l];
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
