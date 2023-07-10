use crate::math::vec3::*;
use crate::math::ray::*;

#[derive(Clone, Copy)]
pub struct AABB {
    pub minimum: Point3,
    pub maximum: Point3
}

pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
    let small = Point3::new(
        box0.minimum.x.min(box1.minimum.x),
        box0.minimum.y.min(box1.minimum.y),
        box0.minimum.z.min(box1.minimum.z)
    );

    let big = Point3::new(
        box0.maximum.x.max(box1.maximum.x),
        box0.maximum.y.max(box1.maximum.y),
        box0.maximum.z.max(box1.maximum.z)
    );

    return AABB::new(small, big);
}

impl AABB {
    pub fn new(minimum: Point3, maximum: Point3) -> AABB {
        return AABB {
            minimum,
            maximum
        };
    }

    // TODO: Try to implement the optimized version
    pub fn hit(self, r: Ray, mut t_min: f64, mut t_max: f64) -> bool {
        let t0 = ((self.minimum.x - r.origin().x) / r.dir().x).min((self.maximum.x - r.origin().x) / r.dir().x);
        let t1 = ((self.minimum.x - r.origin().x) / r.dir().x).max((self.maximum.x - r.origin().x) / r.dir().x);
        t_min = t_min.max(t0);
        t_max = t_max.max(t1);
        if t_max <= t_min {
            return false;
        }

        let t0 = ((self.minimum.y - r.origin().y) / r.dir().y).min((self.maximum.y - r.origin().y) / r.dir().y);
        let t1 = ((self.minimum.y - r.origin().y) / r.dir().y).max((self.maximum.y - r.origin().y) / r.dir().y);
        t_min = t_min.max(t0);
        t_max = t_max.max(t1);
        if t_max <= t_min {
            return false;
        }

        let t0 = ((self.minimum.z - r.origin().z) / r.dir().z).min((self.maximum.z - r.origin().z) / r.dir().z);
        let t1 = ((self.minimum.z - r.origin().z) / r.dir().z).max((self.maximum.z - r.origin().z) / r.dir().z);
        t_min = t_min.max(t0);
        t_max = t_max.max(t1);
        if t_max <= t_min {
            return false;
        }

        return true;
    }
}