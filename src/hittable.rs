use crate::{ray::Ray, interval::Interval};

use super::hit_record::HitRecord;

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;
}
