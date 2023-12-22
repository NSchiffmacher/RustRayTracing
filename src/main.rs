

use writter::Writter;

pub mod writter;
pub mod vector;

fn main() -> Result<(), std::io::Error> {
    let width = 256;
    let height = 256;
    let mut writter: Box<dyn Writter> = Box::new(writter::BmpWritter::new("outzput/test.bpm".to_string(), (width, height)));
    writter.try_open()?;
    
    for y in 0..height {
        for x in 0..width {
            let r: f64 = y as f64 / (height as f64 - 1.);
            let g: f64 = x as f64 / (width as f64 - 1.);
            let b: f64 = 0.;

            writter.set_at((x, y), (r, g, b));
        }
    }

    writter.save()
}
