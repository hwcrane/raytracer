use nalgebra::{vector, point, Vector3};
use rand::{random, Rng};

use crate::{hittable_list::HittableList, material::Material, sphere::Sphere, random::rng_vec_bound};

/// Generates the scene from the end of the Ray Tracing in One Weekend book
pub fn random_balls() -> HittableList {
    let mut world = HittableList::new();
    let ground_material = Material::Lambertian {
        albedo: vector![0.5, 0.5, 0.5],
    };
    world.add(Box::new(Sphere::new(
        point![0., -1000., 0.],
        1000.,
        &ground_material,
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
                    let mat = Material::Lambertian { albedo };
                    world.add(Box::new(Sphere::new(center, 0.2, &mat)));
                } else if chose_mat < 0.95 {
                    let albedo = rng_vec_bound(0.5, 1.);
                    let fuzz = rand::thread_rng().gen_range((0.)..0.5);
                    let mat = Material::Metal { albedo, fuzz };
                    world.add(Box::new(Sphere::new(center, 0.2, &mat)));
                } else {
                    let mat = Material::Dielectric { ir: 1.5 };
                    world.add(Box::new(Sphere::new(center, 0.2, &mat)));
                };
            }
        }
    }

    let mat1 = Material::Dielectric { ir: 1.5 };
    let mat2 = Material::Lambertian {
        albedo: vector![0.4, 0.2, 0.1],
    };
    let mat3 = Material::Metal {
        albedo: vector![0.7, 0.6, 0.6],
        fuzz: 0.0,
    };

    world.add(Box::new(Sphere::new(point![0., 1., 0.], 1., &mat1)));
    world.add(Box::new(Sphere::new(point![-4., 1., 0.], 1., &mat2)));
    world.add(Box::new(Sphere::new(point![4., 1., 0.], 1., &mat3)));

    world
}
