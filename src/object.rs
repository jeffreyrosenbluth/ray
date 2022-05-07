use rand::prelude::*;

use crate::aabb::*;
use crate::geom::*;
use crate::material::*;
use std::ops::Range;
use std::sync::Arc;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
    pub time: Float,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3, time: Float) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    pub fn at(&self, t: Float) -> Point3 {
        self.origin + t * self.direction
    }
}

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
    pub t: Float,
    pub u: Float,
    pub v: Float,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(
        p: Point3,
        normal: Vec3,
        material: Arc<dyn Material>,
        t: Float,
        u: Float,
        v: Float,
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
        t: Float,
        u: Float,
        v: Float,
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
    fn hit(&self, r: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord>;
    fn bounding_box(&self, time_range: &Range<Float>) -> Option<Aabb>;
    fn pdf_value(&self, _o: Vec3, _v: Vec3) -> Float {
        0.0
    }
    fn random(&self, _o: Vec3) -> Vec3 {
        vec3(1.0, 0.0, 0.0)
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
    fn hit(&self, r: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        self.as_ref().hit(r, t_min, t_max)
    }

    fn bounding_box(&self, time_range: &Range<Float>) -> Option<Aabb> {
        self.as_ref().bounding_box(time_range)
    }
}

impl Object for Objects {
    fn hit(&self, r: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
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

    fn bounding_box(&self, time_range: &Range<Float>) -> Option<Aabb> {
        let aabb = self.objects.iter().fold(Some(Aabb::EMPTY), |mut acc, o| {
            if let Some(b) = o.bounding_box(time_range) {
                acc = Some(surrounding_box(acc.unwrap(), b));
            } else {
                acc = None;
            };
            acc
        });
        aabb
    }
}

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
    fn hit(&self, r: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        if let Some(mut rec) = self.object.hit(r, t_min, t_max) {
            rec.front_face = !rec.front_face;
            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, time_range: &Range<Float>) -> Option<Aabb> {
        self.object.bounding_box(time_range)
    }
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
    fn hit(&self, r: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        let moved_r = Ray::new(r.origin - self.offset, r.direction, r.time);
        if let Some(mut rec) = self.object.hit(&moved_r, t_min, t_max) {
            rec.p += self.offset;
            rec.set_face_normal(&moved_r, rec.normal);
            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, time_range: &Range<Float>) -> Option<Aabb> {
        if let Some(bbox) = self.object.bounding_box(time_range) {
            Some(Aabb::new(
                bbox.box_min + self.offset,
                bbox.box_max + self.offset,
            ))
        } else {
            None
        }
    }
}

pub struct Rotate<T> {
    pub axis: Axis,
    pub object: T,
    pub sin: Float,
    pub cos: Float,
    pub bbox: Option<Aabb>,
}

impl<T> Rotate<T>
where
    T: Object,
{
    pub fn new(axis: Axis, object: T, degrees: Float) -> Self {
        let theta = degrees * PI / 180.0;
        let sin = theta.sin();
        let cos = theta.cos();
        let mut rect = Aabb::EMPTY;
        let (p, q, s) = axis.order();
        let bbox = object.bounding_box(&(0.0..1.0)).map(|b| {
            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let x = i as Float * b.box_max.x + (1.0 - i as Float) * b.box_min.x;
                        let y = j as Float * b.box_max.y + (1.0 - j as Float) * b.box_min.y;
                        let z = k as Float * b.box_max.z + (1.0 - k as Float) * b.box_min.z;
                        let coords = vec3(x, y, z);
                        let newp = cos * coords[p] + sin * coords[q];
                        let newq = -sin * coords[p] + cos * coords[q];
                        let mut tester = ZERO;
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
    fn hit(&self, r: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
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

    fn bounding_box(&self, _time_range: &Range<Float>) -> Option<Aabb> {
        self.bbox
    }
}

pub struct ConstantMedium<O> {
    pub boundary: O,
    pub phase_function: Isotropic<Color>,
    pub neg_inv_density: Float,
}

impl<O> ConstantMedium<O> {
    pub fn new(boundary: O, color: Color, d: Float) -> Self {
        Self {
            boundary,
            phase_function: Isotropic::new(color),
            neg_inv_density: -1.0 / d,
        }
    }
}

// impl<O> Object for ConstantMedium<O>
// where
//     O: Object,
// {
//     fn hit(&self, r: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
//         let mut rng = thread_rng();
//         let mut rec1 = self.boundary.hit(r, Float::MIN, Float::MAX)?;
//         let mut rec2 = self.boundary.hit(r, rec1.t + 0.0001, Float::MAX)?;
//         rec1.t = rec1.t.max(t_min);
//         rec2.t = rec2.t.min(t_max);
//         if rec1.t >= rec2.t {
//             return None;
//         }
//         rec1.t = rec1.t.max(0.0);
//         let ray_length = r.direction.length();
//         let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
//         let hit_distance = self.neg_inv_density * rng.gen::<Float>().ln();
//         if hit_distance > distance_inside_boundary {
//             return None;
//         }
//         let t = rec1.t + hit_distance / ray_length;
//         Some(HitRecord::new(
//             r.at(t),
//             vec3(1.0, 0.0, 0.0), // arbitrary
//             Arc::new(self.phase_function.clone()),
//             t,
//             1.0,
//             1.0,
//             true, // arbitrary
//         ))
//     }

//     fn bounding_box(&self, time_range: &std::ops::Range<Float>) -> Option<crate::aabb::Aabb> {
//         self.boundary.bounding_box(time_range)
//     }
// }
