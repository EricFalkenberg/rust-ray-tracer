use crate::image::Image;
use crate::viewport::Viewport;

pub struct Scene {
    pub image: Image,
    pub viewport: Viewport
}
impl Scene {
    pub fn new(width: i32, aspect_ratio: f64) -> Self {
        let height = (width as f64 / aspect_ratio) as i32;
        Self {
            image: Image::new(width, aspect_ratio),
            viewport: Viewport::new(width, height)
        }
    }
}

