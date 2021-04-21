use std::fs;

mod brainfuck;
use brainfuck::*;

pub fn exec_from_file(path: String) {
    let program = fs::read_to_string(path).unwrap();
    exec(program);
}

pub fn exec(program: String) {
    let mut bf = BrainFuck::new();
    bf.run(&program);
}
