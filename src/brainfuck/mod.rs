mod scanner;
use scanner::*;

use std::{char, fmt, io, io::Read};

pub struct BrainFuck {
    mem: [u8; 30000],
    ptr: usize,
}

impl BrainFuck {
    pub fn new() -> BrainFuck {
        BrainFuck {
            mem: [0; 30000],
            ptr: 0,
        }
    }

    pub fn clear_memory(&mut self) {
        self.mem = [0; 30000];
    }

    fn next_mem(&mut self) {
        self.ptr += 1;
        self.ptr %= self.mem.len();
    }

    fn prev_mem(&mut self) {
        self.ptr = self.ptr.wrapping_sub(1);
        if self.ptr == usize::MAX {
            self.ptr = self.mem.len() - 1;
        }
    }

    pub fn run(&mut self, program: &str) {
        let tokens = scan(program).unwrap_or_else(|err| {
            panic!("{}", err);
        });

        let mut outputted = false;

        let mut cur_index = 0;
        
        while cur_index < tokens.len() {
            match tokens[cur_index] {
                Token::Plus => self.mem[self.ptr] = self.mem[self.ptr].wrapping_add(1),
                Token::Minus => self.mem[self.ptr] = self.mem[self.ptr].wrapping_sub(1),
                Token::NextMem => self.next_mem(),
                Token::PrevMem => self.prev_mem(),
                Token::LoopStart(end) => {
                    if self.mem[self.ptr] == 0 {
                        cur_index = end;
                    }
                }
                Token::LoopEnd(start) => cur_index = start.wrapping_sub(1),
                Token::Write => {
                    print!("{}", char::from_u32(self.mem[self.ptr] as u32).unwrap());
                    outputted = true;
                }
                Token::Read => {
                    self.mem[self.ptr] = io::stdin()
                        .bytes()
                        .next()
                        .and_then(|result| result.ok())
                        .map(|byte| byte as u8)
                        .unwrap()
                }
            }

            cur_index = cur_index.wrapping_add(1);
        }
        
        if outputted {
            println!();
        }
    }
}

impl fmt::Display for BrainFuck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "mem[{}] = {}", self.ptr, self.mem[self.ptr])?;
        Ok(())
    }
}
