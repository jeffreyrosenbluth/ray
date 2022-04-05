use crate::geom::*;
use crate::object::Ray;

#[derive(Copy, Clone, Debug)]
pub struct Aabb {
    pub box_min: Vec3,
    pub box_max: Vec3,
}

impl Aabb {
    pub fn new(box_min: Vec3, box_max: Vec3) -> Self {
        Self { box_min, box_max }
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / r.direction[a];
            let mut t0 = (self.box_min[a] - r.origin[a]) * inv_d;
            let mut t1 = (self.box_max[a] - r.origin[a]) * inv_d;
            if inv_d < 0.0 {
                (t0, t1) = (t1, t0)
            }
            let t_min = if t0 > t_min { t0 } else { t_min };
            let t_max = if t1 < t_max { t1 } else { t_max };
            if t_max <= t_min {
                return false;
            }
        }
        true
    }
}
