mod scene;
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

use crate::camera::Camera;
use crate::hittable::{Hittable, HittableList};

use Vector3 as Color3;
use crate::material::Material;

fn main() -> Result<(), Error> {
    let mut output_image = File::create("out.ppm")?;
    let camera = Camera::initialize();
    let world = HittableList {
        hittables: vec![
            Hittable::Circle {
                center: Vector3::new(0.0, 0.0, -1.5),
                radius: 0.5,
                material: Material::Lambertian { albedo: Color3::new(0.7, 0.3, 0.3) }
            },
            Hittable::Circle {
                center: Vector3::new(0.0, -100.5, -1.0),
                radius: 100.0,
                material: Material::Lambertian { albedo: Color3::new(0.8, 0.8, 0.0) }
            },
            Hittable::Circle {
                center: Vector3::new(-1.0, 0.0, -1.5),
                radius: 0.5,
                material: Material::Metal { albedo: Color3::new(0.8, 0.8, 0.8 ), fuzz: 0.1 }
            },
            Hittable::Circle {
                center: Vector3::new(1.0, 0.0, -1.5),
                radius: 0.5,
                material: Material::Metal { albedo: Color3::new(0.8, 0.6, 0.2 ), fuzz: 0.5 }
            }
        ]
    };
    camera.render(&mut output_image, &world)
}
