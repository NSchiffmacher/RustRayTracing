
use raytracing::color::Color;
use raytracing::writter::{Writter, PpmWritter};
use raytracing::ray::Ray;
use raytracing::vector::{Point, Vec3};
use raytracing::hittable::{HittableList, Sphere};
use raytracing::interval::Interval;
use raytracing::image_info::ImageInfo;
use raytracing::material::Lambertian;

use std::rc::Rc;

fn ray_color(ray: &Ray, world: &HittableList) -> Color {
    if let Some(hit_record) = world.hit(ray, &Interval::positive()) {
        let v = (hit_record.normal + Vec3::new(1., 1., 1.)) * 0.5;
        return Color::from_vec(v);
    }

    let unit_direction = ray.direction().normalized();
    let a = 0.5 * (unit_direction.y() + 1.0);

    Color::white().lerp(&Color::new(0.5, 0.7, 1.0), a)
}

fn main() -> Result<(), std::io::Error> {
    // Constants
    let aspect_ratio = 16.0 / 9.0;
    let width = 400;
    let viewport_height = 2.0;

    // Image settings
    let height = ((width as f64) / aspect_ratio) as usize;

    // Output settings
    let image_info = ImageInfo::new(width, height, "output/weekend/weekend2.ppm".to_string(), 1, 1);
    let mut writter: Box<dyn Writter> = Box::new(PpmWritter::new(image_info.clone()));
    writter.try_open()?;

    // Camera settings
    let focal_length = 1.0;
    let viewport_width = viewport_height * (width as f64) / (height as f64);

    let camera_center = Vec3::zero();

    // Camera internal settings
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0); // Horizontal vector
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0); // Vertical vector

    let pixel_delta_u = viewport_u / (width as f64);
    let pixel_delta_v = viewport_v / (height as f64);

    let viewport_upper_left = camera_center - Vec3::new(0.0, 0.0, focal_length) - (viewport_u / 2.0) - (viewport_v / 2.0);
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5; // Center of the first pixel

    // Materials
    let lambertian = Rc::new(Lambertian::new(Color::new(0.7, 0.7, 0.7)));

    // Objects
    let mut world = HittableList::new();
    world.add(Sphere::boxed(Point::new(0., 0., -1.), 0.5, lambertian.clone()));
    world.add(Sphere::boxed(Point::new(0., -100.5, -1.), 100., lambertian.clone()));


    // Rendering
    print!("Starting rendering...");
    let rendering_start = std::time::Instant::now();
    
    for y in 0..height {
        print!("\rStarting rendering... {:.2}%    ", 100. * ((height * y + 0) as f64) / ((width * height) as f64));
        for x in 0..width {
            let pixel_center = pixel00_loc + (pixel_delta_u * (x as f64)) + (pixel_delta_v * (y as f64));
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction, 0.);

            let color = ray_color(&ray, &world);
            writter.set_at((x, y), color);
        }
    }

    print!("\rStarting rendering... Done in {:.2}s.\nSaving...", rendering_start.elapsed().as_secs_f64());

    // Saving
    let saving_start = std::time::Instant::now();
    writter.save()?;
    print!("\rSaving... Saving done in {:.2}s.\n", saving_start.elapsed().as_secs_f64());

    Ok(())
}
