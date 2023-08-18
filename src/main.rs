mod image;
mod viewport;
mod ray;
mod util;
mod hittable;
mod camera;
mod material;

use std::fs::File;
use std::io::{Error};
use cgmath::{ElementWise, Vector3};
use rand::Rng;

use crate::camera::{Camera, CameraConfig};
use crate::hittable::{Hittable, HittableList};

use Vector3 as Color3;
use Vector3 as Point3;
use crate::material::Material;
use crate::util::{random_vector, random_vector_bounded, vector_length};

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
    let mut world = HittableList { hittables: vec![] };
    world.add(
        Hittable::Circle {
            center: Vector3::new(0.0, -1000.0, 0.0),
            radius: 1000.0,
            material: Material::Lambertian { albedo: Color3::new(0.5, 0.5, 0.5) }
        }
    );
    let mut rng = rand::thread_rng();
    let offset_point = Point3::new(4.0, 0.2, 0.0);
    for a in -11..11 {
        for b in -11..11 {
            let choice = rng.gen_range(0..=2);
            let a_variance: f64 = rng.gen();
            let b_variance: f64 = rng.gen();
            let center = Point3::new(a as f64 + 0.9*a_variance, 0.2, b as f64 + 0.9*b_variance);
            if vector_length(center - offset_point) > 0.9 {
                if choice == 0 {
                    let albedo = random_vector().mul_element_wise(random_vector());
                    world.add(
                        Hittable::Circle {
                            center,
                            radius: 0.2,
                            material: Material::Lambertian { albedo }
                        }
                    );
                } else if choice == 1 {
                    let albedo = random_vector_bounded(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    world.add(
                        Hittable::Circle {
                            center,
                            radius: 0.2,
                            material: Material::Metal { albedo, fuzz }
                        }
                    );
                } else {
                    world.add(
                        Hittable::Circle {
                            center,
                            radius: 0.2,
                            material: Material::Glass { refraction_index: 1.5 }
                        }
                    );
                }
            }
        }
    }
    world.add(
        Hittable::Circle {
            center: Vector3::new(0.0, 1.0, 0.0),
            radius: 1.0,
            material: Material::Glass { refraction_index: 1.5 }
        }
    );
    world.add(
        Hittable::Circle {
            center: Vector3::new(-4.0, 1.0, 0.0),
            radius: 1.0,
            material: Material::Lambertian { albedo: Color3::new(0.4, 0.2, 0.1) }
        }
    );
    world.add(
        Hittable::Circle {
            center: Vector3::new(4.0, 1.0, 0.0),
            radius: 1.0,
            material: Material::Metal {
                albedo: Color3::new(0.7, 0.6, 0.5),
                fuzz: 0.0
            }
        }
    );
    camera.render(&mut output_image, &world)
}
