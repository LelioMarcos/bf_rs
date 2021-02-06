use bf_rs::*;
use std::env::args;

const MESSAGE: &str = r"bf_rs v0.1 by LelioMarcos (2021).

Usage:
    ./bf_rs path_to_file";

fn main() {
    let mut args = args();
    args.next();

    let mut bf = BrainFuck::new();

    if let Some(file) = args.next() {
        bf.run(file);
    } else {
        println!("{}", MESSAGE);
    }
}
