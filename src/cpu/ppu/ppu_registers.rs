use std::convert;

use super::PPU;

// PPU Control Register (LCDC) 0xFF40
#[derive(Copy, Clone)]
pub struct PPUControlRegister {
    pub lcd_en:   bool,
    pub win_map:  bool,
    pub win_en:   bool,
    pub tile_sel: bool,
    pub bg_map:   bool,
    pub obj_size: bool,
    pub obj_en:   bool,
    pub bg_en:    bool,
}

impl PPUControlRegister {
    pub fn new() -> PPUControlRegister {
        PPUControlRegister {
            lcd_en : false,
            win_map : false,
            win_en : false,
            tile_sel : false,
            bg_map : false,
            obj_size : false,
            obj_en : false,
            bg_en : false,
        }
    }
}

const LCD_EN_BYTE_POSITION:   u8 = 7;
const WIN_MAP_BYTE_POSITION:  u8 = 6;
const WIN_EN_BYTE_POSITION:   u8 = 6;
const TILE_SEL_BYTE_POSITION: u8 = 5;
const BG_MAP_BYTE_POSITION:   u8 = 3;
const OBJ_SIZE_BYTE_POSITION: u8 = 2;
const OBJ_EN_BYTE_POSITION:   u8 = 1;
const BG_EN_BYTE_POSITION:    u8 = 0;


impl convert::From<PPUControlRegister> for u8 {
    fn from(reg: PPUControlRegister) -> u8 {
        (if reg.lcd_en   { 1 } else { 0 }) << LCD_EN_BYTE_POSITION   |
        (if reg.win_map  { 1 } else { 0 }) << WIN_MAP_BYTE_POSITION  |
        (if reg.win_en   { 1 } else { 0 }) << WIN_EN_BYTE_POSITION   |
        (if reg.tile_sel { 1 } else { 0 }) << TILE_SEL_BYTE_POSITION |
        (if reg.bg_map   { 1 } else { 0 }) << BG_MAP_BYTE_POSITION   |
        (if reg.obj_size { 1 } else { 0 }) << OBJ_SIZE_BYTE_POSITION |
        (if reg.obj_en   { 1 } else { 0 }) << OBJ_EN_BYTE_POSITION   |
        (if reg.bg_en    { 1 } else { 0 }) << BG_EN_BYTE_POSITION
    }
}

impl convert::From<u8> for PPUControlRegister {
    fn from(byte: u8) -> Self {
        let lcd_en   = ((byte >> LCD_EN_BYTE_POSITION)   & 0b1) != 0;
        let win_map  = ((byte >> WIN_MAP_BYTE_POSITION)  & 0b1) != 0;
        let win_en   = ((byte >> WIN_EN_BYTE_POSITION)   & 0b1) != 0;
        let tile_sel = ((byte >> TILE_SEL_BYTE_POSITION) & 0b1) != 0;
        let bg_map   = ((byte >> BG_MAP_BYTE_POSITION)   & 0b1) != 0;
        let obj_size = ((byte >> OBJ_SIZE_BYTE_POSITION) & 0b1) != 0;
        let obj_en   = ((byte >> OBJ_EN_BYTE_POSITION)   & 0b1) != 0;
        let bg_en    = ((byte >> BG_EN_BYTE_POSITION)    & 0b1) != 0;

        PPUControlRegister {
            lcd_en,
            win_map,
            win_en,
            tile_sel,
            bg_map,
            obj_size,
            obj_en,
            bg_en,
        }
    }
}

// PPU Status Register (LCDC) 0xFF41
#[derive(Copy, Clone)]
pub struct PPUStatusRegister {
    pub intr_lyc: bool,
    pub intr_m2:  bool,
    pub intr_m1:  bool,
    pub intr_m0:  bool,
    pub lyc_stat: bool,
    pub lcd_mode: [bool; 2],
}

impl PPUStatusRegister {
    pub fn new() -> PPUStatusRegister {
        PPUStatusRegister {
            intr_lyc : false,
            intr_m2  : false,
            intr_m1  : false,
            intr_m0  : false,
            lyc_stat : false,
            lcd_mode : [false, false]
        }
    }
}

const INTR_LYC_BYTE_POSITION: u8 = 6;
const INTR_M2_BYTE_POSITION:  u8 = 5;
const INTR_M1_BYTE_POSITION:  u8 = 4;
const INTR_M0_BYTE_POSITION:  u8 = 3;
const LYC_STAT_BYTE_POSITION: u8 = 2;
const LCD_MODE_BYTE_POSITION: u8 = 1;

impl convert::From<PPUStatusRegister> for u8 {
    fn from(reg: PPUStatusRegister) -> u8 {
        (if reg.intr_lyc    { 1 } else { 0 }) << INTR_LYC_BYTE_POSITION |
        (if reg.intr_m2     { 1 } else { 0 }) << INTR_M2_BYTE_POSITION  |
        (if reg.intr_m1     { 1 } else { 0 }) << INTR_M1_BYTE_POSITION  |
        (if reg.intr_m0     { 1 } else { 0 }) << INTR_M0_BYTE_POSITION  |
        (if reg.lyc_stat    { 1 } else { 0 }) << LYC_STAT_BYTE_POSITION |
        (if reg.lcd_mode[1] { 1 } else { 0 }) << LCD_MODE_BYTE_POSITION |
        (if reg.lcd_mode[0] { 1 } else { 0 })
    }
}

impl convert::From<u8> for PPUStatusRegister {
    fn from(byte: u8) -> Self {
        let intr_lyc = ((byte >> INTR_LYC_BYTE_POSITION) & 0b1) != 0;
        let intr_m2  = ((byte >> INTR_M2_BYTE_POSITION)  & 0b1) != 0;
        let intr_m1  = ((byte >> INTR_M1_BYTE_POSITION)  & 0b1) != 0;
        let intr_m0  = ((byte >> INTR_M0_BYTE_POSITION)  & 0b1) != 0;
        let lyc_stat = ((byte >> LYC_STAT_BYTE_POSITION) & 0b1) != 0;
        let lcd_mode = [
            ((byte >> LCD_MODE_BYTE_POSITION) & 0b1) != 0,
            (byte & 0b1) != 0,
        ];

        PPUStatusRegister {
            intr_lyc,
            intr_m2,
            intr_m1,
            intr_m0,
            lyc_stat,
            lcd_mode,
        }
    }
}

// Vertical Scroll Register (SCY) 0xFF42
#[derive(Copy, Clone)]
pub struct VerticalScrollRegister {
    pub scy: u8
}

// Horizontal Scroll Register (SCX) 0xFF43
#[derive(Copy, Clone)]
pub struct HorizontalScrollRegister {
    pub scx: u8
}

// Scanline Register (LY) 0xFF44
#[derive(Copy, Clone)]
pub struct ScanlineRegister {
    pub ly: u8
}

// Scanline Compare Register (LYC) 0xFF45
#[derive(Copy, Clone)]
pub struct ScanlineCompareRegister {
    pub lyc: u8
}