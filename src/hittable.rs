use cgmath::{ElementWise, InnerSpace, Vector3};
use cgmath::num_traits::Pow;
use rand::Rng;

use Vector3 as Point3;
use Vector3 as Color3;

use crate::material::Material;
use crate::util::{Interval, random_vector, random_vector_bounded, unit_vector, vector_length};
use crate::hittable::Hittable::{Circle, Triangle};
use crate::ray::Ray;
use crate::util;

pub struct HitRecord {
    pub point: Point3<f64>,
    pub normal: Vector3<f64>,
    pub t: f64,
    pub material: Material,
    pub front_face: bool
}
impl HitRecord {
    fn set_face_normal(self: &mut HitRecord, ray: &Ray, outward_normal: Vector3<f64>) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }

}
pub enum Hittable {
    Circle { center: Vector3<f64>, radius: f64, material: Material },
    Triangle { a: Point3<f64>, b: Point3<f64>, c: Point3<f64>, material: Material }
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
            },
            Triangle { a, b, c, material } => {
                let ba = b - a;
                let ca = c - a;
                let n = ba.cross(ca);
                let normal = unit_vector(n);
                let d = normal.dot(*a);

                let denom = normal.dot(ray.direction);

                if f64::abs(denom) < 1e-8 {
                    return None
                }

                let t = (d - normal.dot(ray.origin)) / denom;

                if !ray_t.contains(t) {
                    return None;
                }

                let intersection = ray.at(t);

                let edge0 = b - a;
                let edge1 = c - b;
                let edge2 = a - c;
                let c0 = intersection - a;
                let c1 = intersection - b;
                let c2 = intersection - c;
                if normal.dot(edge0.cross(c0)) > 0.0
                    && normal.dot(edge1.cross(c1)) > 0.0
                    && normal.dot(edge2.cross(c2)) > 0.0
                {
                    let mut record = HitRecord {
                        point: intersection,
                        normal,
                        t,
                        material: material.clone(),
                        front_face: true
                    };
                    record.set_face_normal(ray, normal);
                    Some(record)
                } else {
                    None
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
                    closest_t_hit = hit.t;
                    hit_record = Some(hit);
                }
                None => continue
            }
        }
        hit_record
    }
    pub fn random_spheres() -> Self {
        let mut world = Self { hittables: vec![] };
        world.add(
            Circle {
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
                            Circle {
                                center,
                                radius: 0.2,
                                material: Material::Lambertian { albedo }
                            }
                        );
                    } else if choice == 1 {
                        let albedo = random_vector_bounded(0.5, 1.0);
                        let fuzz = rng.gen_range(0.0..0.5);
                        world.add(
                            Circle {
                                center,
                                radius: 0.2,
                                material: Material::Metal { albedo, fuzz }
                            }
                        );
                    } else {
                        world.add(
                            Circle {
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
            Circle {
                center: Vector3::new(0.0, 1.0, 0.0),
                radius: 1.0,
                material: Material::Glass { refraction_index: 1.5 }
            }
        );
        world.add(
            Circle {
                center: Vector3::new(-4.0, 1.0, 0.0),
                radius: 1.0,
                material: Material::Lambertian { albedo: Color3::new(0.4, 0.2, 0.1) }
            }
        );
        world.add(
            Circle {
                center: Vector3::new(4.0, 1.0, 0.0),
                radius: 1.0,
                material: Material::Metal {
                    albedo: Color3::new(0.7, 0.6, 0.5),
                    fuzz: 0.0
                }
            }
        );
        world
    }

}
