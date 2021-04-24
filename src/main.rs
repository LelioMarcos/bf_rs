use std::env::args;

fn main() {
    let mut args = args();
    args.next();

    if let Some(file) = args.next() {
        bf_rs::exec_from_file(file).unwrap_or_else(|err| panic!("{}", err));
    } else {
        bf_rs::prompt().unwrap_or_else(|err| panic!("{}", err));
    }
}
