use std::{cmp::Ordering, sync::Arc};

use crate::{aabb::Aabb, hittable::Hittable, interval::Interval};
use rand::Rng;

pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: Aabb,
}

impl BvhNode {
    pub fn new(src_objects: &Vec<Arc<dyn Hittable>>) -> BvhNode {
        let mut objects = src_objects.clone();
        let axis: usize = rand::thread_rng().gen_range(0..2);

        let (left, right) = if objects.len() == 1 {
            (objects[0].clone(), objects[0].clone())
        } else if objects.len() == 2 {
            if box_compare(&objects[0], &objects[1], axis).is_gt() {
                (objects[0].clone(), objects[1].clone())
            } else {
                (objects[1].clone(), objects[0].clone())
            }
        } else {
            objects.sort_unstable_by(|a, b| box_compare(a, b, axis).reverse());
            let mid = objects.len() / 2;
            let left: Arc<dyn Hittable> = Arc::new(BvhNode::new(&objects[0..mid].into()));
            let right: Arc<dyn Hittable> = Arc::new(BvhNode::new(&objects[mid..].into()));

            (left, right)
        };
        let bbox = Aabb::merge(&left.bounding_box(), &right.bounding_box());
        BvhNode { left, right, bbox }
    }
}

fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: usize) -> Ordering {
    a.bounding_box()
        .axis(axis)
        .min
        .partial_cmp(&b.bounding_box().axis(axis).min)
        .unwrap()
}
impl Hittable for BvhNode {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        ray_t: crate::interval::Interval,
    ) -> Option<crate::hit_record::HitRecord> {
        if !self.bbox.hit(ray, ray_t) {
            return None;
        }

        let hit_left = self.left.hit(ray, ray_t);
        let hit_right = self.right.hit(
            ray,
            Interval::new(
                ray_t.min,
                if let Some(rec) = &hit_left {
                    rec.t
                } else {
                    ray_t.max
                },
            ),
        );

        if hit_right.is_some() {
            hit_right
        } else {
            hit_left
        }
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
