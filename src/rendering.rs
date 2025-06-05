use crate::math::{UVec2, Vec2};

struct Renderer<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface_config: wgpu::SurfaceConfiguration,
    render_pipeline: wgpu::RenderPipeline,
    globals_buffer: wgpu::Buffer,
    globals_bind_group: wgpu::BindGroup,
    window: Window<'a>,
}

struct Window<'a> {
    sdl_window: &'a sdl3::video::Window,
    sdl_context: sdl3::Sdl,
    size: UVec2,
}

struct Camera {
    window_size: UVec2,
    position: Vec2,
    zoom: f32,
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
struct CameraUniform {
    window_size: [u32; 2],
    position: [f32; 2],
    zoom: f32,
}

struct CameraBuffer {
    buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
}
