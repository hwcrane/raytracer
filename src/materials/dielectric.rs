use nalgebra::Vector3;
use rand::random;

use crate::ray::Ray;

use super::{
    material::{reflect, refract},
    Material,
};

pub struct Dielectric {
    pub ir: f64,
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray_in: &crate::ray::Ray,
        rec: &crate::hit_record::HitRecord,
    ) -> Option<(crate::ray::Ray, nalgebra::Vector3<f64>)> {
        let refract_ratio = if rec.front_face {
            1. / self.ir
        } else {
            self.ir
        };
        let unit_direction = ray_in.direction().normalize();

        let cos_theta = f64::min((-unit_direction).dot(&rec.normal), 1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let direction =
            if refract_ratio * sin_theta > 1. || reflectance(cos_theta, refract_ratio) > random() {
                reflect(&unit_direction, &rec.normal)
            } else {
                refract(&unit_direction, &rec.normal, refract_ratio)
            };

        Some((Ray::new(rec.point, direction), Vector3::new(1., 1., 1.)))
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1. - ref_idx) / (1. + ref_idx);
    r0 = r0 * r0;
    r0 + (1. - r0) * f64::powi(1. - cosine, 5)
}
