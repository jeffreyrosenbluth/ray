use crate::aabb::*;
use crate::geom::*;
use crate::material::Material;
use crate::object::*;
use std::f64::consts::PI;
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

/// Returns (u, v)
pub fn sphere_uv(p: Point3) -> (f64, f64) {
    let theta = (-p.y).acos();
    let phi = (-p.z).atan2(p.x) + PI;
    (phi / (2.0 * PI), theta / PI)
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
        let outward_normal = (p - self.center(r.time)) / self.radius;
        let (u, v) = sphere_uv(outward_normal);
        let mut rec = HitRecord {
            p,
            normal: ZERO,
            material: self.material.clone(),
            t: root,
            u,
            v,
            front_face: true,
        };
        rec.set_face_normal(r, outward_normal);
        Some(rec)
    }

    fn bounding_box(&self, time_range: &Range<f64>) -> Option<crate::aabb::Aabb> {
        let box0 = Aabb::new(
            self.center(self.time_range.start) - vec3(self.radius, self.radius, self.radius),
            self.center(time_range.start) + vec3(self.radius, self.radius, self.radius),
        );
        let box1 = Aabb::new(
            self.center(self.time_range.end) - vec3(self.radius, self.radius, self.radius),
            self.center(time_range.end) + vec3(self.radius, self.radius, self.radius),
        );

        Some(surrounding_box(box0, box1))
    }
}
