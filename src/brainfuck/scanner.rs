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
    let mut loops: Vec<usize> = Vec::new();

    for (i, c) in program.chars().enumerate() {
        tokens.push(
            match c {
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
        
        if c == '[' {
            loops.push(i);
        } else if c == ']' {
            let start = loops.pop().unwrap();
            let end = i;

            tokens[start] = Token::LoopStart(end);
            tokens[end] = Token::LoopEnd(start);
        }
    }

    Ok(tokens)
}
