use crate::writter::Writter;

use std::fs::File;
use std::io::prelude::*;

pub struct BmpWritter {
    filepath: String,
    width: usize,
    height: usize,
    data: Vec<Vec<(f64, f64, f64)>>,
}

impl Writter for BmpWritter {
    fn new(filepath: String, (width, height): (usize, usize)) -> Self where Self: Sized {
        Self {
            filepath,
            width,
            height,
            data: vec![vec![(0., 0., 0.); width]; height],
        }
    }

    fn filepath(&self) -> &String {
        &self.filepath
    }

    fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    fn set_all(&mut self, color: (f64, f64, f64)) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.data[y][x] = color;
            }
        }
    }

    fn set_at(&mut self, position: (usize, usize), color: (f64, f64, f64)) {
        self.data[position.1][position.0] = color;
    }

    fn get_at(&self, position: (usize, usize)) -> (f64, f64, f64) {
        self.data[position.1][position.0]
    }

    fn save(&self) -> Result<(), std::io::Error> {
        let mut file = File::create(self.filepath.clone())?;

        write!(file, "P3\n{} {}\n255\n", self.width, self.height)?;
        for y in 0..self.height {
            for x in 0..self.width {
                let (r, g, b) = self.data[y][x];

                let ir = (r * 255.).floor() as u8;
                let ig = (g * 255.).floor() as u8;
                let ib = (b * 255.).floor() as u8;

                write!(file, "{} {} {}\n", ir, ig, ib)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempdir::TempDir;

    #[test]
    fn test_new() {
        let writter = BmpWritter::new("test.bmp".to_string(), (2, 3));
        assert_eq!(writter.filepath, "test.bmp");
        assert_eq!(writter.width, 2);
        assert_eq!(writter.height, 3);
        assert_eq!(writter.data.len(), 3);
        assert_eq!(writter.data[0].len(), 2);
    }

    #[test]
    fn test_set() {
        let mut writter = BmpWritter::new("test.bmp".to_string(), (2, 3));
        writter.set_all((1., 0.5, 0.22));

        for x in 0..2 {
            for y in 0..3 {
                assert_eq!(writter.data[y][x], (1., 0.5, 0.22));
            }
        }

        writter.set_at((0, 0), (0., 0., 0.));
        assert_eq!(writter.data[0][0], (0., 0., 0.));
    }

    #[test]
    fn test_write() {
        let tmp_dir = TempDir::new("example").unwrap();
        let file_path = tmp_dir.path().join("test.bmp").as_os_str().to_str().unwrap().to_string();

        let mut writter = BmpWritter::new(file_path, (1, 2));
        writter.set_all((1., 0.5, 0.22));
        writter.save().unwrap();

        let mut file = File::open(tmp_dir.path().join("test.bmp")).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        assert!(contents == "P3\n1 2\n255\n255 127 56\n255 127 56\n");
    }
}