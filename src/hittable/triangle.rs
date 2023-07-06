use crate::math::vec3::*;
use crate::hittable::*;

pub struct Triangle {
    v0: Point3,
    v1: Point3,
    v2: Point3,
    mat: Material
}

impl Triangle {
    pub fn new(v0: Point3, v1: Point3, v2: Point3, mat: Material) -> Triangle {
        return Triangle {
            v0,
            v1,
            v2,
            mat
        };
    }
}

impl Hittable for Triangle {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> (bool, HitRecord) {
        // From https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-rendering-a-triangle/ray-triangle-intersection-geometric-solution.html
        // I was too lazy to do the maths by myself

        // Compute the plane's normal
        let v0v1 = self.v1 - self.v0;
        let v0v2 = self.v2 - self.v0;
        // No need to normalize
        let n = cross(v0v1, v0v2); // Unsure of the operation here
        let area = n.length();

        // Finding P (the point of intersection)
        // Check if the ray and plane are parallel
        let n_dot_ray_direction = dot(n, r.dir());
        if n_dot_ray_direction.abs() < 0.00001 {
            return (false, HitRecord::new(&self.mat));
        }

        // Compute the triangle's normal (I think) (I'm stupid)
        let d = -dot(n, self.v0);
        // Compute t (I've given up trying to understand what it means)
        let t = -(dot(n, r.origin()) + d) / n_dot_ray_direction;
        // Check if the triangle is behind the ray
        if t < 0.0 {
            return (false, HitRecord::new(&self.mat));
        }

        // Compute the intersection point
        let p = r.origin() + t * r.dir();

        // Inside-outside test
        let mut c: Vec3;
        // Edge 0
        let edge0 = self.v1 - self.v0;
        let vp0 = p - self.v0;
        c = cross(edge0, vp0);
        if dot(n, c) < 0.0 {
            return (false, HitRecord::new(&self.mat));
        }
        // Edge 1
        let edge1 = self.v2 - self.v1;
        let vp1 = p - self.v1;
        c = cross(edge1, vp1);
        if dot(n, c) < 0.0 {
            return (false, HitRecord::new(&self.mat));
        }
        // Edge 2
        let edge2 = self.v0 - self.v2;
        let vp2 = p - self.v2;
        c = cross(edge2, vp2);
        if dot(n, c) < 0.0 {
            return (false, HitRecord::new(&self.mat));
        }

        // Yay
        return (true, HitRecord::new(&self.mat));
    }
}