use crate::aabb::*;
use crate::object::*;
use std::sync::Arc;

pub struct BvhNode {
    pub left: Arc<dyn Object>,
    pub right: Arc<dyn Object>,
    pub bbox: Aabb,
}

impl BvhNode {
    pub fn new(left: Arc<dyn Object>, right: Arc<dyn Object>, bbox: Aabb) -> Self {
        Self { left, right, bbox }
    }
}

impl Object for BvhNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if self.bbox.hit(r, t_min, t_max) {
            let hit_left = self.left.hit(r, t_min, t_max);
            if hit_left.is_some() {return hit_left};
            let hit_right = self.right.hit(r, t_min, t_max);
            if hit_right.is_some() { return  hit_right};
        }
        None
    }

    fn bounding_box(&self, _time_range: &std::ops::Range<f64>) -> Option<Aabb> {
        Some(self.bbox)
    }
}