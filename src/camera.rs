use std::fs::File;
use std::io::{Error, Write};
use cgmath::Vector3;
use indicatif::ProgressIterator;
use rand::Rng;
use Vector3 as Point3;
use crate::hittable::HittableList;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::util::write_pixel;

use Vector3 as Color3;

pub struct Camera {
    pub scene: Scene,
    pixel00_loc: Point3<f64>,
    pixel_delta_u: Vector3<f64>,
    pixel_delta_v: Vector3<f64>,
    max_ray_bounce_depth: i32
}
impl Camera {
    pub fn initialize() -> Self {
        let max_ray_bounce_depth = 50;
        // Scene
        let scene = Scene::new(400, 16.0/9.0);
        // Viewport edge vectors
        let viewport_u = Vector3::new(scene.viewport.width, 0.0, 0.0);
        let viewport_v = Vector3::new(0.0, -scene.viewport.height, 0.0);
        // Deltas between horizontal & vertical vectors
        let pixel_delta_u = viewport_u / scene.image.width as f64;
        let pixel_delta_v = viewport_v / scene.image.height as f64;
        // Location of upper left pixel
        let viewport_upper_left = scene.viewport.camera_center
            - Vector3::new(0.0, 0.0, scene.viewport.focal_length)
            - (viewport_u/2.0)
            - (viewport_v/2.0);
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
        Camera {
            scene,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            max_ray_bounce_depth
        }
    }
    pub fn render(self: &Camera, file: &mut File, hittables: &HittableList) -> Result<(), Error> {
        file.write_all(format!("P3\n{0} {1}\n255\n", self.scene.image.width, self.scene.image.height).as_bytes())?;
        for j in (0..self.scene.image.height).progress() {
            for i in 0..self.scene.image.width {
                let mut pixel_color = Color3::new(0.0, 0.0, 0.0);
                for _ in 0..self.scene.image.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    let ray_color = ray.color(hittables, self.max_ray_bounce_depth);
                    pixel_color += ray_color;
                }
                write_pixel(file, &pixel_color, self.scene.image.samples_per_pixel)?;
            }
        }
        Ok(())
    }
    fn get_ray(self: &Camera, i: i32, j: i32) -> Ray {
        let pixel_center = self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let origin = self.scene.viewport.camera_center;
        let direction = pixel_sample - origin;
        Ray { origin, direction }
    }
    fn pixel_sample_square(self: &Camera) -> Vector3<f64> {
        let mut rng = rand::thread_rng();
        let px = rng.gen_range(-0.5..0.5);
        let py = rng.gen_range(-0.5..0.5);
        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }
}