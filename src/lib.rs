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

    println!("bf_rs by LelioMarcos\nType \"#\" to display the current memory position and the value in it, or \"clm\" to clear the memory.");

    loop {
        match rl.readline(">> ") {
            Ok(line) => {
                rl.add_history_entry(&line);
                if line == "clm" {
                    bf.clear_memory();
                    println!("Memory Cleared");
                } else if line == "#" {
                    println!("{}", bf.to_string());
                } else {
                    bf.run(&line);
                }
            }
            Err(ReadlineError::Eof) => {
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
