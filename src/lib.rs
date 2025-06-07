use sdl3::{
    event::{Event, WindowEvent},
    keyboard::Keycode,
};

use crate::{
    math::{UVec2, Vec2},
    rendering::camera::Camera,
    world::World,
};

mod math;
mod rendering;
mod world;

pub fn run() {
    let mut window = rendering::Window::new("hi sdl3 + wgpu", UVec2::new(500, 500));

    let world = World {
        camera: Camera::new(Vec2::new(1.0, 0.0), 0.1, &window),
    };

    let mut event_pump = window.sdl_context.event_pump().unwrap();

    let renderer = rendering::Renderer::new(&mut window, &world);

    'running: loop {
        renderer.render(&world).unwrap();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
    }
}
