use cgmath::{InnerSpace, Vector3};
use cgmath::num_traits::Pow;
use crate::hittable::Hittable::Circle;
use crate::ray::Ray;
use crate::util;

use Vector3 as Point3;
use crate::material::Material;
use crate::util::Interval;

pub struct HitRecord {
    pub point: Point3<f64>,
    pub normal: Vector3<f64>,
    pub t: f64,
    pub material: Material,
    pub front_face: bool
}
impl HitRecord {
    fn set_face_normal(mut self: &mut HitRecord, ray: &Ray, outward_normal: Vector3<f64>) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }

}
pub enum Hittable {
    Circle { center: Vector3<f64>, radius: f64, material: Material }
}
impl Hittable {
    pub fn hit(self: &Hittable, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        match self {
            Circle { center, radius, material } => {
                let oc = ray.origin - center;
                let a = util::vector_length_squared(ray.direction);
                let half_b = oc.dot(ray.direction);
                let c = util::vector_length_squared(oc) - radius.pow(2);
                let discriminant = half_b.pow(2) - a*c;
                if discriminant < 0.0 { return None }

                match find_nearest_root(a, half_b, discriminant, ray_t) {
                    Some(root) => {
                        let intersect_point = ray.at(root);
                        let outward_normal = (intersect_point - center) / *radius;
                        let mut record = HitRecord {
                            point: intersect_point,
                            normal: outward_normal,
                            t: root,
                            material: material.clone(),
                            front_face: true
                        };
                        record.set_face_normal(ray, outward_normal);
                        Some(record)
                    }
                    None => None
                }
            }
        }
    }
}

fn find_nearest_root(a: f64, half_b: f64, discriminant: f64, ray_t: Interval) -> Option<f64> {
    let sqrtd = f64::sqrt(discriminant);
    let mut root = (-half_b - sqrtd) / a;
    if !ray_t.surrounds(root) {
        root = (-half_b + sqrtd) / a;
        if !ray_t.surrounds(root) {
            return None
        }
    }
    Some(root)
}
pub struct HittableList {
    pub hittables: Vec<Hittable>
}
impl HittableList {
    pub fn add(self: &mut HittableList, hittable: Hittable) {
        self.hittables.push(hittable);
    }
    pub fn hit(self: &HittableList, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut hit_record: Option<HitRecord> = None;
        let mut closest_t_hit = ray_t.max;
        for hittable in &self.hittables {
            match hittable.hit(ray, Interval::new(ray_t.min, closest_t_hit)) {
                Some(hit) => {
                    closest_t_hit = hit.t.clone();
                    hit_record = Some(hit);
                }
                None => continue
            }
        }
        hit_record
    }
}
