use crate::geom::*;

#[derive(Copy, Clone, Debug)]
pub struct Aabb {
    pub box_min: Point3,
    pub box_max: Point3,
}

impl Aabb {
    pub fn new(box_min: Point3, box_max: Point3) -> Self {
        Self { box_min, box_max }
    }

    pub fn empty() -> Self {
        Self {
            box_min: point3(f32::MAX, f32::MAX, f32::MAX),
            box_max: point3(f32::MIN, f32::MIN, f32::MIN),
        }
    }

    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> bool {
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

    pub fn transform_box(&self, transform: Mat4) -> Self {
        let xa = transform.col(0) * self.box_min.x;
        let xb = transform.col(0) * self.box_max.x;
        let ya = transform.col(1) * self.box_min.y;
        let yb = transform.col(1) * self.box_max.y;
        let za = transform.col(2) * self.box_min.z;
        let zb = transform.col(2) * self.box_max.z;

        let mn = xa.min(xb) + ya.min(yb) + za.min(zb) + transform.col(3);
        let mx = xa.max(xb) + ya.max(yb) + za.max(zb) + transform.col(3);
        Aabb {
            box_min: mn.truncate(),
            box_max: mx.truncate(),
        }
    }
}

pub fn surrounding_box(box0: Aabb, box1: Aabb) -> Aabb {
    let small = point3(
        box0.box_min.x.min(box1.box_min.x),
        box0.box_min.y.min(box1.box_min.y),
        box0.box_min.z.min(box1.box_min.z),
    );

    let large = point3(
        box0.box_max.x.max(box1.box_max.x),
        box0.box_max.y.max(box1.box_max.y),
        box0.box_max.z.max(box1.box_max.z),
    );

    Aabb {
        box_min: small,
        box_max: large,
    }
}
