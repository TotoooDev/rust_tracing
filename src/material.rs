use crate::math::vec3::*;
use crate::math::ray::*;
use crate::hittable::*;

#[derive(Clone, Copy)]

pub enum MaterialType {
    LAMBERTIAN, METAL, DIELECTRIC
}

#[derive(Clone, Copy)]
pub struct Material {
    pub albedo: Color,
    pub fuzz: f64,
    pub refraction_index: f64,
    mat_type: MaterialType
}

impl Material {
    pub fn new(albedo: Color, mat_type: MaterialType) -> Material {
        return Material { albedo, fuzz: 0.0, refraction_index: 0.0, mat_type };
    }

    pub fn scatter(self, r_in: Ray, rec: HitRecord) -> (bool, Color, Ray) {
        match self.mat_type {
            MaterialType::LAMBERTIAN => return self.scatter_lambertian(r_in, rec),
            MaterialType::METAL => return self.scatter_metal(r_in, rec),
            MaterialType::DIELECTRIC => return self.scatter_dielectric(r_in, rec)
        }
    }

    fn scatter_lambertian(self, r_in: Ray, rec: HitRecord) -> (bool, Color, Ray) {
        let mut scatter_direction = rec.normal + Vec3::random_unit();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction);
        let attenuation = self.albedo;
        return (true, attenuation, scattered);
    }

    fn scatter_metal(self, r_in: Ray, rec: HitRecord) -> (bool, Color, Ray) {
        let reflected = reflect(r_in.dir().normalize(), rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_in_sphere());
        let attenuation = self.albedo;
        return (dot(scattered.dir(), rec.normal) > 0.0, attenuation, scattered);
    }

    fn scatter_dielectric(self, r_in: Ray, rec: HitRecord) -> (bool, Color, Ray) {
        let attenuation = self.albedo;
        let refraction_ratio: f64;
        if rec.front_face() {
            refraction_ratio = 1.0 / self.refraction_index;
        }
        else {
            refraction_ratio = self.refraction_index;
        }

        let unit_direction = r_in.dir().normalize();
        let cos_theta = dot(-unit_direction, rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction: Vec3;

        if cannot_refract || Material::reflectance(cos_theta, refraction_ratio) > rand::random::<f64>() {
            direction = reflect(unit_direction, rec.normal);
        }
        else {
            direction = refract(unit_direction, rec.normal, refraction_ratio);
        }

        let scattered = Ray::new(rec.p, direction);

        return (true, attenuation, scattered);
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
    }
}