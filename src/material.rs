use nalgebra::Vector3;
use rand::random;

use crate::{hit_record::HitRecord, random::rng_unit_vec, ray::Ray};

#[derive(Clone)]
pub enum Material {
    Lambertian { albedo: Vector3<f64> },
    Metal { albedo: Vector3<f64>, fuzz: f64 },
    Dielectric { ir: f64 },
}

impl Material {
    pub fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vector3<f64>)> {
        match self {
            Self::Lambertian { albedo } => {
                let mut scatter_direction = rec.normal + rng_unit_vec();
                if vector_near_zero(&scatter_direction) {
                    scatter_direction = rec.normal
                }
                Some((Ray::new(rec.point, scatter_direction), *albedo))
            }
            Self::Metal { albedo, fuzz } => {
                let reflected = reflect(&ray_in.direction().normalize(), &rec.normal);
                let scattered = Ray::new(rec.point, reflected + *fuzz * rng_unit_vec());
                if scattered.direction().dot(&rec.normal) > 0. {
                    Some((scattered, *albedo))
                } else {
                    None
                }
            }
            Self::Dielectric { ir } => {
                let refract_ratio = if rec.front_face { 1. / *ir } else { *ir };
                let unit_direction = ray_in.direction().normalize();

                let cos_theta = f64::min((-unit_direction).dot(&rec.normal), 1.);
                let sin_theta = (1. - cos_theta * cos_theta).sqrt();

                let direction = if refract_ratio * sin_theta > 1.
                    || reflectance(cos_theta, refract_ratio) > random()
                {
                    reflect(&unit_direction, &rec.normal)
                } else {
                    refract(&unit_direction, &rec.normal, refract_ratio)
                };

                Some((Ray::new(rec.point, direction), Vector3::new(1., 1., 1.)))
            }
        }
    }
}

// pub trait Material {
//     fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vector3<f64>)>;
// }

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1. - ref_idx) / (1. + ref_idx);
    r0 = r0 * r0;
    r0 + (1. - r0) * f64::powi(1. - cosine, 5)
}
pub fn reflect(v: &Vector3<f64>, n: &Vector3<f64>) -> Vector3<f64> {
    v - 2. * v.dot(n) * n
}

pub fn vector_near_zero(vec: &Vector3<f64>) -> bool {
    let s = 1e-6;
    vec.x.abs() < s && vec.y.abs() < s && vec.z.abs() < s
}

pub fn refract(uv: &Vector3<f64>, n: &Vector3<f64>, etai_over_etat: f64) -> Vector3<f64> {
    let cos_theta = f64::min((-uv).dot(&n), 1.);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -(1. - r_out_perp.norm_squared()).abs().sqrt() * n;
    r_out_perp + r_out_parallel
}
