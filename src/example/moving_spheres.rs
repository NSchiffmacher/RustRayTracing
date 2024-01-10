use raytracing::camera::Camera;
use raytracing::color::Color;
use raytracing::material::*;
use raytracing::writter::{Writter, PpmWritter};
use raytracing::vector::Point;
use raytracing::hittable::{HittableList, Sphere};
use raytracing::image_info::ImageInfo;
use rand::Rng;

use std::rc::Rc;

fn main() -> Result<(), std::io::Error> {
    moving_spheres()
}

pub fn moving_spheres() -> Result<(), std::io::Error> {
    // Constants
    const FILEPATH: &str = "output/moving_spheres.ppm";
    const WIDTH: usize = 1600;
    const ASPECT_RATIO: f64 = 16.0 / 9.0;

    const SAMPLES_PER_PIXEL: usize = 100;
    const MAX_DEPTH: usize = 50;
    
    const VERTICAL_FOV: f64 = 20.0;
    const LOOK_FROM: Point = Point::new(13.,2.,3.);
    const LOOK_AT: Point = Point::new(0., 0., 0.);
    const DEFOCUS_ANGLE: f64 = 1.0;
    const FOCUS_DISTANCE: f64 = 10.0;
    const UP: Point = Point::new(0., 1., 0.);

    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    let mut world = HittableList::new();
    let mut rng = rand::thread_rng();

    world.add(Sphere::boxed(Point::new(0., -1000., 0.), 1000., ground_material));

    for a in -11..10 {
        for b in -11..10 {
            let choose_mat = rng.gen_range(0f64..1f64);
            let center = Point::new(
                a as f64 + 0.9 * rng.gen_range(0f64..1f64),
                0.2, 
                b as f64 + 0.9 * rng.gen_range(0f64..1f64)
            );

            if (center - Point::new(4., 0.2, 0.)).length() > 0.9 {
                let material: Rc<dyn Material> = if choose_mat < 0.8 {
                    // diffuse
                    let color_a = Color::new(rng.gen_range(0f64..1f64), rng.gen_range(0f64..1f64), rng.gen_range(0f64..1f64));
                    let color_b = Color::new(rng.gen_range(0f64..1f64), rng.gen_range(0f64..1f64), rng.gen_range(0f64..1f64));
                    let albedo = color_a * color_b;
                    Lambertian::new(albedo)
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::new(rng.gen_range(0.5..1f64), rng.gen_range(0.5..1f64), rng.gen_range(0.5..1f64));
                    let fuzz = rng.gen_range(0f64..0.5);
                    Metal::new(albedo, fuzz)
                } else {
                    // glass
                    Dielectric::new(1.5)
                };

                let moving_proba = rng.gen_range(0f64..1f64);
                if moving_proba < 0.2 {
                    let final_center = center + Point::new(0., rng.gen_range(0f64..0.5), 0.);
                    world.add(Sphere::boxed_moving(center, final_center, 0.2, material))
                } else {
                    world.add(Sphere::boxed(center, 0.2, material));
                }
            }
        }
    }

    let mat1 = Dielectric::new(1.5);
    world.add(Sphere::boxed(Point::new(0., 1., 0.), 1., mat1));

    let mat2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.add(Sphere::boxed(Point::new(-4., 1., 0.), 1., mat2));

    let mat3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.);
    world.add(Sphere::boxed(Point::new(4., 1., 0.), 1., mat3));

    let world = world.to_bvh();
    
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
    camera.set(LOOK_FROM, LOOK_AT, FOCUS_DISTANCE, DEFOCUS_ANGLE, UP);

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
