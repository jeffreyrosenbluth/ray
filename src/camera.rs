use crate::geom::*;
use crate::ray::*;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,
    pub vfov: f64, // degrees
    pub aspect_ratio: f64,
    pub theta: f64,
    pub h: f64,
    pub viewport_height: f64,
    pub viewport_width: f64,
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point3,
}

impl Camera {
    pub fn new(lookfrom: Point3, lookat: Point3, vup: Vec3, vfov: f64, aspect_ratio: f64) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        let origin = lookfrom;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;
        Self {
            lookfrom,
            lookat,
            vup,
            vfov,
            aspect_ratio,
            theta,
            h,
            viewport_height,
            viewport_width,
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}

impl Default for Camera {
    fn default() -> Self {
        let lookfrom = point3(-2.0, 2.0, 1.0);
        let lookat = point3(0.0, 0.0, -1.0);
        let vup = vec3(0.0, 1.0, 0.0);
        let aspect_ratio = 16.0 / 9.0;
        let vfov = 90.0;
        Self::new(lookfrom, lookat, vup, vfov, aspect_ratio)
    }
}
