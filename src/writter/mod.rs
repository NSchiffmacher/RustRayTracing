pub mod bmp_writter;

pub use bmp_writter::BmpWritter;

use std::fs::File;

pub trait Writter {
    fn new(filepath: String, size: (usize, usize)) -> Self where Self: Sized;

    fn filepath(&self) -> &String;
    fn size(&self) -> (usize, usize);

    fn set_at(&mut self, position: (usize, usize), color: (f64, f64, f64));
    fn set_all(&mut self, color: (f64, f64, f64));
    fn get_at(&self, position: (usize, usize)) -> (f64, f64, f64);

    fn save(&self) -> Result<(), std::io::Error>;

    fn try_open(&self) -> Result<(), std::io::Error> {
        File::open(self.filepath())?;
        Ok(())
    }
}