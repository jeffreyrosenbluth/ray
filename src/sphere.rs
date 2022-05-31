use crate::aabb::*;
use crate::geom::*;
use crate::material::Material;
use crate::object::*;
use rand::rngs::SmallRng;
use rand::Rng;
use std::f32::consts::PI;
use std::ops::Range;
use std::sync::Arc;

pub struct Sphere {
    pub center0: Point3,
    pub center1: Point3,
    pub radius: f32,
    transform: Mat4,
    inv_transform: Mat4,
    pub material: Arc<dyn Material>,
    pub time_range: Range<f32>,
}

impl Sphere {
    pub fn new_moving(
        center0: Point3,
        center1: Point3,
        radius: f32,
        material: Arc<dyn Material>,
        time_range: Range<f32>,
    ) -> Self {
        let transform = Mat4::IDENTITY;
        let inv_transform = Mat4::IDENTITY;
        Self {
            center0,
            center1,
            radius,
            transform,
            inv_transform,
            material,
            time_range,
        }
    }

    pub fn new(center0: Point3, radius: f32, material: Arc<dyn Material>) -> Self {
        let transform = Mat4::IDENTITY;
        let inv_transform = Mat4::IDENTITY;
        Self {
            center0,
            center1: center0,
            radius,
            transform,
            inv_transform,
            material,
            time_range: 0.0..0.0,
        }
    }

    pub fn center(&self, time: f32) -> Point3 {
        if self.time_range.is_empty() {
            return self.center0;
        }
        self.center0
            + ((time - self.time_range.start) / (self.time_range.end - self.time_range.start))
                * (self.center1 - self.center0)
    }

    pub fn set_transform(mut self, transform: Mat4) -> Self {
        self.transform = transform;
        self.inv_transform = transform.inverse();
        self
    }
}

/// Returns (u, v)
pub fn sphere_uv(p: Point3) -> (f32, f32) {
    let theta = (-p.y).acos();
    let phi = (-p.z).atan2(p.x) + PI;
    (phi / (2.0 * PI), theta / PI)
}

impl Object for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let r = r.transform(self.inv_transform);
        let oc = r.origin - self.center(r.time);
        let a = r.direction.length_squared();
        let half_b = dot(oc, r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

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
        let outward_normal = self
            .inv_transform
            .transpose()
            .transform_vector3((p - self.center(r.time)) / self.radius)
            .normalize();
        let (u, v) = sphere_uv(outward_normal);
        let rec = HitRecord::with_ray(
            &r,
            self.transform.transform_point3(p),
            outward_normal,
            self.material.clone(),
            root,
            u,
            v,
        );
        Some(rec)
    }

    fn bounding_box(&self, time_range: &Range<f32>) -> Option<crate::aabb::Aabb> {
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

    fn pdf_value(&self, o: Vec3, v: Vec3) -> f32 {
        if let Some(_hit) = self.hit(&Ray::new(o, v, 0.0), 0.001, f32::MAX) {
            let cos_theta_max =
                (1.0 - self.radius * self.radius / (self.center0 - o).length_squared()).sqrt();
            let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);
            1.0 / solid_angle
        } else {
            1.0
        }
    }

    fn add_transform(&mut self, transform: Mat4) {
        self.transform = transform * self.transform;
        self.inv_transform = transform.inverse() * self.inv_transform;
    }

    fn random(&self, rng: &mut SmallRng, o: Vec3) -> Vec3 {
        let direction = self.center0 - o;
        let distance_squared = direction.length_squared();
        let uvw = Onb::build_from_w(direction);
        uvw.local(random_to_sphere(rng, self.radius, distance_squared))
    }
}

fn random_to_sphere(rng: &mut SmallRng, radius: f32, distance_squared: f32) -> Vec3 {
    let r1: f32 = rng.gen();
    let r2: f32 = rng.gen();
    let z = 1.0 + r2 * ((1.0 - radius.powi(2) / distance_squared).sqrt() - 1.0);
    let phi = 2.0 * PI * r1;
    let x = phi.cos() * (1.0 - z * z).sqrt();
    let y = phi.sin() * (1.0 - z * z).sqrt();
    Vec3::new(x, y, z)
}
