mod image;
mod viewport;
mod ray;
mod util;
mod hittable;
mod camera;
mod material;

use std::fs::File;
use std::io::{Error};
use cgmath::{Vector3};

use crate::camera::{Camera, CameraConfig};
use crate::hittable::{HittableList};

use Vector3 as Point3;

fn main() -> Result<(), Error> {
    let mut output_image = File::create("out.ppm")?;
    let camera = Camera::initialize(
        CameraConfig {
            aspect_ratio: 16.0 / 9.0,
            image_width: 1200,
            samples_per_pixel: 500,
            max_depth: 50,
            vfov: 20.0,
            lookfrom: Point3::new(13.0, 2.0, 3.0),
            lookat: Point3::new(0.0, 0.0, 0.0),
            vup: Vector3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.6,
            focus_dist: 10.0
        }
    );
    let world = HittableList::random_world();
    camera.render(&mut output_image, &world)
}
