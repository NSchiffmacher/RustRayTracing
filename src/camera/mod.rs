use crate::ray::Ray;
use crate::hittable::HittableList;
use crate::interval::Interval;
use crate::color::Color;
use crate::vector::{Vec3, Point};
use crate::writter::Writter;
use crate::image_info::ImageInfo;

use rand::Rng;

pub struct Camera {
    focal_length: f64,
    camera_center: Point,
    image_info: ImageInfo,

    rng: rand::rngs::ThreadRng,
    
    // viewport_height: f64,
    // viewport_width: f64,

    viewport_u: Vec3,
    viewport_v: Vec3,

    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,

    viewport_upper_left: Vec3,
    pixel00_loc: Vec3,
}

impl Camera {
    pub fn new(viewport_height: f64, image_info: ImageInfo) -> Self {
        let viewport_width = viewport_height * (image_info.width as f64) / (image_info.height as f64);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0); // Horizontal vector
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0); // Vertical vector
    
        let pixel_delta_u = viewport_u / (image_info.width as f64);
        let pixel_delta_v = viewport_v / (image_info.height as f64);
    
        let mut camera = Self {
            focal_length: 1.,
            camera_center: Point::zero(),
            image_info,

            rng: rand::thread_rng(),

            // viewport_height,
            // viewport_width,

            viewport_u,
            viewport_v,

            pixel_delta_u,
            pixel_delta_v,

            viewport_upper_left: Vec3::zero(),
            pixel00_loc: Vec3::zero(),
        };
        camera.set(Point::new(0., 0., 0.), 1.);
        camera
    }

    pub fn set(&mut self, position: Point, focal_length: f64) {
        self.camera_center = position;
        self.focal_length = focal_length;

        self.viewport_upper_left = self.camera_center - Vec3::new(0.0, 0.0, self.focal_length) - (self.viewport_u / 2.0) - (self.viewport_v / 2.0);
        self.pixel00_loc = self.viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5; // Center of the first pixel
    }

    pub fn render(&mut self, world: &HittableList, writter: &mut dyn Writter) {
        print!("Starting rendering...");
        let rendering_start = std::time::Instant::now();
        
        for y in 0..self.image_info.height {
            print!("\rStarting rendering... {:.2}%    ", 100. * ((self.image_info.width * y + 0) as f64) / ((self.image_info.width * self.image_info.height) as f64));
            for x in 0..self.image_info.width {
                let mut color_vec = Vec3::new(0., 0., 0.);

                for _sample in 0..self.image_info.samples_per_pixel {
                    let ray = self.get_ray(x, y);
                    color_vec += self.ray_color(&ray, &world).to_vec();
                }

                // Apply gamma correction
                color_vec /= self.image_info.samples_per_pixel as f64;
                color_vec = Vec3::new(color_vec.x().sqrt(), color_vec.y().sqrt(), color_vec.z().sqrt());

                writter.set_at((x, y), Color::from_vec(color_vec));
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

    fn ray_color_rec(&mut self, ray: &Ray, world: &HittableList, depth: usize) -> Vec3 {
        if depth <= 0 {
            return Vec3::zero();
        }

        if let Some(hit_record) = world.hit(ray, &Interval::positive()) {
            // let v = (hit_record.normal + Vec3::new(1., 1., 1.)) * 0.5;
            // let random = Vec3::random_vector_in_hemisphere(&hit_record.normal, &mut self.rng);
            let direction = hit_record.normal + Vec3::random_unit_vector(&mut self.rng);
            return self.ray_color_rec(&Ray::new(hit_record.point, direction), world, depth - 1) * 0.7;
        }
    
        let unit_direction = ray.direction().normalized();
        let a = 0.5 * (unit_direction.y() + 1.0);
        
        Vec3::new(1., 1., 1.) * (1. - a) + Vec3::new(0.5, 0.7, 1.0) * a
    }
    
    fn ray_color(&mut self, ray: &Ray, world: &HittableList) -> Color {
        Color::from_vec(self.ray_color_rec(ray, world, self.image_info.max_depth))
    }
}