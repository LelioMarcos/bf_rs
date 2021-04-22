use rustyline::{error::ReadlineError, Editor};
use std::{fs, io};

mod brainfuck;
use brainfuck::*;

pub fn exec_from_file(path: String) -> io::Result<()> {
    let program = fs::read_to_string(path)?;

    let mut bf = BrainFuck::new();
    bf.run(&program);

    Ok(())
}

pub fn prompt() -> rustyline::Result<()> {
    let mut bf = BrainFuck::new();
    let mut rl = Editor::<()>::new();

    rl.load_history("history.txt").ok();

    println!("bf_rs v0.2 by LelioMarcos (2021)");

    loop {
        match rl.readline(">> ") {
            Ok(line) => {
                rl.add_history_entry(&line);
                if line == "ClearMem" {
                    bf.clear_memory();
                    println!("Memory Cleard");
                } else {
                    bf.run(&line);
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("Stop");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("Exiting");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    rl.save_history("history.txt")?;

    Ok(())
}
