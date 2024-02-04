use nalgebra::Vector3;

use crate::{hit_record::HitRecord, random::rng_unit_vec, ray::Ray};

pub trait MaterialTrait {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vector3<f64>)>;
}

pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
}

impl MaterialTrait for Material {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vector3<f64>)> {
        match self {
            Self::Lambertian(mat) => mat.scatter(ray_in, rec),
            Self::Metal(mat) => mat.scatter(ray_in, rec),
        }
    }
}

impl Material {
    pub fn lambertian(albedo: Vector3<f64>) -> Self {
        Self::Lambertian(Lambertian { albedo })
    }
    pub fn metal(albedo: Vector3<f64>) -> Self {
        Self::Metal(Metal { albedo })
    }
}

struct Lambertian {
    albedo: Vector3<f64>,
}

struct Metal {
    albedo: Vector3<f64>,
}

impl MaterialTrait for Metal {
   fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vector3<f64>)> {
        let reflected = reflect(&ray_in.direction().normalize(), &rec.normal);
        Some((Ray::new(rec.point, reflected), self.albedo))
   } 
}

impl MaterialTrait for Lambertian {
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
