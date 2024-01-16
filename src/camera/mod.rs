use crate::ray::Ray;
use crate::hittable::{HittableList, Hittable};
use crate::interval::Interval;
use crate::color::Color;
use crate::vector::{Vec3, Point};
use crate::writter::Writter;
use crate::image_info::ImageInfo;

use rand::{Rng, seq::SliceRandom};
use std::io::Write;
use indicatif::{ProgressStyle, ProgressBar};
use itertools::Itertools;

pub struct Camera {
    focus_distance: f64,
    defocus_angle: f64,

    camera_center: Point,
    vertical_fov: f64, // in radians

    image_info: ImageInfo,

    background: Option<Color>,
    
    // viewport_height: f64,
    // viewport_width: f64,

    viewport_u: Vec3,
    viewport_v: Vec3,

    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,

    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,

    viewport_upper_left: Vec3,
    pixel00_loc: Vec3,

    pub print_progress: bool,
    pub shuffle_rendering: bool,
}

impl Camera {
    pub fn new(vfov_degrees: f64, image_info: ImageInfo) -> Self {
        let vfov = vfov_degrees.to_radians();
        let mut camera = Self {
            focus_distance: 1.,
            defocus_angle: 0.,

            camera_center: Point::zero(),
            vertical_fov: vfov,
            image_info,

            background: None,

            viewport_u: Vec3::zero(), // Set in the call to set()
            viewport_v: Vec3::zero(), // Set in the call to set()

            defocus_disk_u: Vec3::zero(), // Set in the call to set()
            defocus_disk_v: Vec3::zero(), // Set in the call to set()

            pixel_delta_u: Vec3::zero(), // Set in the call to set()
            pixel_delta_v: Vec3::zero(), // Set in the call to set()

            viewport_upper_left: Vec3::zero(), // Set in the call to set()
            pixel00_loc: Vec3::zero(),

            print_progress: true,
            shuffle_rendering: false,
        };
        camera.set(Point::new(0., 0., -1.), Point::new(0., 0., 0.), 1., 0., Vec3::new(0., 1., 0.));
        camera
    }

    pub fn set(&mut self, look_from: Point, look_at: Point, focus_distance: f64, defocus_angle: f64, up: Vec3) {
        self.focus_distance = focus_distance;
        self.defocus_angle = defocus_angle;

        // Compute the viewport dimensions from the fov
        let h = (self.vertical_fov / 2.0).tan();
        let viewport_height = 2. * h * self.focus_distance;
        let viewport_width = viewport_height * (self.image_info.width as f64) / (self.image_info.height as f64);

        // Calculate the base vectors
        let w = (look_from - look_at).normalized();
        let u = up.cross(&w).normalized();
        let v = w.cross(&u);

        self.viewport_u = viewport_width * u;
        self.viewport_v = - viewport_height * v;
    
        self.pixel_delta_u = self.viewport_u / (self.image_info.width as f64);
        self.pixel_delta_v = self.viewport_v / (self.image_info.height as f64);

        let defocus_radius = self.focus_distance * (self.defocus_angle / 2.).to_radians().tan();
        self.defocus_disk_u = defocus_radius * u;
        self.defocus_disk_v = defocus_radius * v;

        self.camera_center = look_from;

        self.viewport_upper_left = self.camera_center - (self.focus_distance * w) - (self.viewport_u + self.viewport_v) / 2.;
        self.pixel00_loc = self.viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5; // Center of the first pixel
    }

    pub fn set_background(&mut self, color: Color) {
        self.background = Some(color);
    }

    pub fn render(&mut self, world: &HittableList, writter: &mut dyn Writter) {
        let progress_bar = if self.print_progress {
            let progress_style = ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.green/blue}] {percent}% ({eta_precise})")
                .unwrap()
                .progress_chars("=>-");
            Some(ProgressBar::new((self.image_info.width * self.image_info.height) as u64).with_style(progress_style))
        } else {
            None
        };
        let mut xs_ys = (0..self.image_info.width).cartesian_product(0..self.image_info.height).collect_vec();
        let mut rng = rand::thread_rng();

        // use of xs_ys just so that we can completely shuffle the rendering, which is usefull for the "live" rendering project
        if self.shuffle_rendering {
            xs_ys.shuffle(&mut rng);
        }

        let rendering_start = std::time::Instant::now();
        for (x, y) in xs_ys {
            let mut color = Color::black();
            
            for _sample in 0..self.image_info.samples_per_pixel {
                let ray = self.get_ray(x, y);
                color += self.ray_color(&ray, &world, self.image_info.max_depth)
            }
            
            // Apply gamma correction
            color *= 1. / (self.image_info.samples_per_pixel as f64);
            color = Color::new(color.r.sqrt(), color.g.sqrt(), color.b.sqrt());
            
            writter.set_at((x, y), color);
            if let Some(progress) = &progress_bar {
                progress.inc(1);
            }
        }

        if let Some(progress) = &progress_bar {
            progress.finish();
            println!("Done rendering in {:.2}s.\r", rendering_start.elapsed().as_secs_f64());
            std::io::stdout().flush().unwrap();
        }
    }

    fn get_ray(&mut self, x: usize, y: usize) -> Ray {
        let mut rng = rand::thread_rng();

        let pixel_center = self.pixel00_loc + (self.pixel_delta_u * (x as f64)) + (self.pixel_delta_v * (y as f64));
        let pixel_sample = pixel_center + self.pixel_random_square();

        let ray_origin: Vec3 = if self.defocus_angle <= 0.0 {
            self.camera_center
        } else {
            self.defocus_disk_sample()
        };
        let ray_time = rng.gen_range(0.0..1.0);

        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction, ray_time)
    }

    fn defocus_disk_sample(&self) -> Point {
        // Returns a random point in the camera defocus disk.
        let p = Point::random_in_disk();
        self.camera_center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
    }

    fn pixel_random_square(&mut self) -> Point {
        let mut rng = rand::thread_rng();
        let px = rng.gen_range(-0.5..0.5);
        let py = rng.gen_range(-0.5..0.5);

        self.pixel_delta_u * px + self.pixel_delta_v * py
    }
    
    fn ray_color(&mut self, ray: &Ray, world: &HittableList, depth: usize) -> Color {
        if depth <= 0 {
            return Color::black();
        }

        if let Some(hit_record) = world.hit(ray, &Interval::positive()) {
            let color_from_emission = hit_record.material.emitted(&hit_record.uv, &hit_record.point);
            if let Some((attenuation, scattered_ray)) = hit_record.material.scatter(ray, &hit_record) {
                let color_from_scatter = attenuation * self.ray_color(&scattered_ray, world, depth - 1);
                return color_from_emission + color_from_scatter;
            } 
            return color_from_emission;
        }
    
        if let Some(color) = &self.background {
            color.clone()
        } else {
            // Sky color
            let unit_direction = ray.direction().normalized();
            let a = 0.5 * (unit_direction.y() + 1.0);
            Color::white().lerp(&Color::new(0.5, 0.7, 1.0), a)
        }
    }
}