use crate::aabb::*;
use crate::geom::*;
use crate::object::*;
use rand::prelude::*;
use std::cmp::Ordering;
use std::ops::Range;
use std::sync::Arc;

pub struct BvhNode {
    pub left: Arc<Geometry>,
    pub right: Arc<Geometry>,
    pub bbox: Aabb,
    transform: Mat4,
    inv_transform: Mat4,
}

impl BvhNode {
    pub fn new(objects: &mut Objects, start: usize, end: usize, time: Range<f32>) -> Self {
        let mut rng = rand::thread_rng();
        let axis: i32 = rng.gen_range(0..3);
        let comparator = match axis {
            0 => Self::x_comparator,
            1 => Self::y_comparator,
            2 => Self::z_comparator,
            _ => unreachable!("Random int in range [0, 2] must be in range"),
        };
        let object_span = end - start;
        let (left, right) = match object_span {
            1 => {
                let first: Arc<Geometry> = objects.objects.remove(0).into();
                (first.clone(), first)
            }
            2 => {
                let first: Arc<Geometry> = objects.objects.remove(0).into();
                let second: Arc<Geometry> = objects.objects.remove(0).into();
                match comparator(&*first, &*second) {
                    Ordering::Less => (first, second),
                    _ => (second, first),
                }
            }
            _ => {
                objects.objects.sort_by(|x, y| comparator(x, y));
                let mid = start + object_span / 2;
                let left: Arc<Geometry> =
                    Arc::new(Geometry::bvh_node(objects, start, mid, time.clone()));
                let right: Arc<Geometry> =
                    Arc::new(Geometry::bvh_node(objects, mid, end, time.clone()));

                (left, right)
            }
        };

        let box_left = left.bounding_box(&time).unwrap();
        let box_right = right.bounding_box(&time).unwrap();
        let bbox = surrounding_box(box_left, box_right);
        let transform = Mat4::IDENTITY;
        let inv_transform = Mat4::IDENTITY;
        Self {
            left,
            right,
            bbox,
            transform,
            inv_transform,
        }
    }

    fn comparator(x: &Geometry, y: &Geometry, axis: usize) -> Ordering {
        let box_x = x.bounding_box(&(0.0..0.0)).unwrap();
        let box_y = y.bounding_box(&(0.0..0.0)).unwrap();
        let x = box_x.box_min[axis];
        let y = box_y.box_min[axis];
        if x < y {
            Ordering::Less
        } else if x > y {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
    pub fn x_comparator(x: &Geometry, y: &Geometry) -> Ordering {
        Self::comparator(x, y, 0)
    }
    pub fn y_comparator(x: &Geometry, y: &Geometry) -> Ordering {
        Self::comparator(x, y, 1)
    }
    pub fn z_comparator(x: &Geometry, y: &Geometry) -> Ordering {
        Self::comparator(x, y, 2)
    }
}

impl Object for BvhNode {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if !self.bbox.hit(ray, t_min, t_max) {
            return None;
        }
        let left_record = self.left.hit(ray, t_min, t_max);
        let t = if let Some(record) = &left_record {
            record.t
        } else {
            t_max
        };
        let right_record = self.right.hit(ray, t_min, t);
        right_record.or(left_record)
    }

    fn bounding_box(&self, _time_range: &std::ops::Range<f32>) -> Option<Aabb> {
        Some(self.bbox)
    }

    fn add_transform(&mut self, transform: Mat4) {
        self.transform = transform * self.transform;
        self.inv_transform = transform.inverse() * self.inv_transform;
    }
}
