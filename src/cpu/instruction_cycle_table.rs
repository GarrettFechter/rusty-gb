// get the number of cycles an instruction should take
// should be in a data structure not a match statement
// generated from a table of opcodes with a combination of excel and vim macros

pub fn get_cycle_count(instruction: u8, prefixed: bool) -> u8 {
    match (instruction, prefixed) {
        (0x00, false) => 4,
        (0x01, false) => 12,
        (0x02, false) => 8,
        (0x03, false) => 8,
        (0x04, false) => 4,
        (0x05, false) => 4,
        (0x06, false) => 8,
        (0x07, false) => 4,
        (0x08, false) => 20,
        (0x09, false) => 8,
        (0x0A, false) => 8,
        (0x0B, false) => 8,
        (0x0C, false) => 4,
        (0x0D, false) => 4,
        (0x0E, false) => 8,
        (0x0F, false) => 4,

        (0x10, false) => 4,
        (0x11, false) => 12,
        (0x12, false) => 8,
        (0x13, false) => 8,
        (0x14, false) => 4,
        (0x15, false) => 4,
        (0x16, false) => 8,
        (0x17, false) => 4,
        (0x18, false) => 12,
        (0x19, false) => 8,
        (0x1A, false) => 8,
        (0x1B, false) => 8,
        (0x1C, false) => 4,
        (0x1D, false) => 4,
        (0x1E, false) => 8,
        (0x1F, false) => 4,

        _ => 0,
    }
}
