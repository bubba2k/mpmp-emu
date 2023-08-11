mod decoder;
mod hex_parser;
mod ir;
mod program;
mod runtime;

extern crate num;
#[macro_use]
extern crate num_derive;

use std::env;
use std::process::exit;

use crate::hex_parser::bytevec_from_hexfile;
use crate::program::Program;
use crate::runtime::CpuState;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    if args.len() < 2 {
        println!("Usage: <hex_file_path>");
        exit(1);
    }

    let hex_vec = bytevec_from_hexfile(&args[1]).unwrap();
    let program = Program::from(hex_vec.as_slice());

    let mut cpu = CpuState::default();

    while !cpu.received_halt {
        cpu.execute_next_prog_op(&program);
    }

    println!("{}", cpu.ostream.string);
}
