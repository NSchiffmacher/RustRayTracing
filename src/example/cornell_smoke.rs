
use raytracing::camera::Camera;
use raytracing::material::*;
use raytracing::writter::{Writter, PpmWritter};
use raytracing::vector::{Point, Vec3};
use raytracing::hittable::{HittableList, ConstantMedium, Quad, yaw_rotated_cuboid};
use raytracing::image_info::ImageInfo;
use raytracing::terminal::{Terminal, Position};
use raytracing::color::Color;

fn main() -> Result<(), std::io::Error> {
    cornell_smoke()
}

pub fn cornell_smoke() -> Result<(), std::io::Error> {
    // Constants
    const FILEPATH: &str = "output/cornell_smoke.ppm";
    const WIDTH: usize = 600;
    const ASPECT_RATIO: f64 = 1.;

    const SAMPLES_PER_PIXEL: usize = 2000;
    const MAX_DEPTH: usize = 50;

    const VERTICAL_FOV: f64 = 40.0;
    const LOOK_FROM: Point = Point::new(278., 278., -800.);
    const LOOK_AT: Point = Point::new(278., 278., 0.);
    const DEFOCUS_ANGLE: f64 = 0.;
    const UP: Point = Point::new(0., 1., 0.);
    let focus_distance = (LOOK_FROM - LOOK_AT).length();

    welcome_message();

    // Textures

    // Materials
    let red = Lambertian::new(Color::new(0.65, 0.05, 0.05));
    let green = Lambertian::new(Color::new(0.12, 0.45, 0.15));
    let white = Lambertian::new(Color::new(0.73, 0.73, 0.73));
    let light = DiffuseLight::white(15.);

    // World
    let mut world = HittableList::new();
    world += Quad::new(Point::new(555.,0.,0.), Vec3::new(0.,555.,0.), Vec3::new(0.,0.,555.), green);
    world += Quad::new(Point::new(0.,0.,0.), Vec3::new(0.,555.,0.), Vec3::new(0.,0.,555.), red);
    world += Quad::new(Point::new(343., 554., 332.), Vec3::new(-130.,0.,0.), Vec3::new(0.,0.,-105.), light);
    world += Quad::new(Point::new(0.,0.,0.), Vec3::new(555.,0.,0.), Vec3::new(0.,0.,555.), white.clone());
    world += Quad::new(Point::new(555., 555., 555.), Vec3::new(-555., 0., 0.), Vec3::new(0., 0., -555.), white.clone());
    // world += Quad::new(Point::new(0., 0., 555.), Vec3::new(555., 0., 0.), Vec3::new(0., 555., 0.), white.clone());
    

    world += ConstantMedium::from_color(
        Box::new(yaw_rotated_cuboid(Point::new(212.5, 82.5, 147.5), Vec3::new(165., 165., 165.), -18., white.clone())), 
        0.01,
        Color::black());
    world += ConstantMedium::from_color(
        Box::new(yaw_rotated_cuboid(Point::new(347.5, 165.0, 377.5), Vec3::new(165., 330., 165.), 15., white.clone())), 
        0.01,
        Color::white());
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
    // camera.set_background(BACKGROUND_COLOR);

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