use futures::executor;

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

use winit::event;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
use winit_input_helper::WinitInputHelper;
use winit::dpi::{LogicalSize, PhysicalSize};

use pixels::{Error, Pixels, SurfaceTexture, raw_window_handle::HasRawWindowHandle};

pub struct PPU {
    input: WinitInputHelper,
    pixels: Pixels,
    window: Window,
}

impl PPU {
    pub fn new(event_loop: &EventLoop<()>) -> PPU {
        let window = WindowBuilder::new()
            .with_title("rusty-gb")
            .with_inner_size(LogicalSize::new(1000.0, 1000.0))
            .build(&event_loop)
            .unwrap();

        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);

        let pixels = executor::block_on(
            Pixels::new_async(320, 320, surface_texture))
            .unwrap();
        let input = WinitInputHelper::new();

        let mut ppu = PPU {
            input,
            pixels,
            window,
        };

        // append window to web canvas
        #[cfg(target_arch = "wasm32")]
            ppu.append_window_to_web_canvas();

        ppu
    }

    pub fn render(&mut self) -> Result<(), Error> {
        self.populate_frame_random_rgba();
        self.pixels.render()
    }
    
    pub fn request_refresh(&mut self) {
        self.window.request_redraw();
    }

    #[cfg(target_arch="wasm32")]
    fn append_window_to_web_canvas(&mut self) {
        // set window size manually (winit prevents sizing with CSS)
        // use winit::dpi::PhysicalSize;
        // window.set_inner_size(PhysicalSize::new(1000, 1000));
        
        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("rusty_gb_body")?;
                let canvas = web_sys::Element::from(self.window.canvas());
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Unable to append to canvas")
    }

    fn populate_frame_random_rgba(&mut self) {
        // update frame with random rgba values
        let frame: &mut [u8] = self.pixels.get_frame_mut();
        let mut i = rand::random::<usize>() % frame.len();
        i = i - (i % 4);

        let rgba = [rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), 255];
        frame[i..i+4].copy_from_slice(&rgba);
    }
}