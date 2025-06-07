use wgpu::util::DeviceExt;

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
    _padding: f32, // 24 bytes
}

pub struct CameraBuffer {
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
}

impl Camera {
    pub fn new(position: Vec2, zoom: f32, window: &super::Window) -> Self {
        Self {
            window_size: window.size,
            position,
            zoom,
        }
    }

    /// Creates a 'CameraUniform' from the 'Camera'
    pub fn to_uniform(&self) -> CameraUniform {
        CameraUniform {
            window_size: [self.window_size.x, self.window_size.y],
            position: [self.position.x, self.position.y],
            zoom: self.zoom,
            _padding: 0.0,
        }
    }
}

impl CameraUniform {
    /// Creats a `CameraBuffer` from the `CameraUniform`
    pub fn to_buffer(&self, device: &wgpu::Device) -> CameraBuffer {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[*self]),
            usage: wgpu::BufferUsages::UNIFORM,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        CameraBuffer {
            buffer,
            bind_group,
            bind_group_layout,
        }
    }
}
