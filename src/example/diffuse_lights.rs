
use raytracing::camera::Camera;
use raytracing::material::*;
use raytracing::writter::{Writter, GeneralWritter};
use raytracing::vector::{Point, Vec3};
use raytracing::hittable::{HittableList, Sphere, Quad};
use raytracing::image_info::ImageInfo;
use raytracing::terminal::{Terminal, Position};
use raytracing::texture::NoiseTexture;
use raytracing::color::Color;

fn main() -> Result<(), std::io::Error> {
    diffuse_lights()
}

pub fn diffuse_lights() -> Result<(), std::io::Error> {
    // Constants
    const FILEPATH: &str = "output/diffuse_lights.png";
    const WIDTH: usize = 400;
    const ASPECT_RATIO: f64 = 16.0 / 9.0;

    const SAMPLES_PER_PIXEL: usize = 1000;
    const MAX_DEPTH: usize = 50;

    const LIGHT_COLOR: Color = Color { r: 5., g: 5., b: 5. };
    const BACKGROUND_COLOR: Color = Color { r: 0., g: 0., b: 0. };
    
    const VERTICAL_FOV: f64 = 20.0;
    const LOOK_FROM: Point = Point::new(26.,3.,6.);
    const LOOK_AT: Point = Point::new(0., 2., 0.);
    const DEFOCUS_ANGLE: f64 = 0.;
    const UP: Point = Point::new(0., 1., 0.);
    let focus_distance = (LOOK_FROM - LOOK_AT).length();

    welcome_message();

    // Textures
    let noise_texture = NoiseTexture::new(4.);
    let diff_light = DiffuseLight::from_color(LIGHT_COLOR);

    // Materials
    let noise_surface = Lambertian::from_texture(noise_texture);

    // World
    let mut world = HittableList::new();
    world.add(Sphere::new(Point::new(0., -1000., 0.), 1000., noise_surface.clone()));
    world.add(Sphere::new(Point::new(0., 2., 0.), 2., noise_surface));

    world.add(Quad::new(Point::new(3., 1., -2.), Vec3::new(2., 0., 0.), Vec3::new(0., 2., 0.), diff_light.clone()));
    world.add(Sphere::new(Point::new(0., 7., 0.), 2., diff_light));
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
    camera.set_background(BACKGROUND_COLOR);

    // Output settings
    let mut writter: Box<dyn Writter> = Box::new(GeneralWritter::new(image_info.clone()));
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