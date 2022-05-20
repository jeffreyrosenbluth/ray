use crate::geom::*;
use crate::object::Object;
use rand::Rng;
use rand::rngs::SmallRng;
use std::sync::Arc;

pub trait Pdf {
    fn value(&self, direction: Vec3) -> Float;
    fn generate(&self, rng: &mut SmallRng) -> Vec3;
}

pub struct CosinePdf {
    pub uvw: Onb,
}

impl CosinePdf {
    pub fn new(uvw: Onb) -> Self {
        Self { uvw }
    }

    pub fn with_w(w: Vec3) -> Self {
        Self {
            uvw: Onb::build_from_w(w),
        }
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction: Vec3) -> Float {
        let cosine = dot(direction.normalize(), self.uvw.w);
        if cosine <= 0.0 {
            1.0
        } else {
            cosine / PI
        }
    }

    fn generate(&self, rng: &mut SmallRng) -> Vec3 {
        self.uvw.local(random_cosine_direction(rng))
    }
}

pub struct ObjectPdf<T>
where
    T: ?Sized,
{
    pub object: Arc<T>,
    pub o: Point3,
}

impl<T> ObjectPdf<T>
where
    T: ?Sized,
{
    pub fn new(object: Arc<T>, o: Point3) -> Self {
        Self { object, o }
    }
}

impl<T> Pdf for ObjectPdf<T>
where
    T: Object + ?Sized,
{
    fn value(&self, direction: Vec3) -> Float {
        self.object.pdf_value(self.o, direction)
    }

    fn generate(&self, rng: &mut SmallRng) -> Vec3 {
        self.object.random(rng, self.o)
    }
}

pub struct MixturePdf {
    pub p0: Arc<dyn Pdf>,
    pub p1: Arc<dyn Pdf>,
}

impl MixturePdf {
    pub fn new(p0: Arc<dyn Pdf>, p1: Arc<dyn Pdf>) -> Self {
        Self { p0, p1 }
    }
}

impl Pdf for MixturePdf {
    fn value(&self, direction: Vec3) -> Float {
        0.5 * self.p0.value(direction) + 0.5 * self.p1.value(direction)
    }

    fn generate(&self, rng: &mut SmallRng) -> Vec3 {
        let rd = rng.gen_bool(0.5);
        if rd {
            self.p0.generate(rng)
        } else {
            self.p1.generate(rng)
        }
    }
}