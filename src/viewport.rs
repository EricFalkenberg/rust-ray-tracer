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
        Self {
            width: 2.0 * (image_width as f64 / image_height as f64),
            height: 2.0,
            focal_length: 1.0,
            camera_center: Point3::new(0.0, 0.0, 0.0)
        }
    }
}