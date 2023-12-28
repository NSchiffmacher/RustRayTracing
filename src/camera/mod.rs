use crate::ray::Ray;
use crate::hittable::HittableList;
use crate::interval::Interval;
use crate::color::Color;
use crate::vector::{Vec3, Point};
use crate::writter::Writter;

use rand::Rng;

pub struct Camera {
    focal_length: f64,
    aspect_ratio: f64,
    samples_per_pixel: usize,

    rng: rand::rngs::ThreadRng,
    
    image_height: usize,
    image_width: usize,

    viewport_height: f64,
    viewport_width: f64,

    camera_center: Point,

    viewport_u: Vec3,
    viewport_v: Vec3,

    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,

    viewport_upper_left: Vec3,
    pixel00_loc: Vec3,
}

impl Camera {
    pub fn new(focal_length: f64, camera_center: Point, samples_per_pixel: usize, aspect_ratio: f64, image_width: usize, viewport_height: f64) -> Self {
        let image_height = ((image_width as f64) / aspect_ratio) as usize;
        let viewport_width = viewport_height * (image_width as f64) / (image_height as f64);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0); // Horizontal vector
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0); // Vertical vector
    
        let pixel_delta_u = viewport_u / (image_width as f64);
        let pixel_delta_v = viewport_v / (image_height as f64);
    
        let viewport_upper_left = camera_center - Vec3::new(0.0, 0.0, focal_length) - (viewport_u / 2.0) - (viewport_v / 2.0);
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5; // Center of the first pixel
    
        Self {
            focal_length,
            aspect_ratio,
            samples_per_pixel,

            rng: rand::thread_rng(),

            image_height,
            image_width,

            viewport_height,
            viewport_width,

            camera_center,

            viewport_u,
            viewport_v,

            pixel_delta_u,
            pixel_delta_v,

            viewport_upper_left,
            pixel00_loc,
        }
    }

    pub fn render(&mut self, world: &HittableList, writter: &mut dyn Writter) {
        print!("Starting rendering...");
        let rendering_start = std::time::Instant::now();
        
        for y in 0..self.image_height {
            print!("\rStarting rendering... {:.2}%    ", 100. * ((self.image_height * y + 0) as f64) / ((self.image_width * self.image_height) as f64));
            for x in 0..self.image_width {
                let mut color_vec = Vec3::new(0., 0., 0.);

                for _sample in 0..self.samples_per_pixel {
                    let ray = self.get_ray(x, y);
                    color_vec += self.ray_color(&ray, &world).to_vec();
                }
                writter.set_at((x, y), Color::from_vec(&(color_vec / (self.samples_per_pixel as f64))));
            }
        }

        print!("\rStarting rendering... Done in {:.2}s.\nSaving...", rendering_start.elapsed().as_secs_f64());

    }

    fn get_ray(&mut self, x: usize, y: usize) -> Ray {
        let pixel_center = self.pixel00_loc + (self.pixel_delta_u * (x as f64)) + (self.pixel_delta_v * (y as f64));
        let pixel_sample = pixel_center + self.pixel_random_square();

        let ray_direction = pixel_sample - self.camera_center;
        Ray::new(self.camera_center, ray_direction)
    }

    fn pixel_random_square(&mut self) -> Vec3 {
        let px = self.rng.gen_range(-0.5..0.5);
        let py = self.rng.gen_range(-0.5..0.5);

        self.pixel_delta_u * px + self.pixel_delta_v * py
    }
    
    fn ray_color(&self, ray: &Ray, world: &HittableList) -> Color {
        if let Some(hit_record) = world.hit(ray, &Interval::positive()) {
            let v = (hit_record.normal + Vec3::new(1., 1., 1.)) * 0.5;
            return Color::from_vec(&v);
        }
    
        let unit_direction = ray.direction().normalized();
        let a = 0.5 * (unit_direction.y() + 1.0);
    
        Color::white().lerp(&Color::new(0.5, 0.7, 1.0), a)
    }
}