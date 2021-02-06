use std::fmt;
use std::{char, fs, io, io::Read};

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

    fn load_program(&self, path: &str) -> io::Result<Vec<char>> {
        if !path.ends_with(".bf") {
            return Err(io::Error::from(io::ErrorKind::InvalidInput));
        }
        let program = fs::read_to_string(path)?;

        Ok(program.chars().collect())
    }

    pub fn run(&mut self, path: String) {
        let file = self.load_program(path.as_str()).unwrap_or_else(|err| {
            panic!("{}", err);
        });

        let mut loop_start_stack: Vec<usize> = Vec::new();

        let mut cur_index = 0;
        while cur_index < file.len() {
            match file[cur_index] {
                '+' => self.mem[self.ptr] = self.mem[self.ptr].wrapping_add(1),
                '-' => self.mem[self.ptr] = self.mem[self.ptr].wrapping_sub(1),
                '>' => self.ptr = self.ptr.wrapping_add(1),
                '<' => self.ptr = self.ptr.wrapping_sub(1),
                '[' => {
                    if self.mem[self.ptr] == 0 {
                        let mut count = 1;
                        for (i, c) in file.iter().enumerate().skip(cur_index + 1) {
                            match *c {
                                '[' => count += 1,
                                ']' => {
                                    count -= 1;
                                    if count == 0 {
                                        cur_index = i;
                                        break;
                                    }
                                }
                                _ => (),
                            }
                        }
                    } else {
                        loop_start_stack.push(cur_index);
                    }
                }
                ']' => cur_index = loop_start_stack.pop().unwrap() - 1,
                '.' => print!("{}", char::from_u32(self.mem[self.ptr] as u32).unwrap()),
                ',' => {
                    self.mem[self.ptr] = io::stdin()
                        .bytes()
                        .next()
                        .and_then(|result| result.ok())
                        .map(|byte| byte as u8)
                        .unwrap()
                }
                '#' => println!("{}", self.to_string()),
                _ => (),
            }
            self.ptr %= file.len();
            cur_index += 1;
        }
    }
}

impl fmt::Display for BrainFuck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "DEBUG: {:?} \n mem[{}] = {}",
            self.mem, self.ptr, self.mem[self.ptr]
        )?;
        Ok(())
    }
}
