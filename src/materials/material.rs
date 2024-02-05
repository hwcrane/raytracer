use nalgebra::Vector3;

use crate::{hit_record::HitRecord, ray::Ray};

pub trait Material {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vector3<f64>)>;
}

pub fn reflect(v: &Vector3<f64>, n: &Vector3<f64>) -> Vector3<f64>{
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
