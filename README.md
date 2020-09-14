# rusty-gb

A Game Boy emulator in Rust

## Goals
* High-level Game Boy (DMG-01) emulation
* Play Tetris
* Play Pokemon

## Non-Goals
* Cycle-accurate emulation
* Serial cable support

## Status
Actively working on this project

- [ ] CPU
  - [x] LD instructions
  - [x] 8 bit arithmetic/logical instructions
  - [x] 16 bit arithmetic/logical instructions
  - [x] control instructions
  - [x] jump, call, return, reset instructions
  - [x] misc instructions
  - [ ] 0xCB prefixed instructions
  - [ ] limited unit testing
  - [ ] integration testing by comparing to register values in BGB (Wine) after running a game 
- [ ] Interrupt Controller
- [ ] Timers
- [ ] Pixel Processing Unit (PPU) with SDL2(?) implementation
  - [ ] run boot ROM
  - [ ] create manual boot ROM logo
  - [ ] play Tetris
- [ ] Sound Controller
  - [ ] Channel 1 ("Pulse A")
  - [ ] Channel 2 ("Pulse B")
  - [ ] Channel 3 ("Wave")
  - [ ] Channel 4 ("Noise")

Possible Extensions:
- [ ] WASM support, to run in-browser
  - [ ] publish on a website
- [ ] fast forward (2x, 4x, 8x, 16x emulation)
- [ ] save states (probably simply by copying all memory)
  - [ ] rewind 10 seconds
- [ ] option to display 160x144 viewport inside of 256x256 background, depends on PPU emulation accuracy
- [ ] add Game Boy color support

## Reference Material
This is all probably a subset of [this curated list of Game Boy development sources](https://gbdev.io/list.html)
* I heavily used [DMG-01: How to emulate a Game Boy](https://blog.ryanlevick.com/DMG-01/public/book/) by [Ryan Levick](https://github.com/rylev) as a starting point
* [The Ultimate Game Boy Talk](https://youtu.be/HyzD8pNlpwI) as a fairly comprehensive overview
* [Game Boy Technical Reference](https://gekkio.fi/files/gb-docs/gbctr.pdf) by [Gekkio](https://github.com/Gekkio). I also looked at some of his [Rust GB emulator](https://github.com/Gekkio/mooneye-gb), especially for the implementation of the DAA instruction
* [Game Boy Pandocs](https://gbdev.io/pandocs/)
* [Official Nintendo Manual](https://ia803208.us.archive.org/9/items/GameBoyProgManVer1.1/GameBoyProgManVer1.1.pdf)
* [Pastraise Opcode Reference](https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html)
* [GameBoy Opcode Summary](http://www.devrs.com/gb/files/opcodes.html), though this has a few typos
* z80 CPU User Manual to describe some CPU instructions
* [Bootstrap ROM contents](https://gbdev.gg8.se/wiki/articles/Gameboy_Bootstrap_ROM)
