
use raytracing::camera::Camera;
use raytracing::color::Color;
use raytracing::material::*;
use raytracing::writter::{Writter, PpmWritter};
use raytracing::vector::Point;
use raytracing::hittable::{HittableList, Sphere};
use raytracing::image_info::ImageInfo;

use std::rc::Rc;

fn main() -> Result<(), std::io::Error> {
    // Constants
    const WIDTH: usize = 800;
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const SAMPLES_PER_PIXEL: usize = 100;
    const MAX_DEPTH: usize = 50;
    const VERTICAL_FOV: f64 = 90.0;
    const FILEPATH: &str = "output/weekend/weekend4.ppm";

    const LOOK_FROM: Point = Point::new(0., 0., -1.);
    const LOOK_AT: Point = Point::new(0., 0., 0.);
    const UP: Point = Point::new(0., 1., 0.);

    const R: f64 = 0.70710678118;

    // Materials
    let material_left = Rc::new(Lambertian::new(Color::blue()));
    let material_right = Rc::new(Lambertian::new(Color::red()));

    // Objects
    let mut world = HittableList::new();
    world.add(Sphere::boxed(Point::new(-R, 0., 0.), R, material_left.clone()));
    world.add(Sphere::boxed(Point::new(R, 0., 0.), R, material_right.clone()));
    
    // Image settings
    let image_info = ImageInfo::from_aspect_ratio(
        ASPECT_RATIO, 
        WIDTH, 
        FILEPATH.to_string(), 
        SAMPLES_PER_PIXEL, 
        MAX_DEPTH
    );


    // Camera 
    let mut camera = Camera::new(VERTICAL_FOV, image_info.clone());
    camera.set(LOOK_FROM, LOOK_AT, (LOOK_AT - LOOK_FROM).length(), 0., UP);

    // Output settings
    let mut writter: Box<dyn Writter> = Box::new(PpmWritter::new(image_info.clone()));
    writter.try_open()?;

    // Rendering
    camera.render(&world, &mut *writter);

    // Saving
    let saving_start = std::time::Instant::now();
    writter.save()?;
    print!("\rSaving... Saving done in {:.2}s.\n", saving_start.elapsed().as_secs_f64());

    Ok(())
}
