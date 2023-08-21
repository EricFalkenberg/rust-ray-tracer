use std::fs::File;
use std::io::{Error, Write};
use std::sync::Mutex;
use cgmath::Vector3;
use indicatif::ProgressBar;
use rand::Rng;
use rayon::prelude::*;
use Vector3 as Point3;
use crate::hittable::HittableList;
use crate::ray::Ray;
use crate::util::{random_in_unit_disc, unit_vector, write_pixel};

use Vector3 as Color3;
use crate::image::Image;
use crate::viewport::Viewport;

pub struct Camera {
    image: Image,
    camera_center: Point3<f64>,
    pixel00_loc: Point3<f64>,
    pixel_delta_u: Vector3<f64>,
    pixel_delta_v: Vector3<f64>,
    defocus_disc_u: Vector3<f64>,
    defocus_disc_v: Vector3<f64>,
    defocus_angle: f64,
    max_ray_bounce_depth: i32
}
impl Camera {
    pub fn initialize(config: CameraConfig) -> Self {
        // Init image and viewport
        let image = Image::new(config.image_width, config.aspect_ratio);
        let viewport = Viewport::new(&image, &config);

        // Camera vector space
        let camera_center = config.lookfrom;
        let w = unit_vector(config.lookfrom - config.lookat);
        let u = config.vup.cross(w);
        let v = w.cross(u);

        // Viewport edge vectors
        let viewport_u = viewport.width * u;
        let viewport_v = viewport.height * -v;

        // Deltas between horizontal & vertical vectors
        let pixel_delta_u = viewport_u / image.width as f64;
        let pixel_delta_v = viewport_v / image.height as f64;

        // Location of upper left pixel
        let viewport_upper_left = camera_center
            - (config.focus_dist * w)
            - (viewport_u/2.0)
            - (viewport_v/2.0);
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_radius = config.focus_dist * (config.defocus_angle / 2.0).to_radians().tan();
        let defocus_disc_u = u * defocus_radius;
        let defocus_disc_v = v * defocus_radius;

        Camera {
            image,
            camera_center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_disc_u,
            defocus_disc_v,
            defocus_angle: config.defocus_angle,
            max_ray_bounce_depth: config.max_depth
        }
    }
    pub fn render(self: &Camera, file: &mut File, hittables: &HittableList) -> Result<(), Error> {
        file.write_all(format!("P3\n{0} {1}\n255\n", self.image.width, self.image.height).as_bytes())?;
        let progress = Mutex::new(ProgressBar::new(self.image.height as u64));
        let pixels = (0..self.image.height).into_par_iter().map(|j| {
            let row = (0..self.image.width).into_par_iter().map(move |i| {
                (0..self.image.samples_per_pixel).into_par_iter().map(|_| {
                    let ray = self.get_ray(i, j);
                    ray.color(hittables, self.max_ray_bounce_depth)
                }).sum()
            }).collect::<Vec<Color3<f64>>>();
            progress.lock().unwrap().inc(1);
            row
        }).collect::<Vec<Vec<Color3<f64>>>>();
        for pixel_row in pixels {
            for pixel_color in pixel_row {
                write_pixel(file, &pixel_color, self.image.samples_per_pixel)?;
            }
        }
        // for j in (0..self.image.height).progress() {
        //     for i in 0..self.image.width {
        //         let mut pixel_color = Color3::new(0.0, 0.0, 0.0);
        //         for _ in 0..self.image.samples_per_pixel {
        //             let ray = self.get_ray(i, j);
        //             let ray_color = ray.color(hittables, self.max_ray_bounce_depth);
        //             pixel_color += ray_color;
        //         }
        //         write_pixel(file, &pixel_color, self.image.samples_per_pixel)?;
        //     }
        // }
        Ok(())
    }
    fn get_ray(self: &Camera, i: i32, j: i32) -> Ray {
        let pixel_center = self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let origin = if self.defocus_angle <= 0.0 { self.camera_center } else { self.defocus_disc_sample() };
        let direction = pixel_sample - origin;
        Ray { origin, direction }
    }
    fn pixel_sample_square(self: &Camera) -> Vector3<f64> {
        let mut rng = rand::thread_rng();
        let px = rng.gen_range(-0.5..0.5);
        let py = rng.gen_range(-0.5..0.5);
        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }
    fn defocus_disc_sample(self: &Camera) -> Point3<f64> {
        let p = random_in_unit_disc();
        self.camera_center + (p.x * self.defocus_disc_u) + (p.y * self.defocus_disc_v)

    }
}

pub struct CameraConfig {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pub vfov: f64,
    pub lookfrom: Point3<f64>,
    pub lookat: Point3<f64>,
    pub vup: Vector3<f64>,
    pub defocus_angle: f64,
    pub focus_dist: f64
}