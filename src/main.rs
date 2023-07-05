use std::num::NonZeroU32;
use chrono::Utc;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder}, dpi::LogicalSize,
};

mod math;
mod sphere;
mod hittable;
mod hittable_list;
mod camera;
mod material;
mod renderer;

use crate::math::vec3::*;
use crate::renderer::ImageSpecs;
use crate::renderer::Renderer;
use crate::sphere::*;
use crate::hittable_list::*;
use crate::camera::*;

fn main() {
    // IMAGE
    let image_specs = ImageSpecs {
        aspect_ratio: 4.0 / 3.0,
        image_width: 800,
        image_height: 600 as u32,
        samples_per_pixel: 3,
        max_depth: 5
    };
    
    // CAMERA
    let cam = Camera::new(
        Point3::new(13.0, 2.0, 3.0), 
        Vec3::new(0.0, 0.0, 0.0), 
        Vec3::new(0.0, 1.0, 0.0), 
        20.0, 
        image_specs.aspect_ratio);

    // WORLD
    let world = HittableList::<Sphere>::random_scene();
    
    // RENDER
    let mut renderer = Renderer::new(image_specs, cam, world);
    let start = Utc::now();
    let img = renderer.render();
    img.save("result.png").unwrap();

    let finish = Utc::now();
    let time = finish - start;
    println!("Done in {} seconds!", time.num_seconds());

    // WINDOW
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("rust_tracing")
        .with_inner_size(LogicalSize::new(799, 599))
        .build(&event_loop)
        .unwrap();

    let context = unsafe { softbuffer::Context::new(&window) }.unwrap();
    let mut surface = unsafe { softbuffer::Surface::new(&context, &window) }.unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::RedrawRequested(window_id) if window_id == window.id() => {
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
                    let y = index / width;
                    let x = index % width;

                    let r = img.get_pixel(x, y).0[0] as u32;
                    let g = img.get_pixel(x, y).0[1] as u32;
                    let b = img.get_pixel(x, y).0[2] as u32;

                    buffer[index as usize] = b | (g << 8) | (r << 16);
                }
                buffer.present().unwrap();
            }

            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}