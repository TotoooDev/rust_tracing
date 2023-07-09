use crate::hittable::*;
use crate::aabb::*;
use crate::hittable_list::HittableList;
use crate::material::*;
use crate::math::vec3::*;

pub struct BVH {
    pub left: Box<dyn Hittable>,
    pub right: Box<dyn Hittable>,
    pub bounding_box: AABB
}

impl BVH {
    pub fn new(mut objects: Vec<Box<dyn Hittable>>) -> BVH {
        
    }
}

impl Hittable for BVH {
    fn hit(&self, r: crate::math::ray::Ray, t_min: f64, t_max: f64) -> (bool, HitRecord) {
        if !self.bounding_box.hit(r, t_min, t_max) {
            return (false, HitRecord::new(&Material::new(Color::new(0.0, 0.0, 0.0), MaterialType::LAMBERTIAN)));
        }

        let hit_left = self.left.hit(r, t_min, t_max);
        let hit_right = self.right.hit(r, t_min, t_max);

        if hit_left.0 || hit_right.0 {
            return hit_left; // We don't actually care about the hit record here
        }

        return (false, HitRecord::new(&Material::new(Color::new(0.0, 0.0, 0.0), MaterialType::LAMBERTIAN)));
    }

    fn bounding_box(&self) -> (bool, AABB) {
        return (true, self.bounding_box);
    }
}