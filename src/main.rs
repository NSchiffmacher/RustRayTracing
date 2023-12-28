
use raytracing::camera::Camera;
use raytracing::writter::{Writter, PpmWritter};
use raytracing::vector::Point;
use raytracing::hittable::{HittableList, Sphere};


fn main() -> Result<(), std::io::Error> {
    // Constants
    let aspect_ratio = 16.0 / 9.0;
    let width = 800;
    let viewport_height = 2.0;

    // Image settings
    let height = ((width as f64) / aspect_ratio) as usize;

    // Output settings
    let mut writter: Box<dyn Writter> = Box::new(PpmWritter::new("output/test.ppm".to_string(), (width, height)));
    writter.try_open()?;

    // Objects
    let mut world = HittableList::new();
    world.add(Sphere::boxed(Point::new(0., 0., -1.), 0.5));
    world.add(Sphere::boxed(Point::new(0., -100.5, -1.), 100.));

    // Camera 
    let camera_center = Point::new(0., 0., 0.);
    let focal_length = 1.;
    let samples_per_pixel = 10;
    let mut camera = Camera::new(focal_length, camera_center, samples_per_pixel, aspect_ratio, width, viewport_height);

    // Rendering
    camera.render(&world, &mut *writter);

    // Saving
    let saving_start = std::time::Instant::now();
    writter.save()?;
    print!("\rSaving... Saving done in {:.2}s.\n", saving_start.elapsed().as_secs_f64());

    Ok(())
}
