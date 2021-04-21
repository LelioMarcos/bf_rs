use std::io;

pub enum Token {
    Plus,
    Minus,
    NextMem,
    PrevMem,
    LoopStart,
    LoopEnd,
    Write,
    Read,
    Debug,
}

pub fn scan(program: &str) -> io::Result<Vec<Token>> {
    let mut tokens: Vec<Token> = Vec::new();

    for c in program.chars() {
        match c {
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '>' => tokens.push(Token::NextMem),
            '<' => tokens.push(Token::PrevMem),
            '[' => tokens.push(Token::LoopStart),
            ']' => tokens.push(Token::LoopEnd),
            '.' => tokens.push(Token::Write),
            ',' => tokens.push(Token::Read),
            '#' => tokens.push(Token::Debug),
            _ => (),
        }
    }

    Ok(tokens)
}
