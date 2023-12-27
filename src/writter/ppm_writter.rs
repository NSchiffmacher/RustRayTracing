use crate::writter::Writter;
use crate::color::Color;

use std::fs::File;
use std::io::prelude::*;

pub struct PpmWritter {
    filepath: String,
    width: usize,
    height: usize,
    data: Vec<Vec<Color>>,
}

impl Writter for PpmWritter {
    fn new(filepath: String, (width, height): (usize, usize)) -> Self where Self: Sized {
        Self {
            filepath,
            width,
            height,
            data: vec![vec![Color::black(); width]; height],
        }
    }

    fn filepath(&self) -> &String {
        &self.filepath
    }

    fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    fn set_all(&mut self, color: Color) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.data[y][x] = color.clone();
            }
        }
    }

    fn set_at(&mut self, position: (usize, usize), color: Color) {
        self.data[position.1][position.0] = color;
    }

    fn get_at(&self, position: (usize, usize)) -> Color {
        self.data[position.1][position.0].clone()
    }

    fn save(&self) -> Result<(), std::io::Error> {
        let mut file = File::create(self.filepath.clone())?;

        write!(file, "P3\n{} {}\n255\n", self.width, self.height)?;
        for y in 0..self.height {
            for x in 0..self.width {
                write!(file, "{}\n", self.data[y][x].to_ppm_string())?;
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
        let writter = PpmWritter::new("test.bmp".to_string(), (2, 3));
        assert_eq!(writter.filepath, "test.bmp");
        assert_eq!(writter.width, 2);
        assert_eq!(writter.height, 3);
        assert_eq!(writter.data.len(), 3);
        assert_eq!(writter.data[0].len(), 2);
    }

    #[test]
    fn test_set() {
        let mut writter = PpmWritter::new("test.bmp".to_string(), (2, 3));
        writter.set_all(Color::new(1., 0.5, 0.22));

        for x in 0..2 {
            for y in 0..3 {
                assert_eq!(writter.data[y][x].r, 1.);
                assert_eq!(writter.data[y][x].g, 0.5);
                assert_eq!(writter.data[y][x].b, 0.22);
            }
        }

        writter.set_at((0, 0), Color::black());
        assert_eq!(writter.data[0][0].r, 0.);
        assert_eq!(writter.data[0][0].g, 0.);
        assert_eq!(writter.data[0][0].b, 0.);
    }

    #[test]
    fn test_write() {
        let tmp_dir = TempDir::new("example").unwrap();
        let file_path = tmp_dir.path().join("test.bmp").as_os_str().to_str().unwrap().to_string();

        let mut writter = PpmWritter::new(file_path, (1, 2));
        writter.set_all(Color::new(1., 0.5, 0.22));
        writter.save().unwrap();

        let mut file = File::open(tmp_dir.path().join("test.bmp")).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        assert!(contents == "P3\n1 2\n255\n255 127 56\n255 127 56\n");
    }
}