use rand::thread_rng;

use crate::geom::*;
use crate::ray::*;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub origin: Point3,
    pub lookat: Point3,
    pub aperture: f64,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point3,
    u: Vec3,
    v: Vec3,
}

impl Camera {
    pub fn new(
        origin: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let w = (origin - lookat).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;
        Self {
            origin,
            lookat,
            aperture,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let mut rng = thread_rng();
        let rd = self.aperture / 2.0 * random_in_unit_disk(&mut rng);
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}