pub mod camera;
pub mod hit_record;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod materials;
pub mod random;
pub mod ray;
pub mod sphere;

extern crate nalgebra as na;

use std::sync::Arc;

use camera::Camera;
use hittable_list::HittableList;
use materials::{Dielectric, Lambertian, Metal};
use na::point;
use na::vector;
pub use na::{Point3, Vector3};
use rand::{random, Rng};
use random::rng_vec_bound;
use sphere::Sphere;

fn main() {
    // Image Dimentions
    let aspect_ratio: f64 = 16. / 9.;
    let image_width: u32 = 1200;
    let samples_per_pixel = 500;
    let max_depth = 50;
    let fov = 20.;
    let lookfrom = point![13., 2., 3.];
    let lookat = point![0., 0., 0.];
    let vup = vector![0., 1., 0.];
    let defocus_angle = 0.6;
    let focus_dist = 10.;

    // Camera
    let cam = Camera::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        fov,
        lookfrom,
        lookat,
        vup,
        defocus_angle,
        focus_dist,
    );

    // World
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian {
        albedo: vector![0.5, 0.5, 0.5],
    });
    world.add(Box::new(Sphere::new(
        point![0., -1000., 0.],
        1000.,
        ground_material.clone(),
    )));

    for a in (-11)..11 {
        for b in (-11)..11 {
            let chose_mat: f64 = random();
            let center = point![
                a as f64 + 0.9 * random::<f64>(),
                0.2,
                b as f64 + 0.9 as f64 * random::<f64>()
            ];

            if (center - point![4., 0.2, 0.]).norm() > 0.9 {
                if chose_mat < 0.8 {
                    let albedo: Vector3<f64> = vector![
                        random::<f64>() * random::<f64>(),
                        random::<f64>() * random::<f64>(),
                        random::<f64>() * random::<f64>()
                    ];
                    let mat = Arc::new(Lambertian { albedo });
                    world.add(Box::new(Sphere::new(center, 0.2, mat)));
                } else if chose_mat < 0.95 {
                    let albedo = rng_vec_bound(0.5, 1.);
                    let fuzz = rand::thread_rng().gen_range((0.)..0.5);
                    let mat = Arc::new(Metal { albedo, fuzz });
                    world.add(Box::new(Sphere::new(center, 0.2, mat)));
                } else {
                    let mat = Arc::new(Dielectric { ir: 1.5 });
                    world.add(Box::new(Sphere::new(center, 0.2, mat)));
                };
            }
        }
    }

    let mat1 = Arc::new(Dielectric { ir: 1.5 });
    let mat2 = Arc::new(Lambertian {
        albedo: vector![0.4, 0.2, 0.1],
    });
    let mat3 = Arc::new(Metal {
        albedo: vector![0.7, 0.6, 0.6],
        fuzz: 0.0,
    });

    world.add(Box::new(Sphere::new(point![0., 1., 0.], 1., mat1)));
    world.add(Box::new(Sphere::new(point![-4., 1., 0.], 1., mat2)));
    world.add(Box::new(Sphere::new(point![4., 1., 0.], 1., mat3)));

    cam.render_par(&world).save("output.png").unwrap();
}
