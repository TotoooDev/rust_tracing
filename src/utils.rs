use tobj;
use crate::hittable::triangle::Triangle;
use crate::material::{Material, MaterialType};
use crate::math::vec3::{Color, Point3};

pub fn triangles_from_obj(path: String) -> Vec<Triangle> {
    let (models, materials_unsafe) = tobj::load_obj(path, &tobj::LoadOptions::default()).unwrap();
    let materials = materials_unsafe.unwrap();

    let mut triangles = Vec::<Triangle>::new();

    for (i, m) in models.iter().enumerate() {
        let mesh = &m.mesh;
        for index in 0..mesh.indices.len() / 3 {
            let idx0 = mesh.indices[3 * index] as usize;
            let idx1 = mesh.indices[3 * index + 1] as usize;
            let idx2 = mesh.indices[3 * index + 2] as usize;

            let v0 = Point3::new(mesh.positions[3 * idx0] as f64, mesh.positions[3 * idx0 + 1] as f64, mesh.positions[3 * idx0 + 2] as f64);
            let v1 = Point3::new(mesh.positions[3 * idx1] as f64, mesh.positions[3 * idx1 + 1] as f64, mesh.positions[3 * idx1 + 2] as f64);
            let v2 = Point3::new(mesh.positions[3 * idx2] as f64, mesh.positions[3 * idx2 + 1] as f64, mesh.positions[3 * idx2 + 2] as f64);

            let triangle = Triangle::new(
                v0,
                v1,
                v2,
                Material::new(Color::new(1.0, 0.0, 0.0), MaterialType::LAMBERTIAN)
            );

            triangles.push(triangle);
        }
    }

    return triangles;
}