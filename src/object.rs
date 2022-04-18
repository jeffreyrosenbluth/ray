use crate::aabb::*;
use crate::geom::*;
use crate::material::Material;
use std::ops::Range;
use std::sync::Arc;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
    pub time: f64,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3, time: f64) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(
        p: Point3,
        normal: Vec3,
        material: Arc<dyn Material>,
        u: f64,
        v: f64,
        t: f64,
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
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time_range: &Range<f64>) -> Option<Aabb>;
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

    pub fn add(&mut self, object: Box<dyn Object>) {
        self.objects.push(object);
    }
}

impl Object for Objects {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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

    fn bounding_box(&self, time_range: &Range<f64>) -> Option<Aabb> {
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

pub struct Translate<T> {
    pub object: T,
    pub offset: Vec3,
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
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_r = Ray::new(r.origin - self.offset, r.direction, r.time);
        if let Some(mut rec) = self.object.hit(&moved_r, t_min, t_max) {
            rec.p += self.offset;
            rec.set_face_normal(&moved_r, rec.normal);
            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, time_range: &Range<f64>) -> Option<Aabb> {
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

pub struct RotateY<T> {
    pub object: T,
    pub sin: f64,
    pub cos: f64,
    pub bbox: Option<Aabb>,
}

impl<T> RotateY<T>
where
    T: Object,
{
    pub fn new(object: T, degrees: f64) -> Self {
        let theta = degrees * PI / 180.0;
        let sin = theta.sin();
        let cos = theta.cos();
        let mut rect = Aabb::EMPTY;
        let bbox = object.bounding_box(&(0.0..1.0)).map(|b| {
            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let x = i as f64 * b.box_max.x + (1.0 - i as f64) * b.box_min.x;
                        let y = j as f64 * b.box_max.y + (1.0 - j as f64) * b.box_min.y;
                        let z = k as f64 * b.box_max.z + (1.0 - k as f64) * b.box_min.z;
                        let newx = cos * x + sin * z;
                        let newz = -sin * x + cos * z;
                        let tester = vec3(newx, y, newz);
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
            object,
            sin,
            cos,
            bbox,
        }
    }
}

impl<T> Object for RotateY<T>
where
    T: Object,
{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = r.origin;
        let mut direction = r.direction;
        origin[0] = self.cos * r.origin[0] - self.sin * r.origin[2];
        origin[2] = self.sin * r.origin[0] + self.cos * r.origin[2];
        direction[0] = self.cos * r.direction[0] - self.sin * r.direction[2];
        direction[2] = self.sin * r.direction[0] + self.cos * r.direction[2];
        let rotated_r = Ray::new(origin, direction, r.time);
        let hr = self.object.hit(&rotated_r, t_min, t_max).map(|mut rec| {
            let mut p = rec.p;
            let mut normal = rec.normal;
            p[0] = self.cos * rec.p[0] + self.sin * rec.p[2];
            p[2] = -self.sin * rec.p[0] + self.cos * rec.p[2];
            normal[0] = self.cos * rec.normal[0] + self.sin * rec.normal[2];
            normal[2] = -self.sin * rec.normal[0] + self.cos * rec.normal[2];
            rec.p = p;
            rec.set_face_normal(&rotated_r, normal);
            rec
        });
        hr
    }

    fn bounding_box(&self, _time_range: &Range<f64>) -> Option<Aabb> {
        self.bbox
    }
}