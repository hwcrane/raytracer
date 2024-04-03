
use crate::{shapes::Aabb, utility::Interval};

use super::{hit_record::HitRecord, Ray};

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;
    fn bounding_box(&self) -> &Aabb;
}
