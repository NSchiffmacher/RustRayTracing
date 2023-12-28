#[derive(Debug, Clone)]
pub struct ImageInfo {
    pub width: usize,
    pub height: usize,
    pub aspect_ratio: f64,
    pub samples_per_pixel: usize,
    pub filepath: String,
    pub max_depth: usize,
}

impl ImageInfo {
    pub fn from_aspect_ratio(aspect_ratio: f64, width: usize, filepath: String, samples_per_pixel: usize, max_depth: usize) -> Self {
        let height = ((width as f64) / aspect_ratio) as usize;
        Self {
            width,
            height,
            aspect_ratio,
            filepath,
            samples_per_pixel,
            max_depth,
        }
    }

    pub fn new(width: usize, height: usize, filepath: String, samples_per_pixel: usize, max_depth: usize) -> Self {
        let aspect_ratio = (width as f64) / (height as f64);
        Self {
            width,
            height,
            aspect_ratio,
            filepath,
            samples_per_pixel,
            max_depth,
        }
    }
}