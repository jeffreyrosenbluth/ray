use rand::prelude::*;

use crate::geom::*;
use crate::object::*;

#[derive(Debug, Clone)]
pub struct Camera {
    pub origin: Point3,
    pub lookat: Point3,
    pub aperture: Float,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point3,
    u: Vec3,
    v: Vec3,
    exposure: std::ops::Range<Float>,
}

impl Camera {
    pub fn new(
        origin: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: Float,
        aspect_ratio: Float,
        aperture: Float,
        focus_dist: Float,
        exposure: std::ops::Range<Float>,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let viewport_height = 2.0 * (theta / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;
        let w = (origin - lookat).normalize();
        let u = cross(vup, w).normalize();
        let v = cross(w, u);

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
            exposure,
        }
    }

    pub fn basic(
        origin: Point3,
        lookat: Point3,
        vfov: Float,
        aspect_ratio: Float,
        aperture: Float,
        focus_dist: Float,
    ) -> Self {
        Self::new(
            origin,
            lookat,
            vec3(0.0, 1.0, 0.0),
            vfov,
            aspect_ratio,
            aperture,
            focus_dist,
            0.0..1.0,
        )
    }

    pub fn get_ray(&self, s: Float, t: Float) -> Ray {
        let mut rng = thread_rng();
        let rd = self.aperture / 2.0 * random_in_unit_disk(&mut rng);
        let offset = self.u * rd.x + self.v * rd.y;
        let time = rng.gen_range(self.exposure.start..self.exposure.end);
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
            time,
        )
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera::new(
            point3(0.0, 0.0, 1.0),
            point3(0.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
            45.0,
            16.0 / 9.0,
            0.0,
            1.0,
            0.0..0.0,
        )
    }
}
