use sdl3::{event::Event, keyboard::Keycode};
use wgpu::{
    BackendOptions, InstanceFlags,
    rwh::{HasDisplayHandle, HasWindowHandle},
};

pub fn run() {
    env_logger::init();

    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("wgpu + sdl3", 800, 800)
        .build()
        .unwrap();

    let window_handle = window.window_handle().unwrap();
    let display_handle = window.display_handle().unwrap();

    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        flags: InstanceFlags::all(),
        backend_options: BackendOptions::default(),
    });

    let surface =
        create_surface::create_surface(&instance, &window).expect("failed to create surface");

    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptionsBase {
        power_preference: wgpu::PowerPreference::HighPerformance,
        force_fallback_adapter: false,
        compatible_surface: Some(&surface),
    }))
    .expect("failed to find adapter");

    let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
        label: None,
        required_features: wgpu::Features::empty(),
        required_limits: wgpu::Limits::default(),
        memory_hints: wgpu::MemoryHints::Performance,
        trace: wgpu::Trace::Off,
    }))
    .unwrap();

    let surface_config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface.get_capabilities(&adapter).formats[0],
        width: window.size().0,
        height: window.size().1,
        present_mode: wgpu::PresentMode::Fifo,
        desired_maximum_frame_latency: 2,
        alpha_mode: wgpu::CompositeAlphaMode::Auto,
        view_formats: vec![],
    };

    surface.configure(&device, &surface_config);

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
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

        let frame = match surface.get_current_texture() {
            Ok(frame) => frame,
            Err(_) => {
                surface.configure(&device, &surface_config);
                surface
                    .get_current_texture()
                    .expect("failed to aqquire frame even after reconfigure")
            }
        };

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("render pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.0,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        queue.submit(Some(encoder.finish()));
        frame.present();
    }
}

// taken from the example in sdl3 repo for wgpu struff, i dont want to write this myself :(
mod create_surface {
    use sdl3::video::Window;
    use wgpu::rwh::{HasDisplayHandle, HasWindowHandle};

    // contains the unsafe impl as much as possible by putting it in this module
    struct SyncWindow<'a>(&'a Window);

    unsafe impl<'a> Send for SyncWindow<'a> {}
    unsafe impl<'a> Sync for SyncWindow<'a> {}

    impl<'a> HasWindowHandle for SyncWindow<'a> {
        fn window_handle(&self) -> Result<wgpu::rwh::WindowHandle<'_>, wgpu::rwh::HandleError> {
            self.0.window_handle()
        }
    }
    impl<'a> HasDisplayHandle for SyncWindow<'a> {
        fn display_handle(&self) -> Result<wgpu::rwh::DisplayHandle<'_>, wgpu::rwh::HandleError> {
            self.0.display_handle()
        }
    }

    pub fn create_surface<'a>(
        instance: &wgpu::Instance,
        window: &'a Window,
    ) -> Result<wgpu::Surface<'a>, String> {
        instance
            .create_surface(SyncWindow(window))
            .map_err(|err| err.to_string())
    }
}
