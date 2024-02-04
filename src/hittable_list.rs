use std::sync::Arc;

use crate::interval::Interval;

use super::{hit_record::HitRecord, hittable_enum::Hittable, hittable_trait::HittableTrait};

pub struct HittableList {
    objects: Vec<Arc<Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: vec![] }
    }

    pub fn add(&mut self, object: Hittable) {
        self.objects.push(Arc::new(object))
    }
}

impl HittableTrait for HittableList {
    fn hit(&self, ray: &crate::ray::Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut best_rec = None;
        let mut hit_anything = false;
        let mut closest = ray_t;

        for obj in &self.objects {
            if let Some(rec) = obj.hit(ray, closest) {
                hit_anything = true;
                closest.max = rec.t;
                best_rec = Some(rec);
            }
        }

        if hit_anything {
            best_rec
        } else {
            None
        }
    }
}
