use chrono::Utc;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Window},
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

fn main() -> std::io::Result<()> {

    
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });


    // IMAGE
    let image_specs = ImageSpecs {
        aspect_ratio: 16.0 / 9.0,
        image_width: 640,
        image_height: (640 as f64 / (16.0 / 9.0)) as u32,
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

    return Ok(());
}