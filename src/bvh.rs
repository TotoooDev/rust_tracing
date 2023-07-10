use std::cmp::Ordering;

use crate::hittable::*;
use crate::aabb::*;
use crate::hittable_list::HittableList;
use crate::material::*;
use crate::math::vec3::*;

pub struct BVH {
    pub left: Option<Box<dyn Hittable>>,
    pub right: Option<Box<dyn Hittable>>,
    pub bounding_box: AABB
}

impl BVH {
    pub fn new(mut list: HittableList) -> BVH {
        let len = list.objects.len();
        return Self::real_new(&mut list.objects, 0, len);
    }

    fn real_new(objects: &mut Vec<Box<dyn Hittable>>, start: usize, end: usize) -> BVH {
        // Randomly choose an axis
        let random = rand::random::<u32>() % 3;
        let random_axis = match random {
            0 => { Axis::X },
            1 => { Axis::Y },
            2 => { Axis::Z },
            _ => { Axis::X } // bruh
        };

        // Sort the primitives
        let comparator_function: fn (&Box<dyn Hittable>, &Box<dyn Hittable>) -> Ordering = match random_axis {
            Axis::X => { Self::box_compare_x },
            Axis::Y => { Self::box_compare_x },
            Axis::Z => { Self::box_compare_x }
        };

        let left: Option<Box<dyn Hittable>>;
        let right: Option<Box<dyn Hittable>>;
        let bounding_box: AABB;

        let object_span = end - start;
        if object_span == 1 {
            left = Some(objects[start]);
            right = Some(objects[start]);
        }
        else if object_span == 2 {
            if comparator_function(&objects[start], &objects[start + 1]) == Ordering::Less {
                left = Some(objects[start]);
                right = Some(objects[start + 1]);
            }
            else {
                left = Some(objects[start + 1]);
                right = Some(objects[start]);
            }
        }
        else {
            objects.sort_by(comparator_function);
            
            let mid = start + object_span / 2;
            left = Some(Box::new(BVH::real_new(objects, start, mid)));
            right = Some(Box::new(BVH::real_new(objects, mid, end)));
        }

        let (_hit_left, box_left) = Option::expect(left.as_ref(), "a").bounding_box();
        let (_hit_right, box_right) = Option::expect(right.as_ref(), "a").bounding_box();
        bounding_box = surrounding_box(box_left, box_right);

        return BVH {
            left,
            right,
            bounding_box
        };
    }

    fn box_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>, axis: Axis) -> Ordering {
        let (hit_a, box_a) = a.bounding_box();
        let (hit_b, box_b) = b.bounding_box();

        if !hit_a || !hit_b {
            println!("No bounding box in bvh_node constructor");
        }

        match axis {
            Axis::X => { if box_a.minimum.x < box_b.minimum.x { return Ordering::Less; } else if box_a.minimum.x == box_b.minimum.x { return Ordering::Equal } else { return Ordering::Greater } },
            Axis::Y => { if box_a.minimum.y < box_b.minimum.y { return Ordering::Less; } else if box_a.minimum.y == box_b.minimum.y { return Ordering::Equal } else { return Ordering::Greater } },
            Axis::Z => { if box_a.minimum.z < box_b.minimum.z { return Ordering::Less; } else if box_a.minimum.z == box_b.minimum.z { return Ordering::Equal } else { return Ordering::Greater } }
        }
    }

    fn box_compare_x(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> Ordering {
        return Self::box_compare(a, b, Axis::X);
    }

    fn box_compare_y(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> Ordering {
        return Self::box_compare(a, b, Axis::Y);
    }

    fn box_compare_z(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> Ordering {
        return Self::box_compare(a, b, Axis::Z);
    }
}

impl Hittable for BVH {
    fn hit(&self, r: crate::math::ray::Ray, t_min: f64, t_max: f64) -> (bool, HitRecord) {
        if !self.bounding_box.hit(r, t_min, t_max) {
            return (false, HitRecord::new(&Material::new(Color::new(0.0, 0.0, 0.0), MaterialType::LAMBERTIAN)));
        }

        let hit_left = self.left.expect("a").hit(r, t_min, t_max);
        let hit_right = self.right.expect("a").hit(r, t_min, t_max);

        if hit_left.0 || hit_right.0 {
            return hit_left; // We don't actually care about the hit record here
        }

        return (false, HitRecord::new(&Material::new(Color::new(0.0, 0.0, 0.0), MaterialType::LAMBERTIAN)));
    }

    fn bounding_box(&self) -> (bool, AABB) {
        return (true, self.bounding_box);
    }
}