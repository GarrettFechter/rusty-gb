use std::{thread, time};

use bevy::{
    core::Time,
    prelude::*,
    render::camera::OrthographicCameraBundle,
    window::WindowDescriptor,
};
// use std::convert::TryInto;

mod ppu;
use ppu::PPU;

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
        .add_plugin(HelloPlugin)
        .run();

}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn().insert(Person).insert(Name("John".to_string()));
    commands.spawn().insert(Person).insert(Name("Bob".to_string()));
    commands.spawn().insert(Person).insert(Name("Bill".to_string()));
}

struct GreetTimer(Timer);

fn greet_people(
    time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // update time with the time elapsed since the last update
    // // if that caused the timer to finished, say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("Hello, {}!", name.0);
        }
    }
}

fn hello_world() {
    println!("Hello, world!");
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        // true makes from_seconds repeat
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_people)
            .add_system(greet_people);
        //.add_system(hello_world)
        // app.add_resource(PPU::new());
        // app.add_resource(CPU::new());
    }
}
