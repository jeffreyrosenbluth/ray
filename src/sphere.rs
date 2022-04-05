use crate::geom::*;
use crate::material::Material;
use crate::object::*;
use std::ops::Range;
use std::sync::Arc;

pub struct Sphere {
    pub center0: Point3,
    pub center1: Point3,
    pub radius: f64,
    pub material: Arc<dyn Material>,
    pub time_range: Range<f64>,
}

impl Sphere {
    pub fn new_moving(
        center0: Point3,
        center1: Point3,
        radius: f64,
        material: Arc<dyn Material>,
        time_range: Range<f64>,
    ) -> Self {
        Self {
            center0,
            center1,
            radius,
            material,
            time_range,
        }
    }

    pub fn new(center0: Point3, radius: f64, material: Arc<dyn Material>) -> Self {
        Self {
            center0,
            center1: center0,
            radius,
            material,
            time_range: 0.0..0.0,
        }
    }

    pub fn center(&self, time: f64) -> Point3 {
        if self.time_range.is_empty() {
            return self.center0;
        }
        self.center0
            + ((time - self.time_range.start) / (self.time_range.end - self.time_range.start))
                * (self.center1 - self.center0)
    }
}

impl Object for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center(r.time);
        let a = r.direction.length2();
        let half_b = dot(oc, r.direction);
        let c = oc.length2() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        };

        let sqrtd = discriminant.sqrt();
        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            };
        }
        let p = r.at(root);
        let mut rec = HitRecord {
            t: root,
            p,
            material: self.material.clone(),
            normal: ZERO,
            front_face: true,
        };
        let outward_normal = (rec.p - self.center(r.time)) / self.radius;
        rec.set_face_normal(r, outward_normal);
        Some(rec)
    }
}
