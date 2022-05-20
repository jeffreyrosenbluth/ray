use crate::aabb::*;
use crate::geom::*;
use crate::material::Material;
use crate::object::*;
use std::f32::consts::PI;
use std::ops::Range;
use std::sync::Arc;
use rand::rngs::SmallRng;
use rand::Rng;

pub struct Sphere {
    pub center0: Point3,
    pub center1: Point3,
    pub radius: Float,
    pub material: Arc<dyn Material>,
    pub time_range: Range<Float>,
}

impl Sphere {
    pub fn new_moving(
        center0: Point3,
        center1: Point3,
        radius: Float,
        material: Arc<dyn Material>,
        time_range: Range<Float>,
    ) -> Self {
        Self {
            center0,
            center1,
            radius,
            material,
            time_range,
        }
    }

    pub fn new(center0: Point3, radius: Float, material: Arc<dyn Material>) -> Self {
        Self {
            center0,
            center1: center0,
            radius,
            material,
            time_range: 0.0..0.0,
        }
    }

    pub fn center(&self, time: Float) -> Point3 {
        if self.time_range.is_empty() {
            return self.center0;
        }
        self.center0
            + ((time - self.time_range.start) / (self.time_range.end - self.time_range.start))
                * (self.center1 - self.center0)
    }
}

/// Returns (u, v)
pub fn sphere_uv(p: Point3) -> (Float, Float) {
    let theta = (-p.y).acos();
    let phi = (-p.z).atan2(p.x) + PI;
    (phi / (2.0 * PI), theta / PI)
}

impl Object for Sphere {
    fn hit(&self, r: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
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
        let rec = HitRecord::with_ray(r, p, outward_normal, self.material.clone(), root, u, v);
        Some(rec)
    }

    fn bounding_box(&self, time_range: &Range<Float>) -> Option<crate::aabb::Aabb> {
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

    fn pdf_value(&self, o: Vec3, v: Vec3) -> Float {
        if let Some(_hit) = self.hit(&Ray::new(o, v, 0.0), 0.001, f32::MAX) {
            let cos_theta_max = (1.0 - self.radius * self.radius / (self.center0 - o).length2()).sqrt();
            let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);
            1.0 / solid_angle
        } else {
            1.0
        }
    }

    fn random(&self, rng: &mut SmallRng, o: Vec3) -> Vec3 {
        let direction = self.center0 - o;
        let distance_squared = direction.length2();
        let uvw = Onb::build_from_w(direction);
        uvw.local(random_to_sphere(rng, self.radius, distance_squared))
    }
}

fn random_to_sphere(rng: &mut SmallRng, radius: f32, distance_squared: f32) -> Vec3 {
    let r1: Float = rng.gen();
    let r2: Float = rng.gen();
    let z = 1.0 + r2 * ((1.0 - radius.powi(2) / distance_squared).sqrt() - 1.0);
    let phi = 2.0 * PI * r1;
    let x = phi.cos() * (1.0 - z * z).sqrt();
    let y = phi.sin() * (1.0 - z * z).sqrt();
    Vec3::new(x, y, z)
}