use cgmath::{InnerSpace, Vector3};
use cgmath::num_traits::Pow;
use rand::Rng;

use Vector3 as Color3;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::util;
use crate::util::{Interval, near_zero, random_unit_vector, reflect, refract, unit_vector};

pub enum Material {
    Lambertian { albedo: Color3<f64> },
    Metal { albedo: Color3<f64>, fuzz: f64 },
    Glass { refraction_index: f64 }
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
            Material::Glass { refraction_index } => {
                let mut rng = rand::thread_rng();
                let attenuation = Color3::new(1.0, 1.0, 1.0);
                let refraction_ratio = if hit.front_face { 1.0 / *refraction_index } else { *refraction_index };

                let unit_direction = unit_vector(ray.direction);
                let cos_theta = f64::min((-unit_direction).dot(hit.normal), 1.0);
                let sin_theta = f64::sqrt(1.0 - cos_theta*cos_theta);

                let cannot_refract = (refraction_ratio * sin_theta) > 1.0;
                let dbl: f64 = rng.gen_range(0.0..1.0);

                let direction = if cannot_refract || Material::reflectance(cos_theta, refraction_ratio) > dbl {
                    reflect(unit_direction, hit.normal)
                } else {
                    refract(unit_direction, hit.normal, refraction_ratio)
                };
                let scattered = Ray { origin: hit.point, direction };
                (attenuation, scattered)
            }
        }
    }
    pub fn clone(self: &Material) -> Material {
        match self {
            Material::Lambertian { albedo } => Material::Lambertian { albedo: *albedo },
            Material::Metal { albedo, fuzz } => Material::Metal { albedo: *albedo, fuzz: *fuzz },
            Material::Glass { refraction_index } => Material::Glass { refraction_index: *refraction_index }
        }
    }
    fn reflectance(cos: f64, ref_idx: f64) -> f64 {
        let r0 = ((1.0-ref_idx) / (1.0+ref_idx)).pow(2.0);
        r0 + (1.0-r0)*((1.0-cos).pow(5.0))
    }
}