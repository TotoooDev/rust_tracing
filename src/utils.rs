use tobj;
use crate::hittable::triangle::Triangle;
use crate::hittable_list::HittableList;
use crate::material::{Material, MaterialType};
use crate::math::vec3::{Color, Point3};

pub fn triangles_from_obj(path: &String) -> HittableList<Triangle> {
    let mut world = HittableList::<Triangle>::new();

    let (models, materials_unsafe) = tobj::load_obj(path, &tobj::GPU_LOAD_OPTIONS).unwrap();
    let materials = materials_unsafe.unwrap();

    let mut triangles = Vec::<Triangle>::new();

    for (i, m) in models.iter().enumerate() {
        let mesh = &m.mesh;

        for v in 0..mesh.positions.len() / 9 {
            let v0 = Point3::new(mesh.positions[3 * v] as f64, mesh.positions[3 * v + 1] as f64, mesh.positions[3 * v + 2] as f64);
            let v1 = Point3::new(mesh.positions[3 * v + 3] as f64, mesh.positions[3 * v + 4] as f64, mesh.positions[3 * v + 5] as f64);
            let v2 = Point3::new(mesh.positions[3 * v + 6] as f64, mesh.positions[3 * v + 7] as f64, mesh.positions[3 * v + 8] as f64);
            
            world.add(Triangle::new(v0, v1, v2, Material::new(Color::random(), MaterialType::LAMBERTIAN)));
        }
    }

    return world;
}