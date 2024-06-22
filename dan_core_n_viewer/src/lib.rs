use dan_core_n::behavior::handlers::TopDanmakuBehaviorsHandler;
use dan_core_n::behavior::standard_behaviors::*;
use std::sync::Arc;

use pollster::FutureExt;
use winit::event_loop::ActiveEventLoop;
use winit::window::{WindowAttributes, WindowId};
use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

// Based on https://sotrh.github.io/learn-wgpu

struct TopState<'a> {
    top_handler: TopDanmakuBehaviorsHandler,
    display_state: Option<DisplayState<'a>>,
}

struct DisplayState<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: Arc<Window>,
}

impl<'a> DisplayState<'a> {
    // Creating some of the wgpu types requires async code
    fn new(window: Window) -> DisplayState<'a> {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            #[cfg(target_arch = "wasm32")]
            backends: wgpu::Backends::GL,
            ..Default::default()
        });

        let window_arc = Arc::new(window);

        let surface = instance.create_surface(window_arc.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .block_on()
            .expect("Could not find adapter");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    required_limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    label: None,
                },
                None,
            )
            .block_on()
            .expect("Could not get device");

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        Self {
            surface,
            device,
            queue,
            config,
            size,
            window: window_arc,
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn input(&mut self, _event: &WindowEvent) -> bool {
        false
    }

    fn update(&mut self) {}

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

impl ApplicationHandler<()> for TopState<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.display_state.is_none() {
            let window = event_loop
                .create_window(WindowAttributes::default())
                .expect("Could not create window");

            #[cfg(target_arch = "wasm32")]
            {
                // Winit prevents sizing with CSS, so we have to set
                // the size manually when on web.
                use winit::dpi::PhysicalSize;
                let _ = window.request_inner_size(PhysicalSize::new(450, 400));

                use winit::platform::web::WindowExtWebSys;
                web_sys::window()
                    .and_then(|win| win.document())
                    .and_then(|doc| {
                        let dst = doc.get_element_by_id("wasm-example")?;
                        let canvas = web_sys::Element::from(window.canvas()?);
                        dst.append_child(&canvas).ok()?;
                        Some(())
                    })
                    .expect("Couldn't append canvas to document body.");
            }

            self.display_state = Some(DisplayState::new(window));
        }
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        self.display_state = None;
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if let Some(display_state) = &mut self.display_state {
            if display_state.window.id() == window_id && !display_state.input(&event) {
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                state: ElementState::Pressed,
                                physical_key: PhysicalKey::Code(KeyCode::Escape),
                                ..
                            },
                        ..
                    } => event_loop.exit(),
                    WindowEvent::Resized(physical_size) => display_state.resize(physical_size),
                    WindowEvent::ScaleFactorChanged { .. } => {
                        //inner_size_writer.request_inner_size()
                        //display_state.resize(new_inner_size)
                    }
                    WindowEvent::RedrawRequested => display_state.render().unwrap(),
                    _ => {}
                }
            }
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(display_state) = &mut self.display_state {
            display_state.render().unwrap();
        }
    }

    fn device_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        device_id: DeviceId,
        event: DeviceEvent,
    ) {
        let _ = (event_loop, device_id, event);
    }

    fn memory_warning(&mut self, _event_loop: &ActiveEventLoop) {
        self.top_handler.cleanup();
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn run() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }

    let mut top_handler = TopDanmakuBehaviorsHandler::new();
    top_handler.register_behavior(motion1_behavior());
    top_handler.register_behavior(gravity1_behavior());
    top_handler.register_behavior(acceleration1_behavior());

    top_handler.register_behavior(rotate_orientation_behavior());
    top_handler.register_behavior(rotate_forward_behavior());

    top_handler.register_behavior(motion3_behavior());
    top_handler.register_behavior(gravity3_behavior());
    top_handler.register_behavior(acceleration3_behavior());

    top_handler.register_behavior(mandatory_end());

    let event_loop = EventLoop::new().unwrap();
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            #[cfg(target_arch = "wasm32")]
            use winit::platform::web::EventLoopExtWebSys;
            event_loop.spawn_app(
                TopState {
                    display_state: None,
                    top_handler,
                }
            );
        } else {
            let mut state = TopState {
                display_state: None,
                top_handler,
            };
            let _ = event_loop.run_app(&mut state);
        }
    }
}
