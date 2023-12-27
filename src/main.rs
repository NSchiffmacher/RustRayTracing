
use raytracing::color::Color;
use raytracing::writter::{Writter, PpmWritter};
use raytracing::ray::Ray;
use raytracing::vector::{Point, Vec3};

fn ray_color(ray: &Ray) -> Color {
    let sphere_center = Point::new(0., 0., -1.);
    let sphere_radius = 0.5;
    if let Some(t) = hit_sphere(&sphere_center, sphere_radius, ray) {
        let hit_point = ray.at(t);
        let normal = (hit_point - sphere_center).normalized();
        let v = (normal + Vec3::new(1., 1., 1.)) * 0.5;
        return Color::from_vec(&v);
    }

    let unit_direction = ray.direction().normalized();
    let a = 0.5 * (unit_direction.y() + 1.0);

    Color::white().lerp(&Color::new(0.5, 0.7, 1.0), a)
}

fn hit_sphere(center: &Point, radius: f64, ray: &Ray) -> Option<f64> {
    let oc = ray.origin() - *center;
    let a = ray.direction().dot(&ray.direction());
    let b_half = ray.direction().dot(&oc);
    let c = oc.dot(&oc) - radius * radius;

    let delta_quarter = b_half * b_half -  a * c;
    if delta_quarter < 0. {
        return None;
    } 
    
    Some((-b_half - delta_quarter.sqrt()) / a)
}

fn main() -> Result<(), std::io::Error> {
    // Constants
    let aspect_ratio = 16.0 / 9.0;
    let width = 400;
    let viewport_height = 2.0;

    // Image settings
    let height = ((width as f64) / aspect_ratio) as usize;

    // Output settings
    let mut writter: Box<dyn Writter> = Box::new(PpmWritter::new("output/test.bpm".to_string(), (width, height)));
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


    // Rendering
    print!("Starting rendering...");
    let rendering_start = std::time::Instant::now();
    
    for y in 0..height {
        print!("\rStarting rendering... {:.2}%    ", 100. * ((height * y + 0) as f64) / ((width * height) as f64));
        for x in 0..width {
            let pixel_center = pixel00_loc + (pixel_delta_u * (x as f64)) + (pixel_delta_v * (y as f64));
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);

            let color = ray_color(&ray);
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
