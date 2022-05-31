use rand::rngs::SmallRng;
use rand::Rng;

use crate::aabb::*;
use crate::geom::*;
use crate::material::*;
use crate::object::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct Rect {
    pub axis: Axis,
    pub p0: f32,
    pub q0: f32,
    pub p1: f32,
    pub q1: f32,
    pub k: f32,
    transform: Mat4,
    inv_transform: Mat4,
    pub material: Arc<dyn Material>,
}

impl Rect {
    pub fn new(
        axis: Axis,
        p0: f32,
        q0: f32,
        p1: f32,
        q1: f32,
        k: f32,
        material: Arc<dyn Material>,
    ) -> Self {
        Self {
            axis,
            p0,
            q0,
            p1,
            q1,
            k,
            transform: Mat4::IDENTITY,
            inv_transform: Mat4::IDENTITY,
            material,
        }
    }

    pub fn set_transform(mut self, transform: Mat4) -> Self {
        self.transform = transform;
        self.inv_transform = transform.inverse();
        self
    }
}

impl Object for Rect {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let r = r.transform(self.inv_transform);
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
        let mut outward_normal = match self.axis {
            Axis::X => vec3(1.0, 0.0, 0.0),
            Axis::Y => vec3(0.0, 1.0, 0.0),
            Axis::Z => vec3(0.0, 0.0, 1.0),
        };
        outward_normal = self
            .inv_transform
            .transpose()
            .transform_vector3(outward_normal)
            .normalize();
        let rec = HitRecord::with_ray(
            &r,
            self.transform.transform_point3(pt),
            outward_normal,
            self.material.clone(),
            t,
            u,
            v,
        );
        Some(rec)
    }

    fn bounding_box(&self, _time_range: &std::ops::Range<f32>) -> Option<crate::aabb::Aabb> {
        let (p, q, s) = self.axis.order();
        let mut a = Vec3::ZERO;
        let mut b = Vec3::ZERO;
        a[p] = self.p0;
        a[q] = self.q0;
        a[s] = self.k - 0.0001;
        b[p] = self.p1;
        b[q] = self.q1;
        b[s] = self.k + 0.0001;
        Some(Aabb::new(a, b))
    }

    fn add_transform(&mut self, transform: Mat4) {
        self.transform = transform * self.transform;
        self.inv_transform = transform.inverse() * self.inv_transform;
    }

    fn pdf_value(&self, o: Vec3, v: Vec3) -> f32 {
        if let Some(rec) = self.hit(&Ray::new(o, v, 0.0), 0.001, std::f32::MAX) {
            let area = (self.p1 - self.p0) * (self.q1 - self.q0);
            let distance_squared = rec.t * rec.t * v.length_squared();
            let cosine = (dot(v, rec.normal) / v.length()).abs();
            return distance_squared / (cosine * area);
        }
        0.0
    }

    fn random(&self, rng: &mut SmallRng, o: Vec3) -> Vec3 {
        let (p, q, s) = self.axis.order();
        let pr = rng.gen_range(self.p0..self.p1);
        let qr = rng.gen_range(self.q0..self.q1);
        let mut random_point = Vec3::ZERO;
        random_point[p] = pr;
        random_point[q] = qr;
        random_point[s] = self.k;
        random_point - o
    }
}

pub struct Cuboid {
    pub box_min: Point3,
    pub box_max: Point3,
    pub sides: Objects,
    transform: Mat4,
    inv_transform: Mat4,
    pub material: Arc<dyn Material>,
}

impl Cuboid {
    pub fn new(box_min: Point3, box_max: Point3, material: Arc<dyn Material>) -> Self {
        let mut sides = Objects::new(Vec::new());
        sides.add(Geometry::rect(
            Axis::Z,
            box_min.x,
            box_min.y,
            box_max.x,
            box_max.y,
            box_max.z,
            material.clone(),
        ));
        sides.add(Geometry::rect(
            Axis::Z,
            box_min.x,
            box_min.y,
            box_max.x,
            box_max.y,
            box_min.z,
            material.clone(),
        ));
        sides.add(Geometry::rect(
            Axis::Y,
            box_min.x,
            box_min.z,
            box_max.x,
            box_max.z,
            box_max.y,
            material.clone(),
        ));
        sides.add(Geometry::rect(
            Axis::Y,
            box_min.x,
            box_min.z,
            box_max.x,
            box_max.z,
            box_min.y,
            material.clone(),
        ));

        sides.add(Geometry::rect(
            Axis::X,
            box_min.y,
            box_min.z,
            box_max.y,
            box_max.z,
            box_max.x,
            material.clone(),
        ));
        sides.add(Geometry::rect(
            Axis::X,
            box_min.y,
            box_min.z,
            box_max.y,
            box_max.z,
            box_min.x,
            material.clone(),
        ));
        Self {
            box_min,
            box_max,
            sides,
            transform: Mat4::IDENTITY,
            inv_transform: Mat4::IDENTITY,
            material,
        }
    }
}

/// XXX tranform rectanges XXX
impl Object for Cuboid {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }

    fn bounding_box(&self, _time_range: &std::ops::Range<f32>) -> Option<Aabb> {
        Some(Aabb::new(self.box_min, self.box_max))
    }

    fn add_transform(&mut self, transform: Mat4) {
        for side in self.sides.objects {
            side.add_transform(transform);
        }
    }
}
