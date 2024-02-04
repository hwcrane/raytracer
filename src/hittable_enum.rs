// use crate::interval::Interval;
//
// use super::{hit_record::HitRecord, hittable_trait::HittableTrait, sphere::Sphere};
//
// pub enum Hittable {
//     Sphere(Sphere),
// }
//
// impl HittableTrait for Hittable {
//     fn hit(&self, ray: &crate::ray::Ray, ray_t: Interval) -> Option<HitRecord> {
//         match self {
//             Hittable::Sphere(sphere) => sphere.hit(ray, ray_t),
//         }
//     }
// }
