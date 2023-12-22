pub mod ppm_writter;

pub use ppm_writter::PpmWritter;

use crate::color::Color;
use std::fs::File;

pub trait Writter {
    fn new(filepath: String, size: (usize, usize)) -> Self where Self: Sized;

    fn filepath(&self) -> &String;
    fn size(&self) -> (usize, usize);

    fn set_at(&mut self, position: (usize, usize), color: Color);
    fn set_all(&mut self, color: Color);
    fn get_at(&self, position: (usize, usize)) -> Color;

    fn save(&self) -> Result<(), std::io::Error>;

    fn try_open(&self) -> Result<(), std::io::Error> {
        File::create(self.filepath())?;
        Ok(())
    }
}