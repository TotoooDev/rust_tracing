use image::{ImageBuffer, RgbImage, imageops};
use rand::random;

use crate::camera::*;
use crate::hittable_list::*;
use crate::sphere::*;
use crate::math;
use crate::math::vec3::*;
use crate::math::ray::*;

pub struct ImageSpecs {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub image_height: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32
}

pub struct Renderer {
    image_specs: ImageSpecs,
    cam: Camera,
    world: HittableList<Sphere>
}

impl Renderer {
    pub fn new(image_specs: ImageSpecs, cam: Camera, world: HittableList<Sphere>) -> Renderer {
        return Renderer {
            image_specs,
            cam,
            world
        };
    }

    pub fn render(&mut self) -> RgbImage {
        let mut img: RgbImage = ImageBuffer::new(self.image_specs.image_width, self.image_specs.image_height);

        for j in 0..self.image_specs.image_height {
            println!("{} scanlines remaining...", self.image_specs.image_height - j);
            
            for i in 0..self.image_specs.image_width {
                let mut color = Color::new(0.0, 0.0, 0.0);
                for _k in 0..self.image_specs.samples_per_pixel {
                    let u = (i as f64 + random::<f64>()) / (self.image_specs.image_width - 1) as f64;
                    let v = (j as f64 + random::<f64>()) / (self.image_specs.image_height - 1) as f64;
                    let ray = self.cam.get_ray(u, v);
                    color += Renderer::ray_color(ray, &mut self.world, self.image_specs.max_depth);
    
                }
                Renderer::put_pixel_float(&mut img, color, i, j, self.image_specs.samples_per_pixel);
            }
        }
    
        img = imageops::rotate180(&img);
        img = imageops::flip_horizontal(&img);

        return img;
    }

    fn put_pixel_float(img: &mut RgbImage, color: Color, x: u32, y: u32, samples_per_pixel: u32) {
        let mut r = color.x;
        let mut g = color.y;
        let mut b = color.z;
    
        let scale = 1.0 / samples_per_pixel as f64;
        r = (r * scale).sqrt();
        g = (g * scale).sqrt();
        b = (b * scale).sqrt();
    
        img.put_pixel(x, y, image::Rgb([
            (255.0 * math::clamp(r, 0.0, 1.0)) as u8, 
            (255.0 * math::clamp(g, 0.0, 1.0)) as u8, 
            (255.0 * math::clamp(b, 0.0, 1.0)) as u8]
        ))
    }

    fn ray_color(r: Ray, world: &mut HittableList<Sphere>, depth: u32) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
    
        let (hit, hit_record) = world.hit(r, 0.001, f64::INFINITY);
        if hit {
            let (scatter_hit, attenuation, scattered) = hit_record.mat.scatter(r, hit_record);
            if scatter_hit {
                return attenuation * Renderer::ray_color(scattered, world, depth - 1);
            }
            return Color::new(0.0, 0.0, 0.0);
        }
        let unit_direction = r.dir().normalize();
        let t = 0.5 * unit_direction.y + 1.0;
        return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
    }
}