use crate::{interval::Interval, hittable_trait::Hittable};

use super::hit_record::HitRecord;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: vec![] }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object)
    }
}

impl Hittable for HittableList {
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
