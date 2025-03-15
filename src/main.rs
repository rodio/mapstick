use std::sync::Arc;

use pixels::{Pixels, PixelsBuilder, SurfaceTexture};
use tiny_skia::{Color, FillRule, Paint, PathBuilder, Pixmap, Stroke, Transform};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

struct App<'win> {
    window: Arc<Window>,
    pixels: Pixels<'win>,
}
const WIDTH: u32 = 640;
const HEIGHT: u32 = 640;

impl<'win> App<'win> {
    fn new(window: Window) -> App<'win> {
        let window = Arc::new(window);
        let surface_texture = SurfaceTexture::new(WIDTH, HEIGHT, Arc::clone(&window));
        let pixels = PixelsBuilder::new(WIDTH, HEIGHT, surface_texture)
            .texture_format(pixels::wgpu::TextureFormat::Rgba8Unorm)
            .build()
            .unwrap();
        Self { window, pixels }
    }
}

impl<'win> ApplicationHandler for App<'win> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        // let window = event_loop
        //     .create_window(
        //         Window::default_attributes()
        //             .with_inner_size(LogicalSize::new(WIDTH / 2, HEIGHT / 2)),
        //     )
        //     .unwrap();

        // let surface_texture = SurfaceTexture::new(WIDTH, HEIGHT, Arc::clone(&self.window));
        // self.pixels = PixelsBuilder::new(WIDTH, HEIGHT, surface_texture)
        //     .texture_format(pixels::wgpu::TextureFormat::Rgba8Unorm)
        //     .build()
        //     .unwrap();
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Close requested");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested | WindowEvent::Moved(_) | WindowEvent::Focused(_) => {
                let mut pixmap = Pixmap::new(WIDTH, HEIGHT).unwrap();

                let mut paint1 = Paint::default();
                paint1.set_color_rgba8(50, 107, 160, 255);
                paint1.anti_alias = false;
                let path1 = PathBuilder::from_circle(200.0, 200.0, 150.0).unwrap();

                pixmap.fill(Color::from_rgba8(255, 200, 255, 255));
                pixmap.fill_path(
                    &path1,
                    &paint1,
                    FillRule::Winding,
                    Transform::from_scale(1.3, 1.3),
                    None,
                );
                let frame = self.pixels.frame_mut();
                frame.copy_from_slice(pixmap.data());
                let _ = self.pixels.render().unwrap();
            }
            _ => (),
        }
    }
}

fn main() {
    // start zeno

    // for (i, v) in mask.iter().enumerate() {
    //     print!("{: ^3} ", v);
    //     if i % 64 == 0 {
    //         println!();
    //     }
    // }
    // end zeno
    //
    // start winit
    //

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let window = event_loop
        .create_window(
            Window::default_attributes().with_inner_size(LogicalSize::new(WIDTH / 2, HEIGHT / 2)),
        )
        .unwrap();

    let mut app = App::new(window);
    let _ = event_loop.run_app(&mut app);

    // end winit
}
