use crate::math::vec3::*;

#[derive(Clone, Copy)]
pub struct Ray {
    origin: Point3,
    dir: Vec3
}

impl Ray {
    pub fn new(origin: Point3, dir: Vec3) -> Ray {
        return Ray { origin, dir };
    }

    pub fn origin(self) -> Point3 {
        return self.origin;
    }

    pub fn dir(self) -> Vec3 {
        return self.dir;
    }

    pub fn at(self, t: f64) -> Point3 {
        return self.origin + self.dir * t;
    }
}