use std::vec::Vec;

use crate::hittable::sphere::*;
use crate::math::vec3::*;
use crate::math::ray::*;
use crate::material::*;
use crate::hittable::*;
use crate::aabb::*;

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> HittableList {
        return HittableList { objects: Vec::new() };
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn random_scene() -> HittableList {
        let mut world = HittableList::new();
    
        let ground_material = Material::new(Color::new(0.5, 0.5, 0.5), MaterialType::LAMBERTIAN);
        world.add(Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));
    
        for a in -11..11 {
            for b in -11..11 {
                let choose_mat = rand::random::<f64>();
                let center = Point3::new(a as f64 + 0.9 * rand::random::<f64>(), 0.2, b as f64 + 0.9 * rand::random::<f64>());
    
                if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                    if choose_mat < 0.8 {
                        let sphere_material = Material::new(Color::random(), MaterialType::LAMBERTIAN);
                        world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                    }
                    else if choose_mat < 0.95 {
                        let mut sphere_material = Material::new(Color::random(), MaterialType::METAL);
                        sphere_material.fuzz = rand::random::<f64>();
                        world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                    }
                    else {
                        let mut sphere_material = Material::new(Color::random(), MaterialType::DIELECTRIC);
                        sphere_material.refraction_index = 1.5;
                        world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                    }
                }
            }
        }
    
        let mut material1 = Material::new(Color::new(1.0, 1.0, 1.0), MaterialType::DIELECTRIC);
        material1.refraction_index = 1.5;
        world.add(Box::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), -1.0, material1)));
    
        let material2 = Material::new(Color::new(1.0, 1.0, 1.0), MaterialType::LAMBERTIAN);
        world.add(Box::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2)));
        
        let mut material3 = Material::new(Color::new(1.0, 1.0, 1.0), MaterialType::METAL);
        material3.fuzz = 0.0;
        world.add(Box::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3)));
    
        return world;
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> (bool, HitRecord) {
        let (_, mut hit_rec) = self.objects[0].hit(r, t_min, t_max);
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            let (hit, rec) = object.hit(r, t_min, closest_so_far);
            if hit {
                hit_anything = true;
                closest_so_far = rec.t;
                hit_rec = rec;
            }
        }

        return (hit_anything, hit_rec);
    }

    fn bounding_box(&self) -> (bool, AABB) {
        if self.objects.is_empty() {
            return (false, AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0)));
        }

        let mut output_box = AABB::new(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 0.0));

        let mut first_box = true;

        for object in &self.objects {
            let (hit, aabb) = object.bounding_box();
            if !hit {
                return (hit, aabb);
            }

            if first_box {
                output_box = aabb;
            }
            else {
                output_box = surrounding_box(output_box, aabb);
                first_box = false;
            }
        }

        return (true, output_box);
    }
}