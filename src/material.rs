use nalgebra::Vector3;

use crate::{hit_record::HitRecord, random::rng_unit_vec, ray::Ray};

pub trait Material {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vector3<f64>)>;
}

pub struct Lambertian {
    pub albedo: Vector3<f64>,
}

pub struct Metal {
    pub albedo: Vector3<f64>,
}

impl Material for Metal {
   fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vector3<f64>)> {
        let reflected = reflect(&ray_in.direction().normalize(), &rec.normal);
        Some((Ray::new(rec.point, reflected), self.albedo))
   } 
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

fn reflect(v: &Vector3<f64>, n: &Vector3<f64>) -> Vector3<f64>{
    v - 2. * v.dot(n) * n
}

fn vector_near_zero(vec: &Vector3<f64>) -> bool {
    let s = 1e-6;
    vec.x.abs() < s && vec.y.abs() < s && vec.z.abs() < s
}
