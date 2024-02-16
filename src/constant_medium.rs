use std::sync::Arc;

use nalgebra::vector;
use rand::random;

use crate::{
    hit_record::HitRecord, hittable::Hittable, interval::Interval, material::Material,
    textures::Texture,
};

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Material,
}

impl ConstantMedium {
    pub fn new(boundary: Arc<dyn Hittable>, density: f64, phase_funcion: Arc<dyn Texture>) -> Self {
        ConstantMedium {
            boundary,
            neg_inv_density: -1. / density,
            phase_function: Material::Isotropic {
                albedo: phase_funcion,
            },
        }
    }
}

impl Hittable for ConstantMedium {
    fn bounding_box(&self) -> &crate::aabb::Aabb {
        self.boundary.bounding_box()
    }

    fn hit(
        &self,
        ray: &crate::ray::Ray,
        ray_t: crate::interval::Interval,
    ) -> Option<crate::hit_record::HitRecord> {
        let mut rec1 = self.boundary.hit(ray, Interval::universe())?;
        let mut rec2 = self
            .boundary
            .hit(ray, Interval::new(rec1.t + 0.0001, f64::MAX))?;

        if rec1.t < ray_t.min {
            rec1.t = ray_t.min
        }

        if rec2.t > ray_t.max {
            rec2.t = ray_t.max
        }

        if rec1.t >= rec2.t {
            return None;
        }

        if rec1.t < 0. {
            rec1.t = 0.
        }

        let ray_length = ray.direction().norm();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random::<f64>().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }
        let t = rec1.t + hit_distance / ray_length;
        let point = ray.at(t);
        let normal = vector![1., 0., 0.];
        let mat = &self.phase_function;
        Some(HitRecord::new(point, normal, mat, t, 0., 0., ray))
    }
}
