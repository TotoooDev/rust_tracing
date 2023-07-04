use crate::math;

use crate::math::vec3::*;
use crate::math::ray::*;
use crate::hittable::*;
use crate::material::*;

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Material
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Material) -> Sphere {
        return Sphere { center, radius, mat };
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> (bool, HitRecord) {
        let oc = r.origin() - self.center;
        let a = r.dir().length_squared();
        let half_b = math::vec3::dot(oc, r.dir());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return (false, HitRecord::new());
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return (false, HitRecord::new());
            }
        }

        let mut rec = HitRecord::new();
        rec.t = root;
        rec.p = r.at(rec.t);
        rec.mat = self.mat;

        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        return (true, rec);
    }
}