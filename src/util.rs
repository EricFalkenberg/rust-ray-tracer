use std::fs::File;
use std::io::{Error, Write};
use cgmath::num_traits::Pow;
use cgmath::{InnerSpace, Vector3};
use rand::Rng;

use Vector3 as Color3;

pub fn random_in_unit_disc() -> Vector3<f64> {
    let mut rng = rand::thread_rng();
    loop {
        let p = Vector3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
        if vector_length_squared(p) < 1.0 {
            return p;
        }
    }
}
pub fn refract(uv: Vector3<f64>, normal: Vector3<f64>, etai_over_etat: f64) -> Vector3<f64> {
    // gonna be real with you, i don't understand a lick of this math.
    let cos_theta = f64::min((-uv).dot(normal), 1.0);
    let r_out_perp = etai_over_etat * (uv + (cos_theta * normal));
    let r_out_parallel = (
        -f64::sqrt(
            f64::abs(1.0 - vector_length_squared(r_out_perp))
        )
    ) * normal;
    r_out_perp + r_out_parallel
}
pub fn reflect(v: Vector3<f64>, normal: Vector3<f64>) -> Vector3<f64> {
    v - 2.0 * v.dot(normal) * normal
}
pub fn near_zero(v: Vector3<f64>) -> bool {
    let s = 1e-8;
    (f64::abs(v.x) < s) && (f64::abs(v.y) < s) && (f64::abs(v.z) < s)
}
pub fn random_on_hemisphere(normal: &Vector3<f64>) -> Vector3<f64> {
    let on_hemisphere = random_unit_vector();
    if normal.dot(on_hemisphere) > 0.0 {
        on_hemisphere
    } else {
        -on_hemisphere
    }
}
pub fn random_unit_vector() -> Vector3<f64> {
    unit_vector(random_vector_in_unit_sphere())
}
pub fn random_vector_in_unit_sphere() -> Vector3<f64> {
    loop {
        let v = random_vector_bounded(-1.0, 1.0);
        if vector_length_squared(v) < 1.0 {
            return v;
        }
    }
}
pub fn random_vector_bounded(min: f64, max: f64) -> Vector3<f64> {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(min..max);
    let y = rng.gen_range(min..max);
    let z = rng.gen_range(min..max);
    Vector3::new(x, y, z)
}
#[allow(dead_code)]
pub fn random_vector() -> Vector3<f64> {
    random_vector_bounded(0.0, 1.0)
}
pub fn unit_vector(v: Vector3<f64>) -> Vector3<f64> {
    let length = vector_length(v);
    v / length
}
pub fn vector_length(v: Vector3<f64>) -> f64 {
    f64::sqrt(vector_length_squared(v))
}
pub fn vector_length_squared(v: Vector3<f64>) -> f64 {
    v.x.pow(2) + v.y.pow(2) + v.z.pow(2)
}
pub struct Interval {
    pub min: f64,
    pub max: f64
}
impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }
    #[allow(dead_code)]
    pub fn contains(self: &Interval, x: f64) -> bool {
        self.min <= x && x <= self.max
    }
    pub fn surrounds(self: &Interval, x: f64) -> bool {
        self.min < x && x < self.max
    }
    pub fn clamp(self: &Interval, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }
    #[allow(dead_code)]
    pub const EMPTY: Interval = Interval { min: f64::INFINITY, max: f64::NEG_INFINITY };
    #[allow(dead_code)]
    pub const WORLD: Interval = Interval { min: f64::NEG_INFINITY, max: f64::INFINITY };
}
pub fn write_pixel(file: &mut File, color: &Color3<f64>, samples_per_pixel: i32) -> Result<(), Error> {
    let scaled = color / samples_per_pixel as f64;
    let r = linear_to_gamma(scaled.x);
    let g = linear_to_gamma(scaled.y);
    let b = linear_to_gamma(scaled.z);
    let intensity = Interval::new(0.0, 0.999);
    let ir: i32 = (256.0 * intensity.clamp(r)) as i32;
    let ig: i32 = (256.0 * intensity.clamp(g)) as i32;
    let ib: i32 = (256.0 * intensity.clamp(b)) as i32;
    file.write_all(format!("{0} {1} {2}\n", ir, ig, ib).as_bytes())
}
fn linear_to_gamma(i: f64) -> f64 {
    f64::sqrt(i)
}


