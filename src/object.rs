use crate::aabb::*;
use crate::geom::*;
use crate::material::*;
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
    fn add_transform(&mut self, _transform: Mat4) {}
    fn pdf_value(&self, _o: Vec3, _v: Vec3) -> f32 {
        panic!("The default implementaion of pdf_value should never be called.");
    }
    fn random(&self, _rng: &mut SmallRng, _o: Vec3) -> Vec3 {
        panic!("The default implementaion of random should never be called.");
    }
}

pub struct Objects {
    pub objects: Vec<Box<dyn Object>>,
}

impl Objects {
    pub fn new(objects: Vec<Box<dyn Object>>) -> Self {
        Self { objects }
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, object: impl Object + 'static) {
        self.objects.push(Box::new(object));
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
        self.as_mut().add_transform(transform);
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
        self.objects
            .iter_mut()
            .for_each(|object| object.add_transform(transform));
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
