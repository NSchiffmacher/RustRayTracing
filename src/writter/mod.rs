mod ppm_writter;
mod general_writter;

pub use ppm_writter::PpmWritter;
pub use general_writter::GeneralWritter;

use crate::color::Color;
use crate::image_info::ImageInfo;

use std::fs::File;

pub trait Writter {
    fn image_info(&self) -> &ImageInfo;

    fn set_at(&mut self, position: (usize, usize), color: Color);
    fn set_all(&mut self, color: Color);
    fn get_at(&self, position: (usize, usize)) -> Color;

    fn save(&self) -> Result<(), std::io::Error>;

    fn try_open(&self) -> Result<(), std::io::Error> {
        File::create(self.image_info().filepath.clone())?;
        Ok(())
    }
}