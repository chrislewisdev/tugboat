use std::{collections::VecDeque, fs};

enum Token {
    Fn,
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    Equals,
    Plus,
    Minus,
    Star,
    Slash,
    Semicolon,
}
enum Stmt {}
enum _Expr {}

pub fn compile(name: &str, contents: String) {
    let tokens = lex(contents);
    let ast = parse(tokens);
    // *Probably* going to need some more steps here.
    let asm = codegen(ast);

    let result = fs::write(format!("{}.asm", name), asm);
    match result {
        Err(err) => println!("Failed to write assembly: {}", err),
        _ => {}
    }
}

fn lex(code: String) -> VecDeque<Token> {
    let mut queue: VecDeque<_> = code.chars().collect();
    let mut tokens: VecDeque<Token> = VecDeque::new();

    while queue.len() > 0 {
        let c = queue.pop_front();
        match c {
            Some('{') => tokens.push_back(Token::LeftBrace),
            _ => {},
        }
    }
    
    VecDeque::new()
}

fn parse(_tokens: VecDeque<Token>) -> Vec<Stmt> {
    Vec::new()
}

fn codegen(_ast: Vec<Stmt>) -> String {
    String::from("")
}
