use crate::math::{UVec2, Vec2};

pub struct Camera {
    window_size: UVec2,
    position: Vec2,
    zoom: f32,
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
pub struct CameraUniform {
    window_size: [u32; 2],
    position: [f32; 2],
    zoom: f32,
}

pub struct CameraBuffer {
    buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
}

impl Camera {
    pub fn new(position: Vec2, zoom: f32, window: &super::Window) -> Self {
        Self {
            window_size: window.size,
            position,
            zoom,
        }
    }

    /// Turns the `Camera` into a `CameraUniform`
    pub fn to_uniform(&self) -> CameraUniform {
        CameraUniform {
            window_size: [self.window_size.x, self.window_size.y],
            position: [self.position.x, self.position.y],
            zoom: self.zoom,
        }
    }
}
