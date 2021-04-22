use std::env::args;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = args();
    args.next();

    if let Some(file) = args.next() {
        bf_rs::exec_from_file(file)?;
    } else {
        bf_rs::prompt()?;
    }

    Ok(())
}
