use crate::math::deg_to_rad;
use crate::math::vec3::*;
use crate::math::ray::*;

#[derive(Clone, Copy)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    pub fn new(lookfrom: Point3, lookat: Point3, vup: Vec3, vfov: f64, aspect_ratio: f64) -> Camera {
        let theta = deg_to_rad(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).normalize();
        let u = cross(vup, w).normalize();
        let v = cross(w, u);

        let origin = lookfrom;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

        return Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical
        };
    }

    pub fn get_ray(self, s: f64, t: f64) -> Ray {
        return Ray::new(
            self.origin,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin
        );
    }
}