use std::{thread, time};

use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use winit::dpi::LogicalSize;

use pixels::{Error, Pixels, SurfaceTexture};

// use std::convert::TryInto;

use rand::prelude::*;

mod ppu;
// use ppu::PPU;

mod cpu;
use cpu::CPU;
use cpu::MemoryBus;
use cpu::Registers;
use cpu::registers::FlagsRegister;

#[allow(dead_code)]
// Controller to run gameboy emulator
fn main() -> Result<(), Error> {
    let mut memory = MemoryBus {
        memory: [0; 0xFFFF],
    };

    memory.memory[0] = 0x00;

    // read instructions into memory

    let fr = FlagsRegister {
        zero:       false,
        subtract:   false,
        half_carry: false,
        carry:      false,
    };

    let my_registers = Registers {
        a: 0,
        b: 0,
        c: 0,
        d: 0,
        e: 0,
        f: fr,
        h: 0,
        l: 0,
    };

    let mut my_cpu = CPU {
        frequency: 4194304, // 4.194304 MHz
        frame_delay: 16750, // equivalent to 59.7 fps
        reg: my_registers,
        bus: memory,
        pc: 0,
        sp: 0xFFFF,
        interrupt_enable: false,
        is_halted: false,
        is_stopped: false,
    };


    my_cpu.step();
    // assert!(my_cpu.is_halted);

    /*
       loop {
       my_cpu.frame_step();
       thread::sleep(time::Duration::from_micros(my_cpu.frame_delay));
       }
       */

    let mut input = WinitInputHelper::new();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("rusty-gb")
        .with_inner_size(LogicalSize::new(1000.0, 1000.0))
        .build(&event_loop)
        .unwrap();

    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut pixels = Pixels::new(320, 320, surface_texture)?;

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            draw(pixels.get_frame());
            if pixels.render().is_err() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        window.request_redraw();
    });

}

fn draw(frame: &mut [u8]) {
    let mut i = rand::random::<usize>() % frame.len();
    i = i - (i % 4);

    let rgba = [rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), 255];
    frame[i..i+4].copy_from_slice(&rgba);
}
