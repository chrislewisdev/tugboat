mod lexer;

use lexer::Token;
use std::collections::VecDeque;

pub enum Stmt {
    Function {
        name: Token,
        arguments: Vec<Token>,
        body: Vec<Stmt>,
    },
    Variable {
        name: Token,
    },
    Halt,
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
    Assign,
}
pub enum Expr {
    Literal,
}

#[derive(Debug, PartialEq, Eq)]
pub struct CompilationError {
    pub msg: String,
    pub line: u32,
}

pub fn compile(_name: &str, contents: String) -> Vec<CompilationError> {
    let mut errors: Vec<CompilationError> = Vec::new();
    let (tokens, lexer_errors) = lexer::lex(contents);
    let (ast, parser_errors) = parse(tokens);

    errors.extend(lexer_errors);
    errors.extend(parser_errors);
    if errors.len() > 0 {
        return errors;
    }

    // *Probably* going to need some more steps here.
    let _asm = codegen(ast);

    //let result = fs::write(format!("{}.asm", name), asm);
    //match result {
    //    Err(err) => println!("Failed to write assembly: {}", err),
    //    _ => {}
    //}

    errors
}

fn parse(_tokens: Vec<Token>) -> (Vec<Stmt>, Vec<CompilationError>) {
    (Vec::new(), Vec::new())
}

fn codegen(_ast: Vec<Stmt>) -> String {
    String::from("")
}
