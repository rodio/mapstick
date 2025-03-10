use std::sync::Arc;

use pixels::{Pixels, PixelsBuilder, SurfaceTexture};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};
use zeno::Transform;

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
                let (mask, _) = // zeno::Mask::new("M 8,56 32,8 56,56 Z")
                    zeno::Mask::new("M3.32031 11.6835C3.32031 16.6541 7.34975 20.6835 12.3203 20.6835C16.1075 20.6835 19.3483 18.3443 20.6768 15.032C19.6402 15.4486 18.5059 15.6834 17.3203 15.6834C12.3497 15.6834 8.32031 11.654 8.32031 6.68342C8.32031 5.50338 8.55165 4.36259 8.96453 3.32996C5.65605 4.66028 3.32031 7.89912 3.32031 11.6835Z")
                    .size(WIDTH, HEIGHT)
                    // .format(zeno::Format::subpixel_bgra())
                    .transform(Some(Transform::scale(25f32, 25f32)))
                    .render();

                // println!(
                //     "chunk: {:#?}",
                //     mask.chunks(4).filter(|c| *(c.get(3).unwrap()) != 0).nth(0)
                // );

                let frame = self.pixels.frame_mut();
                println!("frame size {}", frame.len());
                println!("mask size {}", mask.len());
                // for (i, v) in mask.iter().enumerate() {
                //     print!("{: ^3} ", v);
                //     if i % (64 * 4) == 0 {
                //         println!();
                //     }
                // }
                for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                    let rgba = if mask[i] != 0 {
                        [0x00, 0x00, 0x00, mask[i]]
                    } else {
                        [0xff, 0xff, 0xff, 255]
                    };
                    // let mut rgba = [
                    //     mask[i * 4 + 2],      // b
                    //     mask[i * 4 + 1],      // g
                    //     mask[i * 4 + 0],      // r
                    //     10 + mask[i * 4 + 3], // a
                    // ];
                    // if mask[i * 4 + 2] == 255 {
                    //     rgba[0] = 0x0f;
                    // }
                    // if mask[i * 4 + 1] == 255 {
                    //     rgba[1] = 0x2f;
                    // }
                    // if mask[i * 4 + 0] == 255 {
                    //     rgba[2] = 0x9;
                    // }
                    // if mask[i * 4 + 2] != 0 {
                    //     println!("b={}", mask[i * 4 + 2]);
                    // }
                    // if mask[i * 4 + 0] == 0
                    //     && mask[i * 4 + 1] == 0
                    //     && mask[i * 4 + 2] == 0
                    //     && mask[i * 4 + 3] == 0
                    // {
                    //     pixel.copy_from_slice(&[0x1f, 0xff, 0xff, 255]);
                    //     pixel.copy_from_slice(&[0xff, 0xff, 0xff, 255]);
                    // } else {
                    //     pixel.copy_from_slice(&rgba);
                    // }
                    pixel.copy_from_slice(&rgba);
                }

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
