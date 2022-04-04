use crate::geom::*;
use crate::ray::*;
use std::rc::Rc;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
        Self { center, radius, material }
    }
}

impl Object for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.length2();
        let half_b = oc.dot(r.direction);
        let c = oc.length2() - self.radius * self.radius;
    
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {return None};
        let sqrtd = discriminant.sqrt();
    
        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None };
        }
        let p = r.at(root);
        let mut rec = HitRecord {
            t: root,
            p,
            material: self.material.clone(),
            normal: ZERO,
            front_face: true,
        };
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        Some(rec)
    } 
}