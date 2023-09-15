use std::collections::HashMap;
use cgmath::{InnerSpace, Vector2, Vector3};
use cgmath::num_traits::Pow;
use image::{DynamicImage, GenericImageView, Pixel};
use rand::Rng;

use Vector3 as Color3;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::util;
use crate::util::{Interval, near_zero, random_unit_vector, reflect, refract, unit_vector};

use Vector2 as Point2;

pub enum Material {
    Lambertian { albedo: Color3<f64> },
    Texture { key: String, a: Vector2<f64>, b: Vector2<f64>, c: Vector2<f64> },
    Metal { albedo: Color3<f64>, fuzz: f64 },
    Glass { refraction_index: f64 }
}

impl Material {
    pub fn scatter(self: &Material, ray: &Ray, hit: &HitRecord, textures: &HashMap<String, DynamicImage>) -> (Color3<f64>, Ray) {
        match self {
            Material::Lambertian { albedo } => {
                let mut scatter_direction = hit.normal + util::random_on_hemisphere(&hit.normal);
                if near_zero(scatter_direction) {
                    scatter_direction = hit.normal;
                }
                let new_ray = Ray {
                    origin: hit.point,
                    direction: scatter_direction,
                    time: ray.time
                };
                (*albedo, new_ray)
            }
            Material::Metal { albedo, fuzz } => {
                let fuzz_range = Interval::new(0.0, 1.0);
                let reflection = reflect(ray.direction, hit.normal);
                let new_ray = Ray {
                    origin: hit.point,
                    direction: reflection + fuzz_range.clamp(*fuzz)*random_unit_vector(),
                    time: ray.time
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
                let scattered = Ray { origin: hit.point, direction, time: ray.time };
                (attenuation, scattered)
            }
            Material::Texture { key, a: at, b: bt, c: ct} => {
                let image = textures.get(key).unwrap();
                match &hit.object {
                    Hittable::Circle { .. } => { todo!() }
                    Hittable::Triangle { a: av, b: bv, c: cv, material: _ } => {
                        let p = hit.point;
                        let bary_a = ((bv.y-cv.y)*(p.x-cv.x)+(cv.x-bv.x)*(p.y-cv.y))/((bv.y-cv.y)*(av.x-cv.x)+(cv.x-bv.x)*(av.y-cv.y));
                        let bary_b = ((cv.y-av.y)*(p.x-cv.x)+(av.x-cv.x)*(p.y-cv.y))/((bv.y-cv.y)*(av.x-cv.x)+(cv.x-bv.x)*(av.y-cv.y));
                        let bary_c = 1.0 - bary_a - bary_b;
                        let p_uv = bary_a * at + bary_b * bt + bary_c * ct;
                        let height = image.height();
                        let width = image.width();
                        let x_pixel = f64::round(p_uv.x * width as f64) as u32;
                        let y_pixel = f64::round(p_uv.y * height as f64) as u32;
                        let color = image.get_pixel(x_pixel, height-y_pixel).to_rgb();
                        let albedo = Color3::new(color.0[0] as f64 / 255.0, color.0[1] as f64 / 255.0, color.0[2] as f64 / 255.0);
                        let new_mat = Material::Lambertian { albedo };
                        new_mat.scatter(ray, hit, textures)
                    }
                }
            }
        }
    }
    pub fn clone(self: &Material) -> Material {
        match self {
            Material::Lambertian { albedo } => Material::Lambertian { albedo: *albedo },
            Material::Metal { albedo, fuzz } => Material::Metal { albedo: *albedo, fuzz: *fuzz },
            Material::Glass { refraction_index } => Material::Glass { refraction_index: *refraction_index },
            Material::Texture { key, a, b, c } => Material::Texture { key: key.clone(), a: *a, b: *b, c: *c}
        }
    }
    fn reflectance(cos: f64, ref_idx: f64) -> f64 {
        let r0 = ((1.0-ref_idx) / (1.0+ref_idx)).pow(2.0);
        r0 + (1.0-r0)*((1.0-cos).pow(5.0))
    }
}