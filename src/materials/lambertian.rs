use nalgebra::Vector3;

use crate::{hit_record::HitRecord, random::rng_unit_vec, ray::Ray};

use super::{material::vector_near_zero, Material};

pub struct Lambertian {
    pub albedo: Vector3<f64>,
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vector3<f64>)> {
        let mut scatter_direction = rec.normal + rng_unit_vec();

        if vector_near_zero(&scatter_direction) {
            scatter_direction = rec.normal
        }

        Some((Ray::new(rec.point, scatter_direction), self.albedo))
    }
}
