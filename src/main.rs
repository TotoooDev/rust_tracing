use std::num::NonZeroU32;
use std::thread;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder, dpi::LogicalSize,
};

mod math;
mod hittable;
mod hittable_list;
mod camera;
mod material;
mod renderer;

use crate::math::vec3::*;
use crate::renderer::ImageSpecs;
use crate::renderer::Renderer;
use crate::hittable_list::*;
use crate::camera::*;

fn main() {
    // IMAGE
    let image_specs = ImageSpecs {
        aspect_ratio: 4.0 / 3.0,
        image_width: 800,
        image_height: 600 as u32,
        samples_per_pixel: 10,
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
    let mut img: image::RgbImage = image::ImageBuffer::new(800, 600);
    let mut scanline_index: u32 = image_specs.image_height - 1;

    // WINDOW
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("rust_tracing")
        .with_inner_size(LogicalSize::new(799, 599))
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