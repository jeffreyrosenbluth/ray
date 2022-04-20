use crate::aabb::*;
use crate::geom::*;
use crate::material::*;
use crate::object::*;
use std::sync::Arc;

pub struct Rect {
    pub axis: Axis,
    pub p0: f64,
    pub q0: f64,
    pub p1: f64,
    pub q1: f64,
    pub k: f64,
    pub material: Arc<dyn Material>,
}

impl Rect {
    pub fn new(
        axis: Axis,
        p0: f64,
        q0: f64,
        p1: f64,
        q1: f64,
        k: f64,
        material: Arc<dyn Material>,
    ) -> Self {
        Self {
            axis,
            p0,
            q0,
            p1,
            q1,
            k,
            material,
        }
    }
}

impl Object for Rect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let (p, q, s) = self.axis.order();
        let t = (self.k - r.origin[s]) / r.direction[s];
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.origin[p] + t * r.direction[p];
        let y = r.origin[q] + t * r.direction[q];
        if x < self.p0 || x > self.p1 || y < self.q0 || y > self.q1 {
            return None;
        };
        let u = (x - self.p0) / (self.p1 - self.p0);
        let v = (y - self.q0) / (self.q1 - self.q0);
        let pt = r.at(t);
        let outward_normal = match self.axis {
            Axis::X => vec3(1.0, 0.0, 0.0),
            Axis::Y => vec3(0.0, 1.0, 0.0),
            Axis::Z => vec3(0.0, 0.0, 1.0),
        };
        let mut rec = HitRecord {
            p: pt,
            normal: ZERO,
            material: self.material.clone(),
            t,
            u,
            v,
            front_face: true,
        };
        rec.set_face_normal(r, outward_normal);
        Some(rec)
    }

    fn bounding_box(&self, _time_range: &std::ops::Range<f64>) -> Option<crate::aabb::Aabb> {
        let (p, q, s) = self.axis.order();
        let mut a = ZERO;
        let mut b = ZERO;
        a[p] = self.p0;
        a[q] = self.q0;
        a[s] = self.k - 0.0001;
        b[p] = self.p1;
        b[q] = self.q1;
        b[s] = self.k + 0.0001;
        Some(Aabb::new(a, b))
    }
}

pub struct Cuboid {
    pub box_min: Point3,
    pub box_max: Point3,
    pub sides: Objects,
    pub material: Arc<dyn Material>,
}

impl Cuboid {
    pub fn new(box_min: Point3, box_max: Point3, material: Arc<dyn Material>) -> Self {
        let mut sides = Objects::new(Vec::new());
        sides.add(Box::new(Rect::new(
            Axis::Z,
            box_min.x,
            box_min.y,
            box_max.x,
            box_max.y,
            box_max.z,
            material.clone(),
        )));
        sides.add(Box::new(Rect::new(
            Axis::Z,
            box_min.x,
            box_min.y,
            box_max.x,
            box_max.y,
            box_min.z,
            material.clone(),
        )));

        sides.add(Box::new(Rect::new(
            Axis::Y,
            box_min.x,
            box_min.z,
            box_max.x,
            box_max.z,
            box_max.y,
            material.clone(),
        )));
        sides.add(Box::new(Rect::new(
            Axis::Y,
            box_min.x,
            box_min.z,
            box_max.x,
            box_max.z,
            box_min.y,
            material.clone(),
        )));

        sides.add(Box::new(Rect::new(
            Axis::X,
            box_min.y,
            box_min.z,
            box_max.y,
            box_max.z,
            box_max.x,
            material.clone(),
        )));
        sides.add(Box::new(Rect::new(
            Axis::X,
            box_min.y,
            box_min.z,
            box_max.y,
            box_max.z,
            box_min.x,
            material.clone(),
        )));
        Self {
            box_min,
            box_max,
            sides,
            material,
        }
    }
}

impl Object for Cuboid {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }

    fn bounding_box(&self, _time_range: &std::ops::Range<f64>) -> Option<Aabb> {
        Some(Aabb::new(self.box_min, self.box_max))
    }
}
