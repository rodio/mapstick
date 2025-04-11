mod geometry;

use geometry::{Command, Geometry, Operation};
use prost::Message;
use std::{io::Read, num::NonZeroUsize, sync::Arc};

// use env_logger;
use vello::{
    Renderer, RendererOptions, Scene,
    kurbo::{Affine, Point, Stroke},
    peniko::{self, Color, color::AlphaColor},
    util::{RenderContext, RenderSurface},
};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::{self, EventLoop},
    keyboard::NamedKey,
    window::{Window, WindowAttributes},
};

enum AppState<'app> {
    Active {
        surface: RenderSurface<'app>,
        window: Arc<Window>,
    },
    Suspended(Option<Arc<Window>>),
}

struct App<'app> {
    app_state: AppState<'app>,
    context: RenderContext,
    renderers: Vec<Option<Renderer>>,
    scene: Scene,
}

const WIDTH: u32 = 2000;
const HEIGHT: u32 = 2000;

impl<'app> App<'app> {
    fn new() -> App<'app> {
        Self {
            app_state: AppState::Suspended(None),
            context: RenderContext::new(),
            renderers: vec![],
            scene: Scene::new(),
        }
    }
}

impl<'app> ApplicationHandler for App<'app> {
    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let AppState::Active { surface, window } = &mut self.app_state else {
            return;
        };
        if window.id() != window_id {
            return;
        }

        match event {
            WindowEvent::CloseRequested => {
                log::info!("close requested");
                event_loop.exit()
            }
            WindowEvent::Resized(size) => {
                self.context
                    .resize_surface(surface, size.width, size.height);
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state.is_pressed() {
                    if event.logical_key == NamedKey::Escape {
                        log::info!("exiting on ESC");
                        event_loop.exit();
                    }
                }
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                log::info!("scale factor changed");
                log::info!("{:#?}", scale_factor);
            }
            WindowEvent::RedrawRequested => {
                log::trace!("redraw requested");
                self.scene.reset();

                let shape_col = Color::new([0.0, 0.702, 0.9294, 1.]);
                for i in 0..3 {
                    let geometry = get_geometry(0, i);
                    let path = create_path(geometry);
                    // self.scene
                    //     .stroke(&Stroke::new(6.0), Affine::IDENTITY, shape_col, None, &path);
                    self.scene.fill(
                        peniko::Fill::NonZero,
                        Affine::IDENTITY,
                        shape_col,
                        None,
                        &path,
                    );
                }
                let shape_col = Color::new([0.900, 0.802, 0.9294, 1.]);
                for i in 0..128 {
                    let geometry = get_geometry(8, i);
                    let path = create_path(geometry);
                    self.scene
                        .stroke(&Stroke::new(6.0), Affine::IDENTITY, shape_col, None, &path);
                    // self.scene.fill(
                    //     peniko::Fill::NonZero,
                    //     Affine::IDENTITY,
                    //     shape_col,
                    //     None,
                    //     &path,
                    // );
                }

                let device_handle = &self.context.devices[surface.dev_id];

                let width = surface.config.width;
                let height = surface.config.height;

                let params = &vello::RenderParams {
                    base_color: AlphaColor::from_rgba8(0, 100, 0, 1), // Background color
                    width,
                    height,
                    antialiasing_method: vello::AaConfig::Msaa16,
                };

                let texture = surface.surface.get_current_texture().unwrap();

                self.renderers[surface.dev_id]
                    .as_mut()
                    .unwrap()
                    .render_to_surface(
                        &device_handle.device,
                        &device_handle.queue,
                        &self.scene,
                        &texture,
                        params,
                    )
                    .unwrap();

                // IDK:
                texture.present();
                // IDK:
                device_handle.device.poll(vello::wgpu::MaintainBase::Poll);
            }
            _ => (),
        }
    }

    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        log::info!("resumed");
        let AppState::Suspended(ref mut cached_window) = self.app_state else {
            return;
        };

        let window_attributes = WindowAttributes::default()
            .with_inner_size(LogicalSize::new(WIDTH, HEIGHT))
            .with_title("Mapstick");

        let window = cached_window
            .take()
            .or_else(|| match event_loop.create_window(window_attributes) {
                Ok(w) => Some(Arc::new(w)),
                Err(e) => panic!("{}", e),
            })
            .unwrap();

        let surface_future = self.context.create_surface(
            Arc::clone(&window),
            window.inner_size().width,
            window.inner_size().height,
            vello::wgpu::PresentMode::AutoVsync,
        );
        let surface = pollster::block_on(surface_future).unwrap();
        self.renderers
            .resize_with(self.context.devices.len(), || None);

        if self.renderers[surface.dev_id].is_none() {
            self.renderers[surface.dev_id] = Some(
                Renderer::new(
                    &self.context.devices[surface.dev_id].device,
                    RendererOptions {
                        use_cpu: false,
                        antialiasing_support: vello::AaSupport::all(),
                        num_init_threads: NonZeroUsize::new(1),
                        surface_format: Some(surface.format),
                    },
                )
                .unwrap(),
            );
        }

        self.app_state = AppState::Active { surface, window }
    }

    fn suspended(&mut self, event_loop: &event_loop::ActiveEventLoop) {
        log::info!("suspended");
        if let AppState::Active { window, .. } = &self.app_state {
            self.app_state = AppState::Suspended(Some(Arc::clone(window)));
        }
    }
}

fn create_path(geometry: Geometry) -> vello::kurbo::BezPath {
    let mut path = peniko::kurbo::BezPath::new();
    let mut px = 0.0;
    let mut py = 0.0;
    for operation in geometry.operations {
        match operation {
            Operation {
                command: Command::MoveTo,
                params,
            } => {
                let cx = px + params.get(0).unwrap().raw_value as f64;
                let cy = py + params.get(1).unwrap().raw_value as f64;
                path.move_to(Point::new(cx, cy));
                px = cx;
                py = cy;
            }
            Operation {
                command: Command::LineTo,
                params,
            } => {
                let cx = px + params.get(0).unwrap().raw_value as f64;
                let cy = py + params.get(1).unwrap().raw_value as f64;
                path.line_to(Point::new(cx, cy));
                px = cx;
                py = cy;
            }
            Operation {
                command: Command::ClosePath,
                ..
            } => {
                path.close_path();
            }
        }
    }

    path
}

fn main() {
    env_logger::builder()
        .format_timestamp(Some(env_logger::TimestampPrecision::Millis))
        .filter_level(log::LevelFilter::Info)
        .init();

    let event_loop = EventLoop::new().unwrap();
    let mut app = App::new();
    let _ = event_loop.run_app(&mut app);
}

include!(concat!(env!("OUT_DIR"), "/vector_tile.rs"));

pub fn get_geometry(layer: usize, feature: usize) -> Geometry {
    let mut file = std::fs::File::open("tile1.mvt").unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    let tile = Tile::decode(buf.as_slice()).unwrap();
    let geometry_vec = &tile
        .layers
        .get(layer)
        .unwrap()
        .features
        .get(feature)
        .unwrap()
        .geometry;
    let geometry: Geometry = Geometry::try_from(geometry_vec).unwrap();
    // println!("{:#?}", &geometry);
    // for (i, layer) in tile.layers.into_iter().enumerate() {
    //     std::fs::write(format!("layer{i}.txt"), format!("{:#?}", layer)).unwrap();
    // }
    geometry
}
