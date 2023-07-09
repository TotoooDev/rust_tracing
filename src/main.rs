use std::num::NonZeroU32;
use image::imageops;
use material::Material;
use winit::{
    event::{Event, WindowEvent, ElementState},
    event_loop::EventLoop,
    window::WindowBuilder, dpi::LogicalSize, platform::modifier_supplement::KeyEventExtModifierSupplement,
    keyboard::Key,
};


mod math;
mod hittable;
mod hittable_list;
mod camera;
mod material;
mod renderer;
mod aabb;
mod bvh;

use crate::math::vec3::*;
use crate::renderer::ImageSpecs;
use crate::renderer::Renderer;
use crate::hittable::sphere::*;
use crate::hittable::model::*;
use crate::hittable_list::*;
use crate::camera::*;

fn main() {
    // IMAGE
    let image_specs = ImageSpecs {
        aspect_ratio: 4.0 / 3.0,
        image_width: 800,
        image_height: 600,
        samples_per_pixel: 1,
        max_depth: 3
    };
    
    // CAMERA
    let cam = Camera::new(
        Point3::new(0.0, 20.0, 130.0), 
        Vec3::new(0.0, 10.0, 0.0), 
        Vec3::new(0.0, 1.0, 0.0), 
        60.0, 
        image_specs.aspect_ratio
    );

    // WORLD
    let mut world = HittableList::new();

    let mut model_mat = Material::new(Color::new(0.84, 0.07, 0.08), material::MaterialType::DIELECTRIC);
    model_mat.refraction_index = 1.5;

    let mut big_sphere_mat = Material::new(Color::new(0.56, 0.21, 0.8), material::MaterialType::METAL);
    big_sphere_mat.fuzz = 0.03;

    let mut smol_sphere_mat = Material::new(Color::new(0.2, 0.07, 0.28), material::MaterialType::METAL);
    smol_sphere_mat.fuzz = 0.0;

    world.add(Box::new(Model::new("love.obj".to_string(), Vec3::new(0.0, 0.0, 0.0), model_mat)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, big_sphere_mat)));
    world.add(Box::new(Sphere::new(Point3::new(30.0, 8.0, 60.0), 10.0, Material::new(Color::new(0.21, 0.8, 0.4), material::MaterialType::LAMBERTIAN))));
    world.add(Box::new(Sphere::new(Point3::new(-80.0, 40.0, -55.0), 40.0, smol_sphere_mat)));

    // RENDER
    let mut renderer = Renderer::new(image_specs, cam, world);
    let mut img: image::RgbImage = image::ImageBuffer::new(image_specs.image_width, image_specs.image_height);
    let mut scanline_index: u32 = image_specs.image_height - 1;

    // WINDOW
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("rust_tracing")
        .with_inner_size(LogicalSize::new(image_specs.image_width - 1, image_specs.image_height - 1))
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();

    let context = unsafe { softbuffer::Context::new(&window) }.unwrap();
    let mut surface = unsafe { softbuffer::Surface::new(&context, &window) }.unwrap();

    // MAIN LOOP
    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();
        window.request_redraw();

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => control_flow.set_exit(),

                WindowEvent::KeyboardInput { event, .. } => {
                    if event.state == ElementState::Pressed {
                        match event.key_without_modifiers().as_ref() {
                            Key::Character("s") => {
                                let img_save = imageops::flip_horizontal(&imageops::rotate180(&img));
                                img_save.save("result.png").unwrap();
                            },
                            _ => ()
                        }
                    }
                },

                _ => ()
            },

            Event::RedrawRequested(_) => {
                if scanline_index > 0 {
                    renderer.render_scanline(&mut img, scanline_index);
                    scanline_index -= 1;
                }

                let (width, height) = {
                    let size = window.inner_size();
                    (size.width, size.height)
                };
                surface
                    .resize(
                        NonZeroU32::new(width).unwrap(),
                        NonZeroU32::new(height).unwrap(),
                    )
                    .unwrap();

                let mut buffer = surface.buffer_mut().unwrap();
                for index in 0..(width * height) {
                    let y = image_specs.image_height - 1 - index / width;
                    let x = index % width;

                    let pixel = img.get_pixel_mut(x, y);
                    let image::Rgb(data) = *pixel;

                    let r = data[0] as u32;
                    let g = data[1] as u32;
                    let b = data[2] as u32;

                    buffer[index as usize] = b | (g << 8) | (r << 16);
                }
                buffer.present().unwrap();
            },

            _ => ()
        }
    });
}