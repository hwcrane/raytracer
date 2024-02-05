use std::sync::{Arc, Mutex};

use image::{ImageBuffer, Rgb};
use indicatif::ProgressBar;
use nalgebra::{vector, Point3, Vector3};
use rayon::prelude::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

use crate::{
    hittable::Hittable, hittable_list::HittableList, interval::Interval, random::rng_in_unit_disk,
    ray::Ray,
};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub vfov: f64,
    pub lookat: Point3<f64>,
    pub lookfrom: Point3<f64>,
    pub vup: Vector3<f64>,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    image_height: u32,
    center: Point3<f64>,
    pixel00_loc: Point3<f64>,
    delta_u: Vector3<f64>,
    delta_v: Vector3<f64>,
    u: Vector3<f64>,
    v: Vector3<f64>,
    w: Vector3<f64>,
    defocus_disk_u: Vector3<f64>,
    defocus_disk_v: Vector3<f64>,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        samples_per_pixel: u32,
        max_depth: u32,
        vfov: f64,
        lookfrom: Point3<f64>,
        lookat: Point3<f64>,
        vup: Vector3<f64>,
        defocus_angle: f64,
        focus_dist: f64,
    ) -> Self {
        let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;

        let center = lookfrom;

        // Viewport Dimentions
        let theta = vfov.to_radians();
        let h = (theta / 2.).tan();
        let viewport_height: f64 = 2. * h * focus_dist;
        let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);

        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        // Refrence basis for viewport
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        // Delta vectors
        let delta_u: Vector3<f64> = viewport_u / image_width as f64;
        let delta_v: Vector3<f64> = viewport_v / image_height as f64;

        // Location of upper left
        let viewport_upper_left = center - (focus_dist * w) - (viewport_u / 2.) - (viewport_v / 2.);
        let pixel00_loc = viewport_upper_left + 0.5 * (delta_u + delta_v);

        let defocus_radius = focus_dist * (defocus_angle / 2.).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            vfov,
            lookfrom,
            lookat,
            vup,
            defocus_angle,
            focus_dist,
            image_height,
            center,
            pixel00_loc,
            delta_u,
            delta_v,
            u,
            v,
            w,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub fn render(&mut self, world: &dyn Hittable) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let bar = ProgressBar::new((self.image_height * self.image_width) as u64);
        let img = ImageBuffer::from_fn(self.image_width, self.image_height, |i, j| {
            bar.inc(1);

            let colour: Vector3<f64> = (0..self.samples_per_pixel)
                .map(|_| {
                    let r = self.get_ray(i, j);
                    self.ray_colour(&r, self.max_depth, world)
                })
                .sum();

            self.make_colour(colour)
        });
        bar.finish();
        img
    }

    pub fn render_par(&self, world: &HittableList) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let bar = ProgressBar::new((self.image_height * self.image_width) as u64);
        let mut img = ImageBuffer::new(self.image_width, self.image_height);
        let mut pixels =
            vec![Rgb([0 as u8, 0 as u8, 0 as u8]); (self.image_height * self.image_width) as usize];
        (pixels).par_iter_mut().enumerate().for_each(|(n, pixel)| {
            let i = n as u32 % self.image_width;
            let j = n as u32 / self.image_width;

            bar.inc(1);

            let colour: Vector3<f64> = (0..self.samples_per_pixel)
                .map(|_| {
                    let r = self.get_ray(i, j);
                    self.ray_colour(&r, self.max_depth, world)
                })
                .sum();

            *pixel = self.make_colour(colour);
        });

        bar.finish();

        for (x, y, pixel) in img.enumerate_pixels_mut() {
            *pixel = pixels[(x + self.image_width * y) as usize];
        }

        img
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let pixel_center =
            self.pixel00_loc + ((i as f64) * self.delta_u) + ((j as f64) * self.delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();
        let ray_origin = if self.defocus_angle <= 0. {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn defocus_disk_sample(&self) -> Point3<f64> {
        let p = rng_in_unit_disk();
        self.center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }

    fn pixel_sample_square(&self) -> Vector3<f64> {
        let px: f64 = -0.5 * rand::random::<f64>();
        let py: f64 = -0.5 * rand::random::<f64>();

        (px * self.delta_u) + (py * self.delta_v)
    }

    fn ray_colour(&self, ray: &Ray, depth: u32, world: &dyn Hittable) -> Vector3<f64> {
        if depth <= 0 {
            vector!(0., 0., 0.)
        } else if let Some(rec) = world.hit(ray, Interval::new(0.0001, f64::MAX)) {
            if let Some((scattered, attenuation)) = rec.mat.scatter(ray, &rec) {
                let col = self.ray_colour(&scattered, depth - 1, world);
                vector!(
                    col.x * attenuation.x,
                    col.y * attenuation.y,
                    col.z * attenuation.z,
                )
            } else {
                vector!(0., 0., 0.)
            }
        } else {
            let unit_direction = ray.direction().normalize();
            let a = 0.5 * (unit_direction.y + 1.);
            (1.0 - a) * vector!(1., 1., 1.) + a * vector!(0.5, 0.7, 1.0)
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
