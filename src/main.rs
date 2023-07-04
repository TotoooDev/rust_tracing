use chrono::Utc;
use image::{ImageBuffer, RgbImage, imageops};
use rand::random;

mod math;
mod sphere;
mod hittable;
mod hittable_list;
mod camera;
mod material;

use crate::material::*;
use crate::math::vec3::*;
use crate::math::ray::*;

use crate::sphere::*;

use crate::hittable_list::*;

use crate::camera::*;

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
            return attenuation * ray_color(scattered, world, depth - 1);
        }
        return Color::new(0.0, 0.0, 0.0);
    }
    let unit_direction = r.dir().normalize();
    let t = 0.5 * unit_direction.y + 1.0;
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}

fn random_scene() -> HittableList<Sphere> {
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

fn main() -> std::io::Result<()> {
    // IMAGE
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH : u32 = 1280;
    const IMAGE_HEIGHT : u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 500;
    const MAX_DEPTH: u32 = 50;
    
    let mut img: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    img.put_pixel(100, 100, image::Rgb([255, 0, 255]));

    // CAMERA
    let cam = Camera::new(
        Point3::new(13.0, 2.0, 3.0), 
        Vec3::new(0.0, 0.0, 0.0), 
        Vec3::new(0.0, 1.0, 0.0), 
        20.0, 
        ASPECT_RATIO);

    // WORLD
    let mut world = random_scene();
    
    // RENDER
    let start = Utc::now();
    for j in 0..IMAGE_HEIGHT {
        println!("{} scanlines remaining...", IMAGE_HEIGHT - j);
        
        for i in 0..IMAGE_WIDTH {
            let mut color = Color::new(0.0, 0.0, 0.0);
            for _k in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + random::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
                let ray = cam.get_ray(u, v);
                color += ray_color(ray, &mut world, MAX_DEPTH);

            }
            put_pixel_float(&mut img, color, i, j, SAMPLES_PER_PIXEL);
        }
    }

    img = imageops::rotate180(&img);
    img = imageops::flip_horizontal(&img);
    println!("Writing...");
    img.save("result.png").unwrap();

    let finish = Utc::now();
    let time = finish - start;
    println!("Done in {} seconds!", time.num_seconds());

    return Ok(());
}