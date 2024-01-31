#![allow(
    dead_code,
    unused_assignments,
    unused_variables,
    unused_import_braces,
    unused_braces
)]

use std::fmt::Display;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::io::{BufReader, Bytes};

#[derive(Debug)]
enum InstructionCode {
    MOV,
    PUSH,
    POP,
}

#[derive(Debug)]
enum SubInstructionCode {
    WMakesDataW,
    DATA,
    LIT,
    D,
    S,
    W,
    V,
    Z,
    MOD,
    REG,
    RM,
}

struct SubInstruction {
    code: SubInstructionCode,
    size: u8,
    shift: u8,
    value: u8,
}

impl Display for SubInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Code: {:6?}, Size: {}, Shift: {}, value: {}",
            self.code, self.size, self.shift, self.value
        )
    }
}

impl SubInstruction {
    fn new(code: SubInstructionCode, size: u8, shift: u8, value: u8) -> Self {
        Self {
            code,
            shift,
            size,
            value,
        }
    }
}

struct Instruction {
    code: InstructionCode,
    sub_instructions: Vec<SubInstruction>,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = String::new();
        let mut total_size = 0;
        for sub in self.sub_instructions.iter() {
            d += &format!("{}\n", sub);
            total_size += sub.size;
        }
        write!(
            f,
            "Code: {:?}\n{}Total size: {}",
            self.code, d, total_size
        )
    }
}

impl Instruction {
    fn new(code: InstructionCode, sub_instructions: Vec<SubInstruction>) -> Self {
        Self {
            code,
            sub_instructions,
        }
    }
}

macro_rules! B {
    ($bits:literal) => {
        SubInstruction::new(
            SubInstructionCode::LIT,
            (format!("{:b}", $bits).len() - 1).try_into().unwrap(),
            0,
            $bits.try_into().unwrap(),
        )
    };
}
macro_rules! D {
    () => {
        SubInstruction::new(SubInstructionCode::D, 1, 0, 0)
    };
}

macro_rules! W {
    () => {
        SubInstruction::new(SubInstructionCode::W, 1, 0, 0)
    };
}

macro_rules! MOD {
    () => {
        SubInstruction::new(SubInstructionCode::MOD, 2, 0, 0)
    };
}
macro_rules! REG {
    () => {
        SubInstruction::new(SubInstructionCode::REG, 3, 0, 0)
    };
}
macro_rules! RM {
    () => {
        SubInstruction::new(SubInstructionCode::RM, 3, 0, 0)
    };
}

macro_rules! ImpW {
    ($value:literal) => {
        SubInstruction::new(SubInstructionCode::W, 0, 0, $value)
    };
}
macro_rules! ImpD {
    ($value:literal) => {
        SubInstruction::new(SubInstructionCode::D, 0, 0, $value)
    };
}

macro_rules! x86 {
    ($arr:ident, $code:ident, $( $sub:expr ),* ) => {
        {

        let mut subs: Vec<SubInstruction> = vec![];

        $( subs.push($sub); )*

       $arr.push(Instruction::new(
        $code,
        subs,
    ));
    }
    };
}

fn main() -> io::Result<()> {
    use InstructionCode::{MOV, PUSH};

    // let bytes = fs::read("listing_0037_single_register_mov").unwrap();

    let mut instructions: Vec<Instruction> = vec![];

    x86!(
        instructions,
        MOV,
        B!(0b1100011),
        D!(),
        W!(),
        MOD!(),
        REG!(),
        RM!()
    );

    x86!(
        instructions,
        PUSH,
        B!(0b11111111),
        MOD!(),
        B!(0b110),
        RM!(),
        ImpW!(1),
        ImpD!(1)
    );

    for inst in instructions.iter() {
        println!("{}\n", inst);
    }

    Ok(())
}
