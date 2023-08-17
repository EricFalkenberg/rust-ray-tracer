pub struct Image {
    pub width: i32,
    pub height: i32,
    pub aspect_ratio: f64,
    pub samples_per_pixel: i32
}
impl Image {
    pub fn new(width: i32, aspect_ratio: f64) -> Self {
        Image {
            width,
            height: (width as f64 / aspect_ratio) as i32,
            aspect_ratio,
            samples_per_pixel: 100
        }
    }
}