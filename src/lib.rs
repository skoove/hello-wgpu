use sdl3::{event::Event, keyboard::Keycode, mouse::MouseButton};
use wgpu::{
    BackendOptions, InstanceFlags,
    rwh::{HasDisplayHandle, HasWindowHandle},
};

struct State<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface_config: wgpu::SurfaceConfiguration,
    window_size: (u32, u32),
    window: &'a sdl3::video::Window,
    sdl_context: sdl3::Sdl,

    clear_color: wgpu::Color,
}

impl<'a> State<'a> {
    fn new(window: &'a sdl3::video::Window, sdl_context: sdl3::Sdl) -> Self {
        // let window_handle = window.window_handle().unwrap();
        // let display_handle = window.display_handle().unwrap();

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            flags: InstanceFlags::all(),
            backend_options: BackendOptions::default(),
        });

        let surface =
            create_surface::create_surface(&instance, &window).expect("failed to create surface");

        let adapter =
            pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptionsBase {
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

        State {
            surface,
            device,
            queue,
            surface_config,
            window_size: window.size(),
            window: &window,
            sdl_context,
            clear_color: wgpu::Color::BLACK,
        }
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = match self.surface.get_current_texture() {
            Ok(output) => output,
            Err(_) => {
                self.surface.configure(&self.device, &self.surface_config);
                self.surface.get_current_texture()?
            }
        };

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("clear screen pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(self.clear_color),
                    store: wgpu::StoreOp::Store,
                },
            })],
            ..Default::default()
        });

        self.queue.submit(Some(encoder.finish()));
        output.present();

        Ok(())
    }
}

pub fn run() {
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("wgpu + sdl3", 800, 800)
        .build()
        .unwrap();

    let mut state = State::new(&window, sdl_context);

    let mut event_pump = state.sdl_context.event_pump().unwrap();
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

        state.render().unwrap();
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
