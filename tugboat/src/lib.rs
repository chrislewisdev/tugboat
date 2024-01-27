mod codegen;
mod lexer;
mod parser;

use lexer::Token;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Declaration {
    Variable {
        name: Token,
    },
    Function {
        name: Token,
        arguments: Vec<Token>,
        body: Vec<Stmt>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Stmt {
    Halt,
    While { condition: Expr, body: Vec<Stmt> },
    Assign { target: Token, value: Expr },
    Expression { expr: Expr },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expr {
    Literal { value: u8 },
    Variable { name: Token },
}

#[derive(Debug, PartialEq, Eq)]
pub struct CompilationError {
    pub msg: String,
    pub line: u32,
}

pub fn compile(_name: &str, contents: String) -> Result<String, Vec<CompilationError>> {
    let mut errors: Vec<CompilationError> = Vec::new();
    let (tokens, lexer_errors) = lexer::lex(contents);
    let (ast, parser_errors) = parser::parse(tokens);

    errors.extend(lexer_errors);
    errors.extend(parser_errors);
    if errors.len() > 0 {
        return Err(errors);
    }

    // *Probably* going to need some more steps here.

    Ok(codegen::gen(ast)?)
}
