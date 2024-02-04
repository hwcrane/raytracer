pub mod camera;
pub mod hit_record;
pub mod hittable_enum;
pub mod hittable_list;
pub mod hittable_trait;
pub mod interval;
pub mod material;
pub mod random;
pub mod ray;
pub mod sphere;

extern crate nalgebra as na;

use std::sync::Arc;

use camera::Camera;
use hittable_list::HittableList;
use material::{Lambertian, Metal};
use na::{Point3, Vector3};
use sphere::Sphere;

fn main() {
    // Image Dimentions
    let aspect_ratio: f64 = 16. / 9.;
    let image_width: u32 = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // Camera
    let mut cam = Camera::new(aspect_ratio, image_width, samples_per_pixel, max_depth);

    // World
    let mut world = HittableList::new();

    let material_ground = Arc::new(Lambertian {
        albedo: Vector3::new(0.8, 0.8, 0.0),
    });
    let material_center = Arc::new(Lambertian {
        albedo: Vector3::new(0.7, 0.3, 0.3),
    });
    let material_left = Arc::new(Metal {
        albedo: Vector3::new(0.8, 0.8, 0.8),
    });
    let material_right = Arc::new(Metal {
        albedo: Vector3::new(0.8, 0.6, 0.2),
    });

    world.add(Box::new(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
        material_ground,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0., 0., -1.),
        0.5,
        material_center,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1., 0., -1.),
        0.5,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1., 0., -1.),
        0.5,
        material_right,
    )));

    cam.render(&world).save("output.png").unwrap();
}
