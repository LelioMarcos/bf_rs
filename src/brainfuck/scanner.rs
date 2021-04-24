use std::io;

pub enum Token {
    Plus,
    Minus,
    NextMem,
    PrevMem,
    LoopStart,
    LoopEnd,
    Write,
    Read
}

pub fn scan(program: &str) -> io::Result<Vec<Token>> {
    let mut tokens: Vec<Token> = Vec::new();

    for c in program.chars() {
        tokens.push(
            match c {
                '+' => Token::Plus,
                '-' => Token::Minus,
                '>' => Token::NextMem,
                '<' => Token::PrevMem,
                '[' => Token::LoopStart,
                ']' => Token::LoopEnd,
                '.' => Token::Write,
                ',' => Token::Read,
                _ => continue,
            }
        )
    }

    Ok(tokens)
}
