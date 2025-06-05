use crate::math::{UVec2, Vec2};

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
