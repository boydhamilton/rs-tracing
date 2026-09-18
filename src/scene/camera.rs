use std::io::Write;

use rand::Rng;
use rayon::prelude::*;

use super::colour::{write_colour, Colour};
use super::hittable::Hittable;
use super::ray::Ray;
use super::vec3::{random_unit_vector, unit_vector, Vec3};

const MAX_DEPTH: i32 = 50;

pub struct Camera {
    image_width: i32,
    image_height: i32,
    samples_per_pixel: i32,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i32, samples_per_pixel: i32) -> Camera {
        let image_height = ((image_width as f64 / aspect_ratio) as i32).max(1);

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let center = Vec3::new(0.0, 0.0, 0.0);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        Camera {
            image_width,
            image_height,
            samples_per_pixel,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render<W: Write>(&self, world: &(dyn Hittable + Sync), writer: &mut W) {
        writeln!(writer, "P3\n{} {}\n255", self.image_width, self.image_height).unwrap();

        // Rows are independent, so render them in parallel and write the
        // resulting buffers out in order afterward (PPM output must stay ordered).
        let rows: Vec<Vec<u8>> = (0..self.image_height)
            .into_par_iter()
            .map(|j| {
                let mut row_buf = Vec::new();
                for i in 0..self.image_width {
                    let mut pixel_colour = Vec3::new(0.0, 0.0, 0.0);
                    for _ in 0..self.samples_per_pixel {
                        let ray = self.get_ray(i, j);
                        pixel_colour = pixel_colour + Self::ray_colour(&ray, MAX_DEPTH, world).0;
                    }
                    write_colour(&mut row_buf, Colour(pixel_colour / self.samples_per_pixel as f64));
                }
                row_buf
            })
            .collect();

        for row in rows {
            writer.write_all(&row).unwrap();
        }
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let mut rng = rand::thread_rng();
        let offset_x: f64 = rng.gen_range(-0.5..0.5);
        let offset_y: f64 = rng.gen_range(-0.5..0.5);

        let pixel_sample = self.pixel00_loc
            + self.pixel_delta_u * (i as f64 + offset_x)
            + self.pixel_delta_v * (j as f64 + offset_y);

        Ray::new(self.center, pixel_sample - self.center)
    }

    fn ray_colour(ray: &Ray, depth: i32, world: &(dyn Hittable + Sync)) -> Colour {
        if depth <= 0 {
            return Colour(Vec3::new(0.0, 0.0, 0.0));
        }

        if let Some(rec) = world.hit(ray, 0.001, f64::INFINITY) {
            let scatter_direction = rec.normal + random_unit_vector();
            let scattered = Ray::new(rec.p, scatter_direction);
            return Colour(Self::ray_colour(&scattered, depth - 1, world).0 * 0.5);
        }

        let unit_direction = unit_vector(ray.direction());
        let a = 0.5 * (unit_direction.y + 1.0);
        Colour(Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a)
    }
}
