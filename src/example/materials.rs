use raytracing::camera::Camera;
use raytracing::color::Color;
use raytracing::material::*;
use raytracing::writter::{Writter, GeneralWritter};
use raytracing::vector::Point;
use raytracing::hittable::{HittableList, Sphere};
use raytracing::image_info::ImageInfo;

fn main() -> Result<(), std::io::Error> {
    materials()
}

pub fn materials() -> Result<(), std::io::Error> {
    // Constants
    const FILEPATH: &str = "output/materials.png";
    const WIDTH: usize = 800;
    const ASPECT_RATIO: f64 = 16.0 / 9.0;

    const SAMPLES_PER_PIXEL: usize = 100;
    const MAX_DEPTH: usize = 50;
    
    const VERTICAL_FOV: f64 = 80.0;
    const LOOK_FROM: Point = Point::new(0., 0., 0.);
    const LOOK_AT: Point = Point::new(0., 0., -1.);
    const DEFOCUS_ANGLE: f64 = 0.0;
    const FOCUS_DISTANCE: f64 = 1.0;
    const UP: Point = Point::new(0., 1., 0.);
    
    // Image settings
    let image_info = ImageInfo::from_aspect_ratio(ASPECT_RATIO, WIDTH, FILEPATH.to_string(), SAMPLES_PER_PIXEL, MAX_DEPTH);

    // Output settings
    let mut writter: Box<dyn Writter> = Box::new(GeneralWritter::new(image_info.clone()));
    writter.try_open()?;

    // Materials
    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.5);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 0.0);

    // Objects
    let mut world = HittableList::new();
    world.add(Sphere::new(Point::new(0., -100.5, -1.), 100., material_ground.clone()));
    world.add(Sphere::new(Point::new(0., 0., -1.), 0.5, material_center.clone()));
    world.add(Sphere::new(Point::new(-1., 0., -1.), 0.5, material_left.clone()));
    world.add(Sphere::new(Point::new(-1., 0., -1.), -0.4, material_left.clone()));
    world.add(Sphere::new(Point::new(1., 0., -1.), 0.5, material_right.clone()));

    // Camera 
    let mut camera = Camera::new(VERTICAL_FOV, image_info.clone());
    camera.set(
        LOOK_FROM, 
        LOOK_AT, 
        FOCUS_DISTANCE, 
        DEFOCUS_ANGLE,
        UP);

    // Rendering
    camera.render(&world, &mut *writter);

    // Saving
    let saving_start = std::time::Instant::now();
    writter.save()?;
    print!("\rSaving... Saving done in {:.2}s.\n", saving_start.elapsed().as_secs_f64());

    Ok(())
}
