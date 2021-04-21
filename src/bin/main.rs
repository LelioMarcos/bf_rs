use std::env::args;

const MESSAGE: &str = r"bf_rs v0.2 by LelioMarcos (2021).

Usage:
    ./bf_rs <path_to_file>
    ./bf_rs <code>";

fn main() {
    let mut args = args();
    args.next();

    if let Some(file) = args.next() {
        if file.ends_with(".bf") {
            bf_rs::exec_from_file(file);
        } else {
            bf_rs::exec(file);
        }
    } else {
        println!("{}", MESSAGE);
    }
}
