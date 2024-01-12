use std::io::Error;

use crate::writter::Writter;
use crate::image_info::ImageInfo;
use crate::color::Color;

use image::RgbImage;

pub struct GeneralWritter {
    image: RgbImage,
    image_info: ImageInfo,
}

impl Writter for GeneralWritter {
    fn new(image_info: ImageInfo) -> Self where Self: Sized {
        Self {
            image: RgbImage::new(image_info.width as u32, image_info.height as u32),
            image_info,
        }
    }

    fn image_info(&self) -> &ImageInfo {
        &self.image_info
    }

    fn get_at(&self, position: (usize, usize)) -> Color {
        let px = self.image.get_pixel(position.0 as u32, position.1 as u32);
        Color {
            r: (px[0] as f64) / 255.,
            g: (px[1] as f64) / 255.,
            b: (px[2] as f64) / 255.,
        }
    }

    fn save(&self) -> Result<(), std::io::Error> {
        let save = self.image.save(self.image_info.filepath.clone());
        if save.is_err() {
            Err(Error::new(std::io::ErrorKind::Other, format!("Error saving image: {}", save.err().unwrap())))
        } else {
            Ok(())
        }
    }

    fn set_all(&mut self, color: Color) {
        let (ir, ig, ib) = color.to_integer_rgb();
        let pixel = image::Rgb([ir, ig, ib]);
        self.image = RgbImage::from_pixel(self.image_info.width as u32, self.image_info.height as u32, pixel);
    }

    fn set_at(&mut self, position: (usize, usize), color: Color) {
        let (ir, ig, ib) = color.to_integer_rgb();
        let pixel = image::Rgb([ir, ig, ib]);
        self.image.put_pixel(position.0 as u32, position.1 as u32, pixel);
    }
}