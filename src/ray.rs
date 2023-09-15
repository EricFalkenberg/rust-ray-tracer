use cgmath::{ElementWise, Vector3, VectorSpace};

use Vector3 as Point3;
use crate::util;

use Vector3 as Color3;
use crate::hittable::{HittableList};
use crate::util::Interval;

pub struct Ray {
    pub origin: Point3<f64>,
    pub direction: Vector3<f64>,
    pub time: f64
}
impl Ray {
    pub fn at(self: &Ray, t: f64) -> Point3<f64> {
        self.origin + t*self.direction
    }
    pub fn color(self: &Ray, hittable_list: &HittableList, max_depth: i32) -> Color3<f64> {
        if max_depth <= 0 {
            return Color3::new(0.0, 0.0, 0.0);
        }
        let hit_record = hittable_list.hit(self, Interval::new(0.001, f64::INFINITY));
        match hit_record {
            Some(hit) => {
                let (attenuation, scattered) = hit.material.scatter(self, &hit, &hittable_list.textures);
                attenuation.mul_element_wise(scattered.color(hittable_list, max_depth - 1))
            }
            None => {
                let unit_direction = util::unit_vector(self.direction);
                let a = 0.5 * (unit_direction.y + 1.0);
                Color3::new(1.0, 1.0, 1.0).lerp(Color3::new(0.5, 0.7, 1.0), a)
            }
        }
    }
}