use crate::geom::*;
use crate::material::Material;
use std::sync::Arc;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
    pub time: f64,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3, time: f64) -> Self {
        Self { origin, direction, time }
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
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(
        p: Point3,
        normal: Vec3,
        material: Arc<dyn Material>,
        t: f64,
        front_face: bool,
    ) -> Self {
        Self {
            p,
            normal,
            material,
            t,
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
}

pub struct Objects {
    objects: Vec<Box<dyn Object>>,
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
}
