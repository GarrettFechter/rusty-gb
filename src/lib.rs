// standard
use std::{thread, time};

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

use winit::event_loop::{ControlFlow, EventLoop};
use winit::event::{Event, VirtualKeyCode};

mod cpu;
use cpu::CPU;
use cpu::MemoryBus;
use cpu::Registers;
use cpu::registers::FlagsRegister;

mod ppu;
use ppu::PPU;

#[allow(dead_code)]
#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
// Controller to run gameboy emulator
pub fn run() {
    configure_logger();

    let event_loop = EventLoop::new();

    // initialize rusty-gb objects
    let mut cpu = CPU::new();
    cpu.bus.memory[0] = 0x00;

    let mut ppu = PPU::new(&event_loop);

    // TODO: load something into memory
    //       maybe based on a param?

    cpu.step();
    assert!(cpu.is_halted);


    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            if ppu.render().is_err() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // cpu.frame_step();
        ppu.request_refresh();
        // thread::sleep(time::Duration::from_micros(cpu.frame_delay));
    });
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