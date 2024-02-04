use image::{ImageBuffer, Rgb};
use indicatif::ProgressBar;
use nalgebra::{Point3, Vector3};

use crate::{
    hittable_list::HittableList, hittable_trait::HittableTrait, interval::Interval,
    material::MaterialTrait, random::rng_unit_vec, ray::Ray,
};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    image_height: u32,
    center: Point3<f64>,
    pixel00_loc: Point3<f64>,
    delta_u: Vector3<f64>,
    delta_v: Vector3<f64>,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        samples_per_pixel: u32,
        max_depth: u32,
    ) -> Self {
        let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;

        let center: na::Point3<f64> = na::Point3::new(0., 0., 0.);

        // Viewport Dimentions
        let focal_length: f64 = 1.0;
        let viewport_height: f64 = 2.;
        let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);

        // Refrence basis for viewport
        let viewport_u: Vector3<f64> = Vector3::new(viewport_width, 0., 0.);
        let viewport_v: Vector3<f64> = Vector3::new(0., -viewport_height, 0.);

        // Delta vectors
        let delta_u: Vector3<f64> = viewport_u / image_width as f64;
        let delta_v: Vector3<f64> = viewport_v / image_height as f64;

        // Location of upper left
        let viewport_upper_left =
            center - Vector3::new(0., 0., focal_length) - (viewport_u / 2.) - (viewport_v / 2.);
        let pixel00_loc = viewport_upper_left + 0.5 * (delta_u + delta_v);

        Camera {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            image_height,
            center,
            pixel00_loc,
            delta_u,
            delta_v,
        }
    }

    pub fn render(&mut self, world: &HittableList) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let bar = ProgressBar::new((self.image_height * self.image_width) as u64);
        let img = ImageBuffer::from_fn(self.image_width, self.image_height, |i, j| {
            bar.inc(1);

            let colour: Vector3<f64> = (0..self.samples_per_pixel)
                .map(|_| {
                    let r = self.get_ray(i, j);
                    self.ray_colour(&r, self.max_depth, &world)
                })
                .sum();

            self.make_colour(colour)
        });
        bar.finish();
        img
    }

    fn get_ray(&mut self, i: u32, j: u32) -> Ray {
        let pixel_center =
            self.pixel00_loc + ((i as f64) * self.delta_u) + ((j as f64) * self.delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();
        let ray_direction = pixel_sample - self.center;

        Ray::new(self.center, ray_direction)
    }

    fn pixel_sample_square(&mut self) -> Vector3<f64> {
        let px: f64 = -0.5 * rand::random::<f64>();
        let py: f64 = -0.5 * rand::random::<f64>();

        (px * self.delta_u) + (py * self.delta_v)
    }

    fn ray_colour(&mut self, ray: &Ray, depth: u32, world: &HittableList) -> Vector3<f64> {
        if depth <= 0 {
            Vector3::new(0., 0., 0.)
        } else if let Some(rec) = world.hit(ray, Interval::new(0.0001, f64::MAX)) {
            if let Some((scattered, attenuation)) = rec.mat.scatter(ray, &rec) {
                let col = self.ray_colour(&scattered, depth - 1, world);
                Vector3::new(
                    col.x * attenuation.x,
                    col.y * attenuation.y,
                    col.z * attenuation.z,
                )
            } else {
                Vector3::new(0., 0., 0.)
            }
        } else {
            let unit_direction = ray.direction().normalize();
            let a = 0.5 * (unit_direction.y + 1.);
            (1.0 - a) * Vector3::new(1., 1., 1.) + a * Vector3::new(0.5, 0.7, 1.0)
        }
    }

    fn make_colour(&self, vec: Vector3<f64>) -> Rgb<u8> {
        let scale = 1. / self.samples_per_pixel as f64;

        let r = vec.x * scale;
        let g = vec.y * scale;
        let b = vec.z * scale;

        let intensity = Interval::new(0., 0.999);

        Rgb([
            (intensity.clamp(r.sqrt()) * 256.) as u8,
            (intensity.clamp(g.sqrt()) * 256.) as u8,
            (intensity.clamp(b.sqrt()) * 256.) as u8,
        ])
    }
}
