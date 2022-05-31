use crate::aabb::*;
use crate::bvh::BvhNode;
// use crate::geom::*;
use crate::geom::*;
use crate::material::*;
use crate::rect::{Cuboid, Rect};
use crate::sphere::Sphere;
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use std::ops::Range;
use std::sync::Arc;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(
        p: Point3,
        normal: Vec3,
        material: Arc<dyn Material>,
        t: f32,
        u: f32,
        v: f32,
        front_face: bool,
    ) -> Self {
        Self {
            p,
            normal,
            material,
            t,
            u,
            v,
            front_face,
        }
    }

    pub fn with_ray(
        r: &Ray,
        p: Point3,
        outward_normal: Vec3,
        material: Arc<dyn Material>,
        t: f32,
        u: f32,
        v: f32,
    ) -> Self {
        let front_face = dot(r.direction, outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        Self {
            p,
            normal,
            material,
            t,
            u,
            v,
            front_face,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = dot(r.direction, outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Object: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, time_range: &Range<f32>) -> Option<Aabb>;
    fn add_transform(&mut self, transform: Mat4);
    fn pdf_value(&self, _o: Vec3, _v: Vec3) -> f32 {
        panic!("The default implementaion of pdf_value should never be called.");
    }
    fn random(&self, _rng: &mut SmallRng, _o: Vec3) -> Vec3 {
        panic!("The default implementaion of random should never be called.");
    }
}

pub struct Objects {
    pub objects: Vec<Geometry>,
}

impl Objects {
    pub fn new(objects: Vec<Geometry>) -> Self {
        Self { objects }
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, object: Geometry) {
        self.objects.push(object);
    }
}

impl Object for Box<dyn Object> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.as_ref().hit(r, t_min, t_max)
    }

    fn bounding_box(&self, time_range: &Range<f32>) -> Option<Aabb> {
        self.as_ref().bounding_box(time_range)
    }

    fn add_transform(&mut self, transform: Mat4) {
        self.as_ref().add_transform(transform);
    }

    fn pdf_value(&self, o: Vec3, v: Vec3) -> f32 {
        (**self).pdf_value(o, v)
    }

    fn random(&self, rng: &mut SmallRng, o: Vec3) -> Vec3 {
        (**self).random(rng, o)
    }
}

impl Object for Objects {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut rec = None;
        let mut closest_so_far = t_max;
        for object in &self.objects {
            if let Some(new_rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = new_rec.t;
                rec = Some(new_rec);
            }
        }
        rec
    }

    fn bounding_box(&self, time_range: &Range<f32>) -> Option<Aabb> {
        let aabb = self.objects.iter().fold(Some(Aabb::empty()), |mut acc, o| {
            if let Some(b) = o.bounding_box(time_range) {
                acc = Some(surrounding_box(acc.unwrap(), b));
            } else {
                acc = None;
            };
            acc
        });
        aabb
    }

    fn add_transform(&mut self, transform: Mat4) {
        for object in &self.objects {
            object.add_transform(transform)
        }
    }

    fn pdf_value(&self, o: Vec3, v: Vec3) -> f32 {
        debug_assert!(!self.objects.is_empty());
        self.objects.iter().map(|h| h.pdf_value(o, v)).sum::<f32>() / self.objects.len() as f32
    }

    fn random(&self, rng: &mut SmallRng, o: Vec3) -> Vec3 {
        self.objects.choose(rng).unwrap().random(rng, o)
    }
}

pub struct EmptyObject {}

impl Object for EmptyObject {
    fn hit(&self, _r: &Ray, _t_min: f32, _t_max: f32) -> Option<HitRecord> {
        None
    }

    fn bounding_box(&self, _time_range: &Range<f32>) -> Option<Aabb> {
        None
    }

    fn add_transform(&mut self, transform: Mat4) {}

    fn pdf_value(&self, _o: Vec3, _v: Vec3) -> f32 {
        0.0
    }

    fn random(&self, _rng: &mut SmallRng, _o: Vec3) -> Vec3 {
        Vec3::ZERO
    }
}

pub enum Geometry {
    Sphere(Sphere),
    Rect(Rect),
    Cuboid(Cuboid),
    BvhNode(BvhNode),
}

impl Geometry {
    pub fn sphere(center: Point3, radius: f32, material: Arc<dyn Material>) -> Self {
        Self::Sphere(Sphere::new(center, radius, material))
    }

    pub fn sphere_moving(
        center0: Point3,
        center1: Point3,
        radius: f32,
        material: Arc<dyn Material>,
        time_range: Range<f32>,
    ) -> Self {
        Self::Sphere(Sphere::new_moving(
            center0, center1, radius, material, time_range,
        ))
    }

    pub fn rect(
        axis: Axis,
        p0: f32,
        q0: f32,
        p1: f32,
        q1: f32,
        k: f32,
        material: Arc<dyn Material>,
    ) -> Self {
        Self::Rect(Rect::new(axis, p0, q0, p1, q1, k, material))
    }

    pub fn cuboid(box_min: Point3, box_max: Point3, material: Arc<dyn Material>) -> Self {
        Self::Cuboid(Cuboid::new(box_min, box_max, material))
    }

    pub fn bvh_node(objects: &mut Objects, start: usize, end: usize, time: Range<f32>) -> Self {
        Self::BvhNode(BvhNode::new(objects, start, end, time))
    }
}

impl Object for Geometry {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match self {
            Geometry::Sphere(g) => g.hit(r, t_min, t_max),
            Geometry::Rect(g) => g.hit(r, t_min, t_max),
            Geometry::Cuboid(g) => g.hit(r, t_min, t_max),
            Geometry::BvhNode(g) => g.hit(r, t_min, t_max),
        }
    }

    fn bounding_box(&self, time_range: &Range<f32>) -> Option<Aabb> {
        match self {
            Geometry::Sphere(g) => g.bounding_box(time_range),
            Geometry::Rect(g) => g.bounding_box(time_range),
            Geometry::Cuboid(g) => g.bounding_box(time_range),
            Geometry::BvhNode(g) => g.bounding_box(time_range),
        }
    }

    fn pdf_value(&self, o: Vec3, v: Vec3) -> f32 {
        match self {
            Geometry::Sphere(g) => g.pdf_value(o, v),
            Geometry::Rect(g) => g.pdf_value(o, v),
            Geometry::Cuboid(g) => g.pdf_value(o, v),
            Geometry::BvhNode(g) => g.pdf_value(o, v),
        }
    }

    fn random(&self, rng: &mut SmallRng, o: Vec3) -> Vec3 {
        match self {
            Geometry::Sphere(g) => g.random(rng, o),
            Geometry::Rect(g) => g.random(rng, o),
            Geometry::Cuboid(g) => g.random(rng, o),
            Geometry::BvhNode(g) => g.random(rng, o),
        }
    }

    fn add_transform(&mut self, transform: Mat4) {
        match self {
            Geometry::Sphere(g) => g.add_transform(transform),
            Geometry::Rect(g) => g.add_transform(transform),
            Geometry::Cuboid(g) => g.add_transform(transform),
            Geometry::BvhNode(g) => g.add_transform(transform),
        }
    }
}

#[derive(Clone)]
pub struct FlipFace<T> {
    pub object: T,
}

impl<T> FlipFace<T> {
    pub fn new(object: T) -> Self {
        Self { object }
    }
}

pub struct Translate<T> {
    pub object: T,
    pub offset: Vec3,
}

impl<T> Object for FlipFace<T>
where
    T: Object,
{
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if let Some(mut rec) = self.object.hit(r, t_min, t_max) {
            rec.front_face = !rec.front_face;
            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, time_range: &Range<f32>) -> Option<Aabb> {
        self.object.bounding_box(time_range)
    }

    fn add_transform(&mut self, transform: Mat4) {}
}

impl<T> Translate<T> {
    pub fn new(object: T, offset: Vec3) -> Self {
        Self { object, offset }
    }
}

impl<T> Object for Translate<T>
where
    T: Object,
{
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let moved_r = Ray::new(r.origin - self.offset, r.direction, r.time);
        if let Some(mut rec) = self.object.hit(&moved_r, t_min, t_max) {
            rec.p += self.offset;
            rec.set_face_normal(&moved_r, rec.normal);
            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, time_range: &Range<f32>) -> Option<Aabb> {
        if let Some(bbox) = self.object.bounding_box(time_range) {
            Some(Aabb::new(
                bbox.box_min + self.offset,
                bbox.box_max + self.offset,
            ))
        } else {
            None
        }
    }

    fn add_transform(&mut self, transform: Mat4) {}
}

pub struct Rotate<T> {
    pub axis: Axis,
    pub object: T,
    pub sin: f32,
    pub cos: f32,
    pub bbox: Option<Aabb>,
}

impl<T> Rotate<T>
where
    T: Object,
{
    pub fn new(axis: Axis, object: T, degrees: f32) -> Self {
        let theta = degrees * PI / 180.0;
        let sin = theta.sin();
        let cos = theta.cos();
        let mut rect = Aabb::empty();
        let (p, q, s) = axis.order();
        let bbox = object.bounding_box(&(0.0..1.0)).map(|b| {
            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let x = i as f32 * b.box_max.x + (1.0 - i as f32) * b.box_min.x;
                        let y = j as f32 * b.box_max.y + (1.0 - j as f32) * b.box_min.y;
                        let z = k as f32 * b.box_max.z + (1.0 - k as f32) * b.box_min.z;
                        let coords = vec3(x, y, z);
                        let newp = cos * coords[p] + sin * coords[q];
                        let newq = -sin * coords[p] + cos * coords[q];
                        let mut tester = Vec3::ZERO;
                        tester[p] = newp;
                        tester[q] = newq;
                        tester[s] = coords[s];
                        for c in 0..3 {
                            rect.box_min[c] = rect.box_min[c].min(tester[c]);
                            rect.box_max[c] = rect.box_max[c].min(tester[c]);
                        }
                    }
                }
            }
            rect
        });
        Self {
            axis,
            object,
            sin,
            cos,
            bbox,
        }
    }
}

impl<T> Object for Rotate<T>
where
    T: Object,
{
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut origin = r.origin;
        let mut direction = r.direction;
        let (p, q, _) = self.axis.order();
        origin[p] = self.cos * r.origin[p] - self.sin * r.origin[q];
        origin[q] = self.sin * r.origin[p] + self.cos * r.origin[q];
        direction[p] = self.cos * r.direction[p] - self.sin * r.direction[q];
        direction[q] = self.sin * r.direction[p] + self.cos * r.direction[q];
        let rotated_r = Ray::new(origin, direction, r.time);
        let hr = self.object.hit(&rotated_r, t_min, t_max).map(|mut rec| {
            let mut pt = rec.p;
            let mut normal = rec.normal;
            pt[p] = self.cos * rec.p[p] + self.sin * rec.p[q];
            pt[q] = -self.sin * rec.p[p] + self.cos * rec.p[q];
            normal[p] = self.cos * rec.normal[p] + self.sin * rec.normal[q];
            normal[q] = -self.sin * rec.normal[p] + self.cos * rec.normal[q];
            rec.p = pt;
            rec.set_face_normal(&rotated_r, normal);
            rec
        });
        hr
    }

    fn bounding_box(&self, _time_range: &Range<f32>) -> Option<Aabb> {
        self.bbox
    }

    fn add_transform(&mut self, transform: Mat4) {}
}

pub struct ConstantMedium<O> {
    pub boundary: O,
    pub phase_function: Isotropic<Color>,
    pub neg_inv_density: f32,
    transform: Mat4,
    inv_transform: Mat4,
}

impl<O> ConstantMedium<O> {
    pub fn new(boundary: O, color: Color, d: f32) -> Self {
        Self {
            boundary,
            phase_function: Isotropic::new(color),
            neg_inv_density: -1.0 / d,
            transform: Mat4::IDENTITY,
            inv_transform: Mat4::IDENTITY,
        }
    }
}

impl<O> Object for ConstantMedium<O>
where
    O: Object,
{
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let r = r.transform(self.inv_transform);
        let mut rng = SmallRng::from_entropy();
        let mut rec1 = self.boundary.hit(&r, f32::MIN, f32::MAX)?;
        let mut rec2 = self.boundary.hit(&r, rec1.t + 0.0001, f32::MAX)?;
        rec1.t = rec1.t.max(t_min);
        rec2.t = rec2.t.min(t_max);
        if rec1.t >= rec2.t {
            return None;
        }
        rec1.t = rec1.t.max(0.0);
        let ray_length = r.direction.length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * rng.gen::<f32>().ln();
        if hit_distance > distance_inside_boundary {
            return None;
        }
        let t = rec1.t + hit_distance / ray_length;
        let p = r.at(t);
        Some(HitRecord::new(
            self.transform.transform_point3(p),
            vec3(1.0, 0.0, 0.0), // arbitrary
            Arc::new(self.phase_function.clone()),
            t,
            1.0,
            1.0,
            true, // arbitrary
        ))
    }

    fn bounding_box(&self, time_range: &std::ops::Range<f32>) -> Option<crate::aabb::Aabb> {
        self.boundary.bounding_box(time_range)
    }

    fn add_transform(&mut self, transform: Mat4) {}
}
