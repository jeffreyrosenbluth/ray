use crate::geom::*;
use crate::object::*;
use rand::prelude::*;
pub trait Material: Send + Sync {
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut rng = thread_rng();
        let mut scatter_direction = rec.normal + random_unit_vector(&mut rng);
        if scatter_direction.near_zero() {
            // Catch degenerate scatter direction
            scatter_direction = rec.normal;
        }
        let scattered = Ray::new(rec.p, scatter_direction, r_in.time);
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
        let reflected = reflect(r_in.direction.normalize(), rec.normal);
        let scattered = Ray::new(
            rec.p,
            reflected + self.fuzz * random_in_unit_sphere(&mut rng),
            r_in.time,
        );
        if dot(scattered.direction, rec.normal) > 0.0 {
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

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
        let attenuation = WHITE;
        let refraction_ratio = if hit.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = r_in.direction.normalize();
        let cos_theta = dot(-unit_direction, hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction =
            if cannot_refract || schlick(cos_theta, refraction_ratio) > thread_rng().gen::<f64>() {
                reflect(unit_direction, hit.normal)
            } else {
                refract(unit_direction, hit.normal, refraction_ratio)
            };
        let scattered = Ray::new(hit.p, direction, r_in.time);
        Some((attenuation, scattered))
    }
}
