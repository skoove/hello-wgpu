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
    window: Window<'a>,
}

struct Window<'a> {
    sdl_window: &'a sdl3::video::Window,
    sdl_context: sdl3::Sdl,
    size: UVec2,
}
