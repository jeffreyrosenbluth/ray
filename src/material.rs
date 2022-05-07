use crate::geom::*;
use crate::object::*;
use crate::texture::*;
use rand::prelude::*;
use std::sync::Arc;

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray, Float)>;
    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> Float;
    fn color_emitted(&self, rec: &HitRecord, _u: Float, _v: Float, _p: Point3) -> Color {
        BLACK
    }
}
pub struct Lambertian<T> {
    albedo: Arc<T>,
}

impl<T> Lambertian<T>
where
    T: Texture,
{
    pub fn new(t: T) -> Self {
        Lambertian {
            albedo: Arc::new(t),
        }
    }
}

impl Lambertian<Color> {
    pub fn solid_color(c: Color) -> Self {
        Lambertian {
            albedo: Arc::new(c),
        }
    }
}

pub fn lambertian(r: Float, g: Float, b: Float) -> Arc<Lambertian<Color>> {
    Arc::new(Lambertian::solid_color(color(r, g, b)))
}

pub fn lambertian_texture(texture: impl Texture) -> Arc<Lambertian<impl Texture>> {
    Arc::new(Lambertian::new(texture))
}

// onb uvw;
// uvw.build_from_w(rec.normal);
// auto direction = uvw.local(random_cosine_direction());
// scattered = ray(rec.p, unit_vector(direction), r_in.time());
// alb = albedo->value(rec.u, rec.v, rec.p);
// pdf = dot(uvw.w(), scattered.direction()) / pi;
// return true;

impl<T> Material for Lambertian<T>
where
    T: Texture,
{
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray, Float)> {
        let mut rng = thread_rng();
        let uvw = Onb::build_from_w(rec.normal);
        let direction = uvw.local(random_cosine_direction(&mut rng));
        let scattered = Ray::new(rec.p, direction.normalize(), r_in.time);
        let pdf = dot(uvw.w, scattered.direction) / PI;
        Some((self.albedo.value(rec.u, rec.v, rec.p), scattered, pdf))

        // let mut scatter_direction = rec.normal + random_unit_vector(&mut rng);
        // if scatter_direction.near_zero() {
        //     // Catch degenerate scatter direction
        //     scatter_direction = rec.normal;
        // }
        // let scattered = Ray::new(rec.p, scatter_direction.normalize(), r_in.time);
        // let mut pdf = dot(rec.normal, scattered.direction) / PI;
        // if pdf <= 0.0 {
        //     pdf = 1.0;
        // }
        // Some((self.albedo.value(rec.u, rec.v, rec.p), scattered, pdf))
    }

    fn scattering_pdf(&self, _r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> Float {
        let cosine = dot(rec.normal, scattered.direction.normalize()).max(0.0);
        cosine / PI
    }

    fn color_emitted(&self, _rec: &HitRecord, _u: Float, _v: Float, _p: Point3) -> Color {
        BLACK
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: Float,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: Float) -> Metal {
        Metal { albedo, fuzz }
    }
}

// impl Material for Metal {
//     fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
//         let mut rng = thread_rng();
//         let reflected = reflect(r_in.direction.normalize(), rec.normal);
//         let scattered = Ray::new(
//             rec.p,
//             reflected + self.fuzz * random_in_unit_sphere(&mut rng),
//             r_in.time,
//         );
//         if dot(scattered.direction, rec.normal) > 0.0 {
//             Some((self.albedo, scattered))
//         } else {
//             None
//         }
//     }
// }

pub fn metal(r: Float, g: Float, b: Float, fuzz: Float) -> Arc<Metal> {
    Arc::new(Metal::new(color(r, g, b), fuzz))
}

pub struct Dielectric {
    ir: Float,
}

impl Dielectric {
    pub fn new(index_of_refraction: Float) -> Dielectric {
        Dielectric {
            ir: index_of_refraction,
        }
    }
}

fn schlick(cosine: Float, ir: Float) -> Float {
    let mut r0 = (1.0 - ir) / (1.0 + ir);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

// impl Material for Dielectric {
//     fn scatter(&self, r_in: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
//         let attenuation = WHITE;
//         let refraction_ratio = if hit.front_face {
//             1.0 / self.ir
//         } else {
//             self.ir
//         };
//         let unit_direction = r_in.direction.normalize();
//         let cos_theta = dot(-unit_direction, hit.normal).min(1.0);
//         let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
//         let cannot_refract = refraction_ratio * sin_theta > 1.0;
//         let direction = if cannot_refract
//             || schlick(cos_theta, refraction_ratio) > thread_rng().gen::<Float>()
//         {
//             reflect(unit_direction, hit.normal)
//         } else {
//             refract(unit_direction, hit.normal, refraction_ratio)
//         };
//         let scattered = Ray::new(hit.p, direction, r_in.time);
//         Some((attenuation, scattered))
//     }
// }

pub fn dielectric(index_of_refraction: Float) -> Arc<Dielectric> {
    Arc::new(Dielectric::new(index_of_refraction))
}

pub struct DiffuseLight<T> {
    pub color: Arc<T>,
}

impl<T> DiffuseLight<T>
where
    T: Texture,
{
    pub fn new(c: T) -> Self {
        DiffuseLight { color: Arc::new(c) }
    }
}

impl<T> Material for DiffuseLight<T>
where
    T: Texture,
{
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Color, Ray, Float)> {
        None
    }

    fn color_emitted(&self, rec: &HitRecord, u: Float, v: Float, p: Point3) -> Color {
        if rec.front_face {
            self.color.value(u, v, p)
        } else {
            BLACK
        }
    }

    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> Float {
        1.0
    }
}

pub fn diffuse_light(r: Float, g: Float, b: Float) -> Arc<DiffuseLight<Color>> {
    Arc::new(DiffuseLight::new(color(r, g, b)))
}

#[derive(Clone)]
pub struct Isotropic<T> {
    albedo: T,
}

impl<T> Isotropic<T> {
    pub fn new(albedo: T) -> Self {
        Self { albedo }
    }
}

// impl<T> Material for Isotropic<T>
// where
//     T: Texture,
// {
//     fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
//         let mut rng = thread_rng();
//         let scattered = Ray::new(rec.p, random_in_unit_sphere(&mut rng), r_in.time);
//         let attenuation = self.albedo.value(rec.u, rec.v, rec.p);
//         Some((attenuation, scattered))
//     }
// }

pub fn isotropic(r: Float, g: Float, b: Float) -> Arc<Isotropic<Color>> {
    Arc::new(Isotropic::new(color(r, g, b)))
}
