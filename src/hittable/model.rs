use crate::math::vec3::*;
use crate::hittable::*;

use super::triangle::Triangle;

pub struct Model {
    pub pos: Vec3,
    triangles: Vec<Triangle>
}

impl Model {
    pub fn new(path: String, pos: Vec3, mat: Material) -> Model {
        let (models, _materials_unsafe) = tobj::load_obj(path, &tobj::LoadOptions::default()).unwrap();

        let mut triangles = Vec::<Triangle>::new();

        for (_i, m) in models.iter().enumerate() {
            let mesh = &m.mesh;
            for index in 0..mesh.indices.len() / 3 {
                let idx0 = mesh.indices[3 * index] as usize;
                let idx1 = mesh.indices[3 * index + 1] as usize;
                let idx2 = mesh.indices[3 * index + 2] as usize;

                let v0 = Point3::new(mesh.positions[3 * idx0] as f64, mesh.positions[3 * idx0 + 1] as f64, mesh.positions[3 * idx0 + 2] as f64) + pos;
                let v1 = Point3::new(mesh.positions[3 * idx1] as f64, mesh.positions[3 * idx1 + 1] as f64, mesh.positions[3 * idx1 + 2] as f64) + pos;
                let v2 = Point3::new(mesh.positions[3 * idx2] as f64, mesh.positions[3 * idx2 + 1] as f64, mesh.positions[3 * idx2 + 2] as f64) + pos;

                let triangle = Triangle::new(
                    v0,
                    v1,
                    v2,
                    mat
                );

                triangles.push(triangle);
            }
        }

        return Model { pos, triangles };
    }
}

impl Hittable for Model {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> (bool, HitRecord) {
        // Registered hit record
        let (mut hit, mut hit_record): (bool, HitRecord) = (false, HitRecord::new(&self.triangles[0].mat));
        hit_record.t = f64::INFINITY;

        for triangle in &self.triangles {
            let (temp_hit, temp_hit_record) = triangle.hit(r, t_min, t_max);

            // Continue if the ray didn't hit
            if !temp_hit {
                continue;
            }

            // If the current triangle is closer than the registered one, register it instead
            if temp_hit_record.t < hit_record.t {
                hit = temp_hit;
                hit_record = temp_hit_record;
            }
        }

        return (hit, hit_record);
    }

    fn bounding_box(&self) -> (bool, AABB) {
        return (false, AABB::new(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 0.0)));
    }
}