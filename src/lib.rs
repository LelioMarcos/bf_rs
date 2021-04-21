use std::fs;

mod brainfuck;
use brainfuck::*;

pub fn run_from_file(path: String) {
    let program = fs::read_to_string(path).unwrap();
    run(program);
}

pub fn run(program: String) {
    let mut bf = BrainFuck::new();
    bf.run(program);
}
