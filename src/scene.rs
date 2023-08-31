use std::fs::File;
use std::io::Error;
use std::path::PathBuf;
use cgmath::Vector3;
use crate::camera::{Camera, CameraConfig};

use Vector3 as Point3;
use Vector3 as Color3;
use crate::hittable::{Hittable, HittableList};
use crate::material::Material;
use crate::model::Model;
use crate::Scene;

pub fn render(scene: Scene, output: Option<PathBuf>) -> Result<(), Error> {
    let output_file = File::create(output.unwrap_or(PathBuf::from("out.ppm")))?;
    match scene {
        Scene::Spheres => {
            render_spheres(output_file)
        }
        Scene::Link => {
            render_link(output_file)
        }
    }
}
fn render_spheres(mut output_image: File) -> Result<(), Error> {
    let camera = Camera::initialize(
        CameraConfig {
            aspect_ratio: 16.0 / 9.0,
            image_width: 1200,
            samples_per_pixel: 500,
            max_depth: 50,
            vfov: 15.0,
            lookfrom: Point3::new(13.0, 2.0, 3.0),
            lookat: Point3::new(0.0, 0.0, 0.0),
            vup: Vector3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.6,
            focus_dist: 10.0
        }
    );
    let world = HittableList::random_spheres();
    camera.render(&mut output_image, &world)
}
fn render_link(mut output_image: File) -> Result<(), Error> {
    let camera = Camera::initialize(
        CameraConfig {
            aspect_ratio: 16.0 / 9.0,
            image_width: 1200,
            samples_per_pixel: 500,
            max_depth: 50,
            vfov: 15.0,
            lookfrom: Point3::new(13.0, 2.0, 3.0),
            lookat: Point3::new(0.0, 1.0, 0.0),
            vup: Vector3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.6,
            focus_dist: 13.37
        }
    );
    let model = Model::new(
        String::from("./models/zelda"),
        String::from("Link Adult"),
        vec![
            rotate_y(25.0),
            scale(0.01)
        ]
    );
    let mut world = HittableList{ hittables: model.faces };
    world.add(
        Hittable::Circle {
            center: Vector3::new(0.0, -1000.0, 0.0),
            radius: 1000.0,
            material: Material::Metal { albedo: Color3::new(0.5, 0.5, 0.5), fuzz: 0.5 }
        }
    );
    camera.render(&mut output_image, &world)
}

#[allow(dead_code)]
fn rotate_x(deg: f64) -> Box<dyn Fn(Point3<f64>) -> Point3<f64>> {
    let theta = deg.to_radians();
    Box::new(move |p| {
        Point3::new(
            p.x,
            p.y * theta.cos() - p.z * theta.sin(),
            p.y * theta.sin() + p.z * theta.cos(),
        )
    })
}
fn rotate_y(deg: f64) -> Box<dyn Fn(Point3<f64>) -> Point3<f64>> {
    let theta = deg.to_radians();
    Box::new(move |p| {
        Point3::new(
            p.x * theta.cos() + p.z * theta.sin(),
            p.y,
            p.z * theta.cos() - p.x * theta.sin()
        )
    })
}
fn scale(percentage: f64) -> Box<dyn Fn(Point3<f64>) -> Point3<f64>> {
    Box::new(move |p| {
        Point3::new(
            p.x * percentage,
            p.y * percentage,
            p.z * percentage
        )
    })
}

