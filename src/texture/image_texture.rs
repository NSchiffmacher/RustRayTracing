use crate::vector::Vec3;
use crate::color::Color;
use crate::texture::{Texture, Uv};

use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImageView};

use std::rc::Rc;

pub struct ImageTexture {
    image: DynamicImage,
    width_f: f64,
    height_f: f64,
}

impl ImageTexture {
    pub fn new(path: &str) -> Result<Rc<dyn Texture>, image::ImageError> {
        let image = ImageReader::open(path)?.decode()?;
        Ok(Rc::new(Self {
            width_f: image.width() as f64,
            height_f: image.height() as f64,
            image,
        }))
    }
}

impl Texture for ImageTexture {
    fn value(&self, uv: &Uv, _p: &Vec3) -> Color {
        // Assert that uv coordiantes are from 0 to 1
        let u = uv.u.clamp(0., 1.);
        let v = 1.0 - uv.v.clamp(0., 1.); // Flip V (//! TODO: Why?)

        let i: u32 = (u * self.width_f).floor() as u32;
        let j: u32 = (v * self.height_f).floor() as u32;
        let pixel = self.image.get_pixel(i, j);

        let scale = 1. / 255.;
        let r = pixel[0] as f64 * scale;
        let g = pixel[1] as f64 * scale;
        let b = pixel[2] as f64 * scale;
        Color::new(r, g, b)
    }
}