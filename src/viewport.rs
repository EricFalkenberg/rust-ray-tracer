use crate::camera::CameraConfig;
use crate::image::Image;

pub struct Viewport {
    pub width: f64,
    pub height: f64
}
impl Viewport {

    pub fn new(image: &Image, camera_config: &CameraConfig) -> Self {
        let theta = camera_config.vfov.to_radians();
        let h = (theta/2.0).tan();
        let height = 2.0 * h * camera_config.focus_dist;
        Self {
            width: height * (image.width as f64 / image.height as f64),
            height
        }
    }
}