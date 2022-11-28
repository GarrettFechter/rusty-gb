// standard
use futures::executor;
use std::{thread, time};

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use winit::dpi::LogicalSize;

use pixels::{Pixels, SurfaceTexture};

// internal
mod cpu;
mod ppu;

use cpu::CPU;
use cpu::MemoryBus;
use cpu::Registers;
use cpu::registers::FlagsRegister;
// use ppu::PPU;

#[allow(dead_code)]
#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
// Controller to run gameboy emulator
pub fn run() {
    configure_logger();

    // initialize window
    // let mut input = WinitInputHelper::new();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("rusty-gb")
        .with_inner_size(LogicalSize::new(1000.0, 1000.0))
        .build(&event_loop)
        .unwrap();

    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);

    // append window to web canvas
    #[cfg(target_arch = "wasm32")]
    {
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

    // initialize pixels
    let mut pixels: Pixels = executor::block_on(
        Pixels::new_async(320, 320, surface_texture)
    ).unwrap();

    // initialize rusty-gb objects
    let mut cpu = CPU::new();
    cpu.bus.memory[0] = 0x00;

    // TODO: load something into memory
    //       maybe based on a param?

    cpu.step();
    assert!(cpu.is_halted);


    event_loop.run(move |event, _, control_flow| {
        // TODO - move window stuff into PPU
        if let Event::RedrawRequested(_) = event {
            populate_frame_random_rgba(pixels.get_frame_mut());
            if pixels.render().is_err() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // cpu.frame_step();
        window.request_redraw();
        // thread::sleep(time::Duration::from_micros(cpu.frame_delay));
    });
}

fn populate_frame_random_rgba(frame: &mut [u8]) {
    // update frame with random rgba values
    let mut i = rand::random::<usize>() % frame.len();
    i = i - (i % 4);

    let rgba = [rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), 255];
    frame[i..i+4].copy_from_slice(&rgba);
}

fn configure_logger() {
    // configure logger based on target arch
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }
}