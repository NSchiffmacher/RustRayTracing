use raytracing::camera::Camera;
use raytracing::material::*;
use raytracing::writter::{Writter, PpmWritter};
use raytracing::vector::Point;
use raytracing::hittable::{HittableList, Sphere};
use raytracing::image_info::ImageInfo;
use raytracing::terminal::{Terminal, Position};
use raytracing::texture::NoiseTexture;

use std::rc::Rc;

fn main() -> Result<(), std::io::Error> {
    perlin_noise()
}

pub fn perlin_noise() -> Result<(), std::io::Error> {
    // Constants
    const FILEPATH: &str = "output/perlin_noise.ppm";
    const WIDTH: usize = 400;
    const ASPECT_RATIO: f64 = 16.0 / 9.0;

    const SAMPLES_PER_PIXEL: usize = 100;
    const MAX_DEPTH: usize = 50;
    
    const VERTICAL_FOV: f64 = 20.0;
    const LOOK_FROM: Point = Point::new(13.,2.,3.);
    const LOOK_AT: Point = Point::new(0., 0., 0.);
    const DEFOCUS_ANGLE: f64 = 0.;
    const UP: Point = Point::new(0., 1., 0.);
    let focus_distance = (LOOK_FROM - LOOK_AT).length();

    welcome_message();

    // Textures
    let noise_texture = Rc::new(NoiseTexture::new(4.));

    // Materials
    let noise_surface = Rc::new(Lambertian::from_texture(noise_texture));

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point::new(0., -1000., 0.), 1000., noise_surface.clone())));
    world.add(Box::new(Sphere::new(Point::new(0., 2., 0.), 2., noise_surface)));
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
    camera.set(LOOK_FROM, LOOK_AT, focus_distance, DEFOCUS_ANGLE, UP);

    // Output settings
    let mut writter: Box<dyn Writter> = Box::new(PpmWritter::new(image_info.clone()));
    writter.try_open()?;

    Terminal::cursor_position(&Position{ x: 2, y: 5});
    println!("* Rendering image \"{}\"\r", FILEPATH);

    // Rendering
    camera.render(&world, &mut *writter);

    // Saving
    writter.save()?;

    Ok(())
}

fn welcome_message() {
    Terminal::clear_screen();
    Terminal::cursor_position(&Position{ x: 0, y: 1});
    println!("{}\r", Terminal::repeated('='));
    println!("{}\r", Terminal::centered(" Shitty Raytracer ", '='));
    println!("{}\r", Terminal::repeated('='));
}