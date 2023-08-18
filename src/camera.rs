use std::fs::File;
use std::io::{Error, Write};
use cgmath::Vector3;
use indicatif::ProgressIterator;
use rand::Rng;
use Vector3 as Point3;
use crate::hittable::HittableList;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::util::{unit_vector, vector_length, write_pixel};

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
        let look_from = Point3::new(-2.0, 2.0, 1.0);
        let look_at = Point3::new(0.0, 0.0, -1.0);
        let v_up = Vector3::new(0.0, 1.0, 0.0);

        let max_ray_bounce_depth = 50;
        // Scene
        let scene = Scene::new(400, 16.0/9.0);

        let camera_center = look_from;
        let focal_length = vector_length(look_from - look_at);
        let vfov: f64 = 90.0;
        let theta = vfov.to_radians();
        let h = (theta/2.0).tan();

        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * (scene.image.width as f64 / scene.image.height as f64);

        let w = unit_vector(look_from - look_at);
        let u = v_up.cross(w);
        let v = w.cross(u);

        // Viewport edge vectors
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;
        // Deltas between horizontal & vertical vectors
        let pixel_delta_u = viewport_u / scene.image.width as f64;
        let pixel_delta_v = viewport_v / scene.image.height as f64;
        // Location of upper left pixel
        let viewport_upper_left = camera_center
            - (focal_length * w)
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