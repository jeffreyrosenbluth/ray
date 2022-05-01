use rand::distributions::{Distribution, Standard};
use rand::prelude::*;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub type Float = f32;

pub const INFINITY: Float = std::f32::MAX;
pub const PI: Float = std::f32::consts::PI;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

pub type Point3 = Vec3;
pub type Color = Vec3;
pub const BLACK: Color = Color {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

pub const WHITE: Color = Color {
    x: 1.0,
    y: 1.0,
    z: 1.0,
};

pub const ZERO: Vec3 = Vec3 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

pub const ONE: Vec3 = Vec3 {
    x: 1.0,
    y: 1.0,
    z: 1.0,
};

pub fn degrees_to_radians(degrees: Float) -> Float {
    degrees * PI / 180.0
}

pub fn vec3(x: Float, y: Float, z: Float) -> Vec3 {
    Vec3::new(x, y, z)
}

pub fn point3(x: Float, y: Float, z: Float) -> Point3 {
    Point3::new(x, y, z)
}

pub fn color(r: Float, g: Float, b: Float) -> Color {
    point3(r, g, b)
}

impl Vec3 {
    pub fn new(x: Float, y: Float, z: Float) -> Self {
        Self { x, y, z }
    }

    pub fn length2(self) -> Float {
        dot(self, self)
    }

    pub fn length(self) -> Float {
        self.length2().sqrt()
    }

    pub fn normalize(self) -> Vec3 {
        self / self.length()
    }

    pub fn near_zero(self) -> bool {
        const EPS: Float = 1.0e-8;
        self.x.abs() < EPS && self.y.abs() < EPS && self.z.abs() < EPS
    }

    pub fn map(self, f: fn(Float) -> Float) -> Self {
        Self {
            x: f(self.x),
            y: f(self.y),
            z: f(self.z),
        }
    }
}

pub fn dist2(v: Vec3, w: Vec3) -> Float {
    vec3(v.x - w.x, v.y - w.y, v.z - w.z).length2()
}

pub fn dist(v: Vec3, w: Vec3) -> Float {
    vec3(v.x - w.x, v.y - w.y, v.z - w.z).length()
}

pub fn dot(v: Vec3, w: Vec3) -> Float {
    v.x * w.x + v.y * w.y + v.z * w.z
}

pub fn cross(v: Vec3, w: Vec3) -> Vec3 {
    vec3(
        v.y * w.z - v.z * w.y,
        v.z * w.x - v.x * w.z,
        v.x * w.y - v.y * w.x,
    )
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * dot(v, n) * n
}

pub fn refract(v: Vec3, n: Vec3, eta_ratio: Float) -> Vec3 {
    let uv = v.normalize();
    let dt = dot(uv, n);
    let discriminant = 1.0 - eta_ratio * eta_ratio * (1.0 - dt * dt);
    eta_ratio * (uv - n * dt) - n * discriminant.sqrt()
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        vec3(-self.x, -self.y, -self.z)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        vec3(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        vec3(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl Mul<Float> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Float) -> Self::Output {
        vec3(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl MulAssign<Float> for Vec3 {
    fn mul_assign(&mut self, rhs: Float) {
        *self = *self * rhs;
    }
}

impl Div<Float> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Float) -> Self::Output {
        if rhs == 0.0 {
            panic!("Tried to divide a Vec3 by 0")
        }
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl DivAssign<Float> for Vec3 {
    fn div_assign(&mut self, rhs: Float) {
        *self = *self / rhs;
    }
}

impl Mul<Vec3> for Float {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Div<Vec3> for Float {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        if self == 0.0 {
            panic!("Tried to divide a Vec3 by 0")
        }
        rhs / self
    }
}

impl Distribution<Vec3> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3 {
        let x: Float = rng.gen();
        let y: Float = rng.gen();
        let z: Float = rng.gen();
        Vec3 { x, y, z }
    }
}

pub fn rand_in_cube<R: Rng>(rng: &mut R) -> Vec3 {
    let x: Float = rng.gen_range(-1.0..1.0);
    let y: Float = rng.gen_range(-1.0..1.0);
    let z: Float = rng.gen_range(-1.0..1.0);
    Vec3 { x, y, z }
}

pub fn random_in_unit_sphere<R: Rng>(rng: &mut R) -> Vec3 {
    let mut p: Point3;
    loop {
        p = rand_in_cube(rng);
        if p.length2() >= 1.0 {
            continue;
        }
        break;
    }
    p
}

pub fn random_unit_vector<R: Rng>(rng: &mut R) -> Vec3 {
    random_in_unit_sphere(rng).normalize()
}

pub fn random_in_unit_disk<R: Rng>(rng: &mut R) -> Vec3 {
    loop {
        let p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
        if p.length() < 1.0 {
            return p;
        }
    }
}

pub fn rand_color<R: Rng>(rng: &mut R, range: std::ops::Range<Float>) -> Color {
    let x: Float = rng.gen_range(range.clone());
    let y: Float = rng.gen_range(range.clone());
    let z: Float = rng.gen_range(range);
    Color { x, y, z }
}

pub fn rand_point<R: Rng>(rng: &mut R, range: std::ops::Range<Float>) -> Color {
    let x: Float = rng.gen_range(range.clone());
    let y: Float = rng.gen_range(range.clone());
    let z: Float = rng.gen_range(range);
    Point3 { x, y, z }
}

impl std::ops::Index<u8> for Vec3 {
    type Output = Float;

    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out or range for Vec3"),
        }
    }
}

impl std::ops::IndexMut<u8> for Vec3 {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out or range for Vec3"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    pub const fn order(self) -> (Axis, Axis, Axis) {
        match self {
            Axis::X => (Axis::Y, Axis::Z, Axis::X),
            Axis::Y => (Axis::X, Axis::Z, Axis::Y),
            Axis::Z => (Axis::X, Axis::Y, Axis::Z),
        }
    }
}

impl std::ops::Index<Axis> for Vec3 {
    type Output = Float;

    fn index(&self, index: Axis) -> &Self::Output {
        match index {
            Axis::X => &self.x,
            Axis::Y => &self.y,
            Axis::Z => &self.z,
            _ => panic!("Index out or range for Vec3"),
        }
    }
}

impl std::ops::IndexMut<Axis> for Vec3 {
    fn index_mut(&mut self, index: Axis) -> &mut Self::Output {
        match index {
            Axis::X => &mut self.x,
            Axis::Y => &mut self.y,
            Axis::Z => &mut self.z,
            _ => panic!("Index out or range for Vec3"),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_add() {
        assert_eq!(
            Vec3::new(1.0, 0.0, -1.0) + Vec3::new(2.0, 4.0, 6.0),
            Vec3::new(3.0, 4.0, 5.0)
        )
    }

    #[test]
    fn test_add_assign() {
        let mut x = Vec3::new(1.0, 0.0, -1.0);
        x += Vec3::new(2.0, 4.0, 6.0);
        assert_eq!(x, Vec3::new(3.0, 4.0, 5.0))
    }

    #[test]
    fn test_sub() {
        assert_eq!(
            Vec3::new(1.0, 0.0, -1.0) - Vec3::new(2.0, 4.0, 6.0),
            Vec3::new(-1.0, -4.0, -7.0)
        )
    }
    #[test]
    fn test_sub_assign() {
        let mut x = Vec3::new(1.0, 0.0, -1.0);
        x -= Vec3::new(2.0, 4.0, 6.0);
        assert_eq!(x, Vec3::new(-1.0, -4.0, -7.0))
    }
    #[test]
    fn test_dot() {
        assert_eq!(dot(Vec3::new(1.0, 0.0, -1.0), ONE), 0.0);
    }

    #[test]
    fn test_mul_assign() {
        let mut x = Vec3::new(1.0, 0.0, -1.0);
        x *= 2.0;
        assert_eq!(x, Vec3::new(2.0, 0.0, -2.0));
    }
    #[test]
    fn test_mul_Float() {
        assert_eq!(Vec3::new(1.0, 0.0, -1.0) * 1.0, Vec3::new(1.0, 0.0, -1.0));
    }
    #[test]
    fn test_div() {
        assert_eq!(Vec3::new(1.0, -2.0, 0.0) / 2.0, Vec3::new(0.5, -1.0, 0.0));
    }
    #[test]
    fn test_cross() {
        assert_eq!(
            cross(Vec3::new(1.0, 2.0, 3.0), Vec3::new(2.0, 3.0, 4.0)),
            Vec3::new(8.0 - 9.0, 6.0 - 4.0, 3.0 - 4.0)
        );
    }
    #[test]
    fn test_neg() {
        assert_eq!(-Vec3::new(1.0, -2.0, 3.0), Vec3::new(-1.0, 2.0, -3.0));
    }

    #[test]
    fn test_squared_length() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0).length2(), 14.0 as Float);
    }

    #[test]
    fn test_length() {
        assert_eq!(
            Vec3::new(3.0, 4.0, 5.0).length(),
            ((3.0 * 3.0 + 4.0 * 4.0 + 5.0 * 5.0) as Float).sqrt()
        );
    }
    #[test]
    fn test_unit() {
        assert_eq!(
            Vec3::new(233.0, 0.0, 0.0).normalize(),
            Vec3::new(1.0, 0.0, 0.0)
        );
        assert_eq!(
            Vec3::new(-233.0, 0.0, 0.0).normalize(),
            Vec3::new(-1.0, 0.0, 0.0)
        );
    }
    #[test]
    #[should_panic]
    fn test_unit_panic() {
        Vec3::new(0.0, 0.0, 0.0).normalize();
    }
}
