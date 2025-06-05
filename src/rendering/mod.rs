use log::{error, info, log};

use crate::math::UVec2;

mod camera;

struct Renderer<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface_config: wgpu::SurfaceConfiguration,
    render_pipeline: wgpu::RenderPipeline,
    globals_buffer: wgpu::Buffer,
    globals_bind_group: wgpu::BindGroup,
    window: Window,
}

struct Window {
    sdl_window: sdl3::video::Window,
    sdl_context: sdl3::Sdl,
    size: UVec2,
}

impl Window {
    pub fn new(title: &str, size: UVec2) -> Self {
        let sdl_context = sdl3::init().unwrap();
        let sdl_window = sdl_context
            .video()
            .unwrap()
            .window(title, size.x, size.y)
            .build()
            .unwrap();

        info!("created window with title: {}", title);

        Self {
            sdl_window: sdl_window,
            sdl_context,
            size,
        }
    }
}
