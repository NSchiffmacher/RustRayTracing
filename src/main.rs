
use raytracing::camera::Camera;
use raytracing::color::Color;
use raytracing::material::Lambertian;
use raytracing::writter::{Writter, PpmWritter};
use raytracing::vector::Point;
use raytracing::hittable::{HittableList, Sphere};
use raytracing::image_info::ImageInfo;

use std::rc::Rc;

fn main() -> Result<(), std::io::Error> {
    // Constants
    let width = 800;
    let aspect_ratio = 16.0 / 9.0;
    let samples_per_pixel = 10;
    let max_depth = 50;
    let viewport_height = 2.0;
    let camera_center = Point::new(0., 0., 0.);
    let focal_length = 1.;
    
    // Image settings
    let image_info = ImageInfo::from_aspect_ratio(aspect_ratio, width, "output/test.ppm".to_string(), samples_per_pixel, max_depth);

    // Output settings
    let mut writter: Box<dyn Writter> = Box::new(PpmWritter::new(image_info.clone()));
    writter.try_open()?;

    // Materials
    let lambertian = Rc::new(Lambertian::new(Color::new(0.7, 0.7, 0.7)));

    // Objects
    let mut world = HittableList::new();
    world.add(Sphere::boxed(Point::new(0., 0., -1.), 0.5, lambertian.clone()));
    world.add(Sphere::boxed(Point::new(0., -100.5, -1.), 100., lambertian.clone()));

    // Camera 
    let mut camera = Camera::new(viewport_height, image_info.clone());
    camera.set(camera_center, focal_length);

    // Rendering
    camera.render(&world, &mut *writter);

    // Saving
    let saving_start = std::time::Instant::now();
    writter.save()?;
    print!("\rSaving... Saving done in {:.2}s.\n", saving_start.elapsed().as_secs_f64());

    Ok(())
}
