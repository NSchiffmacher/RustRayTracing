use raytracing::color::Color;
use raytracing::writter::{Writter, PpmWritter};
use raytracing::image_info::ImageInfo;

fn main() -> Result<(), std::io::Error> {
    let width = 256;
    let height = 256;

    let image_info = ImageInfo::new(width, height, "output/weekend/weekend1.ppm".to_string(), 1, 1);
    let mut writter: Box<dyn Writter> = Box::new(PpmWritter::new(image_info.clone()));
    writter.try_open()?;

    print!("Starting rendering...");
    let rendering_start = std::time::Instant::now();
    
    for y in 0..height {
        for x in 0..width {
            let r: f64 = y as f64 / (height as f64 - 1.);
            let g: f64 = x as f64 / (width as f64 - 1.);
            let b: f64 = 0.;

            writter.set_at((x, y), Color::new(r, g, b));

            print!("\rStarting rendering... {:.2}%    ", 100. * ((height * y + x) as f64) / ((width * height) as f64));
        }
    }

    print!("\rStarting rendering... Done in {:.2}s.\nSaving...", rendering_start.elapsed().as_secs_f64());

    let saving_start = std::time::Instant::now();
    writter.save()?;
    print!("\rSaving... Saving done in {:.2}s.\n", saving_start.elapsed().as_secs_f64());

    Ok(())
}
