mod lexer;

use lexer::Token;
use std::{collections::VecDeque, fs};

enum Stmt {}
enum _Expr {}

pub fn compile(name: &str, contents: String) {
    let tokens = lexer::lex(contents);
    let ast = parse(tokens);
    // *Probably* going to need some more steps here.
    let asm = codegen(ast);

    let result = fs::write(format!("{}.asm", name), asm);
    match result {
        Err(err) => println!("Failed to write assembly: {}", err),
        _ => {}
    }
}

fn parse(_tokens: Vec<Token>) -> Vec<Stmt> {
    Vec::new()
}

fn codegen(_ast: Vec<Stmt>) -> String {
    String::from("")
}
