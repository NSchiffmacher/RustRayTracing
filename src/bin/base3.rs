
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
    let width = 800;
    let aspect_ratio = 16.0 / 9.0;
    let samples_per_pixel = 10;
    let max_depth = 75;
    let camera_center = Point::new(0., 0., 0.);
    let vfov = 80.0;
    let focal_length = 1.;
    
    // Image settings
    let image_info = ImageInfo::from_aspect_ratio(aspect_ratio, width, "output/base3.ppm".to_string(), samples_per_pixel, max_depth);

    // Output settings
    let mut writter: Box<dyn Writter> = Box::new(PpmWritter::new(image_info.clone()));
    writter.try_open()?;

    // Materials
    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    // Objects
    let mut world = HittableList::new();
    world.add(Sphere::boxed(Point::new(0., -100.5, -1.), 100., material_ground.clone()));
    world.add(Sphere::boxed(Point::new(0., 0., -1.), 0.5, material_center.clone()));
    world.add(Sphere::boxed(Point::new(-1., 0., -1.), 0.5, material_left.clone()));
    world.add(Sphere::boxed(Point::new(-1., 0., -1.), -0.4, material_left.clone()));
    world.add(Sphere::boxed(Point::new(1., 0., -1.), 0.5, material_right.clone()));

    // Camera 
    let mut camera = Camera::new(vfov, image_info.clone());
    camera.set(camera_center, focal_length);

    // Rendering
    camera.render(&world, &mut *writter);

    // Saving
    let saving_start = std::time::Instant::now();
    writter.save()?;
    print!("\rSaving... Saving done in {:.2}s.\n", saving_start.elapsed().as_secs_f64());

    Ok(())
}
