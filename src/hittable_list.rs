use crate::hittable::Hittable;
use crate::hittable::HitRecord;
use std::vec::Vec;

pub struct HittableList<Object> {
    objects: Vec<Object>
}

impl<Object> HittableList<Object> {
    pub fn new() -> HittableList<Object> {
        return HittableList { objects: Vec::new() };
    }

    pub fn add(&mut self, object: Object) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl<Object: Hittable> HittableList<Object> {
    pub fn hit(&self, r: crate::math::ray::Ray, t_min: f64, t_max: f64) -> (bool, HitRecord) {
        let mut hit_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            let (hit, rec) = object.hit(r, t_min, closest_so_far);
            if hit {
                hit_anything = true;
                closest_so_far = rec.t;
                hit_rec = rec;
            }
        }

        return (hit_anything, hit_rec);
    }
}