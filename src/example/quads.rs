use raytracing::camera::Camera;
use raytracing::material::*;
use raytracing::writter::{Writter, PpmWritter};
use raytracing::vector::{Point, Vec3};
use raytracing::color::Color;
use raytracing::hittable::{HittableList, Quad};
use raytracing::image_info::ImageInfo;
use raytracing::terminal::{Terminal, Position};

use std::rc::Rc;

fn main() -> Result<(), std::io::Error> {
    quads()
}

pub fn quads() -> Result<(), std::io::Error> {
    // Constants
    const FILEPATH: &str = "output/quads.ppm";
    const WIDTH: usize = 400;
    const ASPECT_RATIO: f64 = 1.;

    const SAMPLES_PER_PIXEL: usize = 100;
    const MAX_DEPTH: usize = 50;
    
    const VERTICAL_FOV: f64 = 80.0;
    const LOOK_FROM: Point = Point::new(0., 0., 9.);
    const LOOK_AT: Point = Point::new(0., 0., 0.);
    const DEFOCUS_ANGLE: f64 = 0.;
    const UP: Point = Point::new(0., 1., 0.);
    let focus_distance = (LOOK_FROM - LOOK_AT).length();

    welcome_message();

    // Textures

    // Materials
    let left_red     = Rc::new(Lambertian::new(Color::new(1.0, 0.2, 0.2)));
    let back_green   = Rc::new(Lambertian::new(Color::new(0.2, 1.0, 0.2)));
    let right_blue   = Rc::new(Lambertian::new(Color::new(0.2, 0.2, 1.0)));
    let upper_orange = Rc::new(Lambertian::new(Color::new(1.0, 0.5, 0.0)));
    let lower_teal   = Rc::new(Lambertian::new(Color::new(0.2, 0.8, 0.8)));

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Quad::new(Point::new(-3.,-2., 5.), Vec3::new(0., 0.,-4.), Vec3::new(0., 4., 0.), left_red)));
    world.add(Box::new(Quad::new(Point::new(-2.,-2., 0.), Vec3::new(4., 0., 0.), Vec3::new(0., 4., 0.), back_green)));
    world.add(Box::new(Quad::new(Point::new( 3.,-2., 1.), Vec3::new(0., 0., 4.), Vec3::new(0., 4., 0.), right_blue)));
    world.add(Box::new(Quad::new(Point::new(-2., 3., 1.), Vec3::new(4., 0., 0.), Vec3::new(0., 0., 4.), upper_orange)));
    world.add(Box::new(Quad::new(Point::new(-2.,-3., 5.), Vec3::new(4., 0., 0.), Vec3::new(0., 0.,-4.), lower_teal)));
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