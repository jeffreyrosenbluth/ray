use crate::geom::*;
use rand::prelude::*;
use std::rc::Rc;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(
        p: Point3,
        normal: Vec3,
        material: Rc<dyn Material>,
        t: f64,
        front_face: bool,
    ) -> Self {
        Self {
            p,
            normal: normal.normalize(),
            material,
            t,
            front_face,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        let front_face = r.direction.dot(outward_normal) < 0.0;
        self.normal = if front_face {
            outward_normal.normalize()
        } else {
            -outward_normal.normalize()
        };
    }
}

pub trait Object {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    // fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
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
        let mut temp_rec = None;
        let mut closest_so_far = t_max;
        for object in &self.objects {
            if let Some(rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                temp_rec = Some(rec);
            }
        }
        temp_rec
    }
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(a: Color) -> Lambertian {
        Lambertian { albedo: a }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut rng = thread_rng();
        let mut scatter_direction = rec.normal + random_unit_vector(&mut rng);
        if scatter_direction.near_zero() {
            // Catch degenerate scatter direction
            scatter_direction = rec.normal;
        }
        let scattered = Ray::new(rec.p, scatter_direction);
        Some((self.albedo, scattered))
    }
}
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut rng = thread_rng();
        let reflected = r_in.direction.normalize().reflect(rec.normal);
        let scattered = Ray::new(
            rec.p,
            reflected + self.fuzz * random_in_unit_sphere(&mut rng),
        );
        if scattered.direction.dot(rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Dielectric {
        Dielectric {
            ir: index_of_refraction,
        }
    }
}

fn schlick(cosine: f64, ir: f64) -> f64 {
    let mut r0 = (1.0 - ir) / (1.0 + ir);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

impl Material for Dielectric{
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let mut rng = thread_rng();
        let reflected: Vec3 = r_in.direction.reflect(hit_record.normal);
        let attenuation = ONE;
        let (outward_normal, ni_over_nt, cosine) =
            if r_in.direction.dot(hit_record.normal) > 0.0 {
                let cosine: f64 = self.ir * r_in.direction.dot(hit_record.normal) / r_in.direction.length();
                (-hit_record.normal, self.ir, cosine)
            } else {
                let cosine: f64 = -r_in.direction.dot(hit_record.normal) / r_in.direction.length();
                (hit_record.normal, 1.0 / self.ir, cosine)
            };

        let reflect_prob = schlick(cosine, self.ir);
        let r: f64 = rng.gen();
        match r_in.direction.refract(outward_normal, ni_over_nt) {
            Some(refracted) if reflect_prob <= r => {
                Some((
                    attenuation,
                    Ray {origin: hit_record.p, direction: refracted}
                ))
            },
            _ => {
               Some((
                    attenuation,
                    Ray{origin: hit_record.p, direction: reflected}
                ))
            }
        }
    }
}