use crate::math;
use crate::math::vec3::*;
use crate::math::ray::*;
use crate::material::*;

pub mod sphere;
pub mod triangle;
pub mod model;

#[derive(Clone, Copy)]
pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub mat: &'a Material,
    front_face: bool
}

impl HitRecord<'_> {
    pub fn new(mat: &Material) -> HitRecord {
        return HitRecord {
            p: Vec3 {x: 0.0, y: 0.0, z: 0.0},
            normal: Vec3 {x: 0.0, y: 0.0, z: 0.0},
            t: 0.0,
            mat,
            front_face: false 
        };
    }

    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        self.front_face = math::vec3::dot(r.dir(), outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }

    pub fn front_face(self) -> bool {
        return self.front_face;
    }
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> (bool, HitRecord);
}