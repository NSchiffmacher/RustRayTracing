
use raytracing::camera::Camera;
use raytracing::material::*;
use raytracing::texture::{ImageTexture, NoiseTexture};
use raytracing::writter::{Writter, GeneralWritter};
use raytracing::vector::{Point, Vec3};
use raytracing::hittable::{HittableList, ConstantMedium, Quad, Sphere, yaw_rotated_cuboid};
use raytracing::image_info::ImageInfo;
use raytracing::terminal::{Terminal, Position};
use raytracing::color::Color;

use rand::Rng;

fn main() -> Result<(), std::io::Error> {
    final_scene()
}

pub fn final_scene() -> Result<(), std::io::Error> {
    // Constants
    const FILEPATH: &str = "output/test.png";
    const WIDTH: usize = 500;
    const ASPECT_RATIO: f64 = 1.;

    const SAMPLES_PER_PIXEL: usize = 100;
    const MAX_DEPTH: usize = 50;

    const BACKGROUND_COLOR: Color = Color { r: 0., g: 0., b: 0.};

    const VERTICAL_FOV: f64 = 40.0;
    const LOOK_FROM: Point = Point::new(478., 278., -600.);
    const LOOK_AT: Point = Point::new(278., 278., 0.);
    const DEFOCUS_ANGLE: f64 = 0.;
    const UP: Point = Point::new(0., 1., 0.);
    let focus_distance = (LOOK_FROM - LOOK_AT).length();

    welcome_message();

    let mut rng = rand::thread_rng();

    // Textures
    let earth_texture = ImageTexture::new("assets/earthmap.jpg").expect("Earth texture not found");
    let noise_texture = NoiseTexture::new(0.1);

    // Materials
    let ground = Lambertian::from_rgb(0.48, 0.83, 0.53);
    let light = DiffuseLight::white(7.);
    let sphere_material = Lambertian::from_rgb(0.7, 0.3, 0.1);
    let earth_material = Lambertian::from_texture(earth_texture);
    let noise_material = Lambertian::from_texture(noise_texture);

    let boxes_per_side = 20;
    // World
    let mut world = HittableList::new();

    // Boxes
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.;
            let size = Vec3::new(w, rng.gen_range(1.0..101.0), w);
            let position = Vec3::new(-1000. + i as f64 * w, 0., -1000. + j as f64 * w) + size / 2.;

            world += yaw_rotated_cuboid(position, size, 0., ground.clone());
        }
    }

    // Light
    world += Quad::new(Point::new(123., 554., 147.), Vec3::new(300., 0., 0.), Vec3::new(0., 0., 265.), light);

    // Moving sphere
    let center1 = Point::new(400., 400., 200.);
    let center2 = center1 + Vec3::new(30., 0., 0.);
    world += Sphere::new_moving(center1, center2, 50., sphere_material);

    world += Sphere::new(Point::new(260., 150., 45.), 50., Dielectric::new(1.5));
    world += Sphere::new(Point::new(0., 150., 145.), 50., Metal::new(Color::new(0.8, 0.8, 0.9), 1.));

    // Foggy Dielectric sphere
    let boundary = Sphere::new(Point::new(360., 150., 145.), 70., Dielectric::new(1.5));
    world += boundary.clone();
    world += ConstantMedium::from_color(boundary, 0.2, Color::new(0.2, 0.4, 0.9));

    let boundary = Sphere::new(Point::new(0., 0., 0.), 5000., Dielectric::new(1.5));
    world += ConstantMedium::from_color(boundary, 0.0001, Color::new(1., 1., 1.));

    // Earth sphere
    world += Sphere::new(Point::new(400., 200., 400.), 100., earth_material);

    // Noise sphere
    world += Sphere::new(Point::new(220., 280., 300.), 80., noise_material);

    // Idk
    let white = Lambertian::from_rgb(0.73, 0.73, 0.73);
    let ns = 1000;
    let translation = Vec3::new(-100., 270., 395.);
    let rotation_rad = 15f64.to_radians();
    let (s, c) = rotation_rad.sin_cos();
    for _ in 0..ns {
        let random = Point::random(0., 165.);
        let rotated_random = Vec3::new(
            random.x() * c + random.z() * s,
            random.y(),
            random.z() * c - random.x() * s
        );
        let center = translation + rotated_random;
        world += Sphere::new(center, 10., white.clone());
    }

    // Convert world to BVH tree
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