use std::io;

pub enum Token {
    Plus,
    Minus,
    NextMem,
    PrevMem,
    LoopStart(usize),
    LoopEnd(usize),
    Write,
    Read
}

pub fn scan(program: &str) -> io::Result<Vec<Token>> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut loops: Vec<(usize, usize)> = Vec::new();
    let mut i: usize = 0;

    for (index_file, character) in program.chars().enumerate() {
        tokens.push(
            match character {
                '+' => Token::Plus,
                '-' => Token::Minus,
                '>' => Token::NextMem,
                '<' => Token::PrevMem,
                '[' => Token::LoopStart(0),
                ']' => Token::LoopEnd(0),
                '.' => Token::Write,
                ',' => Token::Read,
                _ => continue,
            }
        );
        
        if character == '[' {
            loops.push((i, index_file));
        }
        else if character == ']' {
            let (start, _) = match loops.pop() {
                Some(top) => top,
                None => {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("at {index_file}, Unmatched ']'"))); 
                }   
            };

            tokens[start] = Token::LoopStart(i);
            tokens[i] = Token::LoopEnd(start);
        }

        i += 1;
    }

    if !loops.is_empty() {
        let (_, i) = loops.pop().unwrap();
        return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("at {i}, Unmatched '['")));
    }

    Ok(tokens)
}
