use std::{thread, time};

// use wgpu;

use bevy::{
    core::Time,
    prelude::*,
    //render::camera::OrthographicCameraBundle,
    window::WindowDescriptor,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
};
// use std::convert::TryInto;

use bevy_pixels::prelude::*;
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
fn main() {
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

    App::new()
        .insert_resource(WindowDescriptor {
            title: "rusty-gb".to_string(),
            width: 160.0,
            height: 144.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PixelsPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        //.add_system(draw)
        .insert_resource(FPSTimer(Timer::from_seconds(30.0, true)))
        .add_system_to_stage(PixelsStage::Draw, main_system)
        //.add_system_to_stage(PixelsStage::Draw, draw)
        .run();

    /*
    let instances = wgpu::Instance::new(wgpu::Backends::VULKAN);
    for adapter in instances.enumerate_adapters(wgpu::Backends::VULKAN) {
        println!("{:?}", adapter.get_info());
    }
    */
}

struct FPSTimer(Timer);

fn draw(
    mut pixels_resource: ResMut<PixelsResource>) {

    // fill with random data
    let frame: &mut [u8] = pixels_resource.pixels.get_frame();
    let random = rand::random::<u8>();

    // fill with random data
    for i in 0..frame.len() {
        frame[i] = random;
    }
}

fn main_system(
    time: Res<Time>,
    mut timer: ResMut<FPSTimer>,
    mut pixels_resource: ResMut<PixelsResource>) {

    if timer.0.tick(time.delta()).just_finished() {
        info!("{:?}", time.delta_seconds());
        info!("Updating frame");
        // get mutable slice for pixel buffer
        let frame: &mut [u8] = pixels_resource.pixels.get_frame();

        // fill with random data
        for i in 0..frame.len() {
            frame[i] = rand::random::<u8>();
        }
    }
}
