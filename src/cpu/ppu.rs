use futures::executor;

pub mod ppu_registers;
pub use ppu_registers::*;

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
    control_reg: PPUControlRegister,
    status_reg: PPUStatusRegister,
    vertical_scroll_reg: VerticalScrollRegister,
    horizontal_scroll_reg: HorizontalScrollRegister,
    scaline_reg: ScanlineRegister,
    scanline_compare_reg: ScanlineCompareRegister,
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

        // append window to web canvas
        #[cfg(target_arch = "wasm32")]
            PPU::append_window_to_web_canvas(&window);

        let pixels = executor::block_on(
            Pixels::new_async(320, 320, surface_texture))
            .unwrap();
        let input = WinitInputHelper::new();

        PPU {
            input,
            pixels,
            window,
            control_reg: PPUControlRegister::new(),
            status_reg: PPUStatusRegister::new(),
            vertical_scroll_reg: VerticalScrollRegister { scy: 0 },
            horizontal_scroll_reg: HorizontalScrollRegister { scx: 0 },
            scaline_reg: ScanlineRegister { ly: 0 },
            scanline_compare_reg: ScanlineCompareRegister { lyc: 0 },
        }
    }

    pub fn render(&mut self) -> Result<(), Error> {
        self.populate_frame_random_rgba();
        self.pixels.render()
    }
    
    pub fn request_refresh(&mut self) {
        self.window.request_redraw();
    }

    #[cfg(target_arch="wasm32")]
    fn append_window_to_web_canvas(window: &Window) {
        // set window size manually (winit prevents sizing with CSS)
        // use winit::dpi::PhysicalSize;
        // window.set_inner_size(PhysicalSize::new(1000, 1000));
        
        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("rusty_gb_body")?;
                let canvas = web_sys::Element::from(window.canvas());
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