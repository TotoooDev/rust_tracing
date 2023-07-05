use std::vec::Vec;

use crate::sphere::*;
use crate::math::vec3::*;
use crate::material::*;
use crate::hittable::Hittable;
use crate::hittable::HitRecord;

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

    pub fn random_scene() -> HittableList<Sphere> {
        let mut world = HittableList::new();
    
        let ground_material = Material::new(Color::new(0.5, 0.5, 0.5), MaterialType::LAMBERTIAN);
        world.add(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, ground_material));
    
        for a in -11..11 {
            for b in -11..11 {
                let choose_mat = rand::random::<f64>();
                let center = Point3::new(a as f64 + 0.9 * rand::random::<f64>(), 0.2, b as f64 + 0.9 * rand::random::<f64>());
    
                if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                    if choose_mat < 0.8 {
                        let sphere_material = Material::new(Color::random(), MaterialType::LAMBERTIAN);
                        world.add(Sphere::new(center, 0.2, sphere_material));
                    }
                    else if choose_mat < 0.95 {
                        let mut sphere_material = Material::new(Color::random(), MaterialType::METAL);
                        sphere_material.fuzz = rand::random::<f64>();
                        world.add(Sphere::new(center, 0.2, sphere_material));
                    }
                    else {
                        let mut sphere_material = Material::new(Color::random(), MaterialType::DIELECTRIC);
                        sphere_material.refraction_index = 1.5;
                        world.add(Sphere::new(center, 0.2, sphere_material));
                    }
                }
            }
        }
    
        let mut material1 = Material::new(Color::new(1.0, 1.0, 1.0), MaterialType::DIELECTRIC);
        material1.refraction_index = 1.5;
        world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), -1.0, material1));
    
        let material2 = Material::new(Color::new(1.0, 1.0, 1.0), MaterialType::LAMBERTIAN);
        world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2));
        
        let mut material3 = Material::new(Color::new(1.0, 1.0, 1.0), MaterialType::METAL);
        material3.fuzz = 0.0;
        world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3));
    
        return world;
    }
}