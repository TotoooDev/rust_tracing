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
    pub fn new(list: HittableList) -> BVH {
        return Self::real_new(list.objects(), 0, list.objects().len());
    }

    fn real_new(mut objects: Vec<Box<dyn Hittable>>, start: usize, end: usize) -> BVH {
        let mut left: Box<dyn Hittable>;
        let mut right: Box<dyn Hittable>;

        let axis = rand::random::<u32>() % 3;
        let mut comparator;
        if axis == 0 {
            comparator = Self::box_compare_x;
        }
        else if axis == 1 {
            comparator = Self::box_compare_y;
        }
        else {
            comparator = Self::box_compare_z;
        }

        let object_span = end - start;

        if object_span == 1 {
            left = objects[start];
            right = objects[start];
        }
        else if object_span == 2 {
            if comparator(objects[start], objects[start + 1]) {
                left = objects[start];
                right = objects[start + 1];
            }
            else {
                left = objects[start + 1];
                right = objects[start];
            }
        }
        else {
            objects.sort_by(comparator);

            let mid = start + object_span / 2;
            left = Box::new(BVH::real_new(objects, start, mid));
            right = Box::new(BVH::real_new(objects, mid, end));
        }

        let bounding_box_left = left.bounding_box();
        let bounding_box_right = right.bounding_box();

        return BVH {
            left,
            right,
            bounding_box: surrounding_box(bounding_box_left.1, bounding_box_right.1)
        };
    }

    fn box_compare(a: Box<dyn Hittable>, b: Box<dyn Hittable>, axis: Axis) -> bool {
        let (hit_a, box_a) = a.bounding_box();
        let (hit_b, box_b) = a.bounding_box();

        if !hit_a || !hit_b {
            println!("No bounding box in bvh_node constructor");
        }

        match axis {
            X => { return box_a.minimum.x < box_b.minimum.x; },
            Y => { return box_a.minimum.y < box_b.minimum.y; },
            Z => { return box_a.minimum.z < box_b.minimum.z; }
        }
    }

    fn box_compare_x(a: Box<dyn Hittable>, b: Box<dyn Hittable>) -> bool {
        return Self::box_compare(a, b, Axis::X);
    }

    fn box_compare_y(a: Box<dyn Hittable>, b: Box<dyn Hittable>) -> bool {
        return Self::box_compare(a, b, Axis::Y);
    }

    fn box_compare_z(a: Box<dyn Hittable>, b: Box<dyn Hittable>) -> bool {
        return Self::box_compare(a, b, Axis::Z);
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