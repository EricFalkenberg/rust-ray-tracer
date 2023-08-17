use cgmath::Vector3;

use Vector3 as Color3;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::util;
use crate::util::{Interval, near_zero, random_unit_vector, reflect};

pub enum Material {
    Lambertian { albedo: Color3<f64> },
    Metal { albedo: Color3<f64>, fuzz: f64 }
}

impl Material {
    pub fn scatter(self: &Material, ray: &Ray, hit: &HitRecord) -> (Color3<f64>, Ray) {
        match self {
            Material::Lambertian { albedo } => {
                let mut scatter_direction = hit.normal + util::random_on_hemisphere(&hit.normal);
                if near_zero(scatter_direction) {
                    scatter_direction = hit.normal;
                }
                let new_ray = Ray {
                    origin: hit.point,
                    direction: scatter_direction
                };
                (*albedo, new_ray)
            }
            Material::Metal { albedo, fuzz } => {
                let fuzz_range = Interval::new(0.0, 1.0);
                let reflection = reflect(ray.direction, hit.normal);
                let new_ray = Ray {
                    origin: hit.point,
                    direction: reflection + fuzz_range.clamp(*fuzz)*random_unit_vector(),
                };
                (*albedo, new_ray)
            }
        }
    }
    pub fn clone(self: &Material) -> Material {
        match self {
            Material::Lambertian { albedo } => Material::Lambertian { albedo: albedo.clone() },
            Material::Metal { albedo, fuzz } => Material::Metal { albedo: albedo.clone(), fuzz: *fuzz }
        }
    }
}