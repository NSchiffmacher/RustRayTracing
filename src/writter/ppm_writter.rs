use crate::writter::Writter;
use crate::color::Color;
use crate::image_info::ImageInfo;

use std::fs::File;
use std::io::prelude::*;
use indicatif::{ProgressStyle, ProgressIterator};

pub struct PpmWritter {
    image_info: ImageInfo,
    data: Vec<Vec<Color>>,
}

impl Writter for PpmWritter {
    fn new(image_info: ImageInfo) -> Self where Self: Sized {
        Self {
            image_info: image_info.clone(),
            data: vec![vec![Color::black(); image_info.width]; image_info.height],
        }
    }
    
    fn image_info(&self) -> &ImageInfo {
        &self.image_info
    }

    fn set_all(&mut self, color: Color) {
        for y in 0..self.image_info.height {
            for x in 0..self.image_info.width {
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
        let progress_style = ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.green/blue}] {percent}% ({eta_precise})")
            .unwrap()
            .progress_chars("=>-");
        let saving_start = std::time::Instant::now();
        let mut file = File::create(self.image_info.filepath.clone())?;

        write!(file, "P3\n{} {}\n255\n", self.image_info.width, self.image_info.height)?;
        for y in (0..self.image_info.height).progress_with_style(progress_style) {
            for x in 0..self.image_info.width {
                write!(file, "{}\n", self.data[y][x].to_ppm_string())?;
            }
        }

        println!("Saving done in {:.2}s.\r", saving_start.elapsed().as_secs_f64());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempdir::TempDir;

    #[test]
    fn test_set() {
        let mut options = ImageInfo::from_aspect_ratio(0., 0, "test.ppm".to_string(), 10, 10);
        options.width = 2;
        options.height = 3;
        let mut writter = PpmWritter::new(options);
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

        let mut options = ImageInfo::from_aspect_ratio(0., 0, file_path, 10, 10);
        options.width = 1;
        options.height = 2;
        let mut writter = PpmWritter::new(options);

        writter.set_all(Color::new(1., 0.5, 0.22));
        writter.save().unwrap();

        let mut file = File::open(tmp_dir.path().join("test.bmp")).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        assert!(contents == "P3\n1 2\n255\n255 127 56\n255 127 56\n");
    }
}