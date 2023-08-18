use cgmath::Vector3;

use Vector3 as Point3;

pub struct Viewport {
    pub width: f64,
    pub height: f64,
    pub focal_length: f64,
    pub camera_center: Point3<f64>
}
impl Viewport {

    pub fn new(image_width: i32, image_height: i32) -> Self {
        let focal_length = 1.0;
        let vfov: f64 = 90.0;
        let theta = vfov.to_radians();
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h * focal_length;
        Self {
            width: viewport_height * (image_width as f64 / image_height as f64),
            height: viewport_height,
            focal_length,
            camera_center: Point3::new(-2.0, 2.0, 1.0)
        }
    }
}