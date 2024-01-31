use std::fs::{File};
use std::fs;
use std::io::{BufReader, Bytes};
use std::io;
use std::io::prelude::*;

fn get_bit_at(x: u8, n: u8) -> bool {
    x & (1 << 3) != 0
}

// #[x86("mov", 6, D, W, MOD, REG, RM, DISP-LO, DISP-HI)]
// struct Instruction {

// }

fn main() -> io::Result<()> {
    let bytes = fs::read("listing_0037_single_register_mov").unwrap();

    Ok(())
}
