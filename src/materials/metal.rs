use nalgebra::Vector3;

use crate::{hit_record::HitRecord, random::rng_unit_vec, ray::Ray};

use super::{material::reflect, Material};

pub struct Metal {
    pub albedo: Vector3<f64>,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vector3<f64>)> {
        let reflected = reflect(&ray_in.direction().normalize(), &rec.normal);
        let scattered = Ray::new(rec.point, reflected + self.fuzz * rng_unit_vec());
        if scattered.direction().dot(&rec.normal) > 0. {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}
