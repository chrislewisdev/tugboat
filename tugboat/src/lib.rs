mod analysis;
mod codegen;
mod lexer;
mod parser;

use lexer::Token;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Declaration {
    Variable {
        name: Token,
        size: u8,
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
    Indexed { name: Token, index: Box<Expr> },
}

#[derive(Debug, PartialEq, Eq)]
pub struct CompilationError {
    pub msg: String,
    pub line: u32,
}

pub fn compile(contents: String) -> Result<String, Vec<CompilationError>> {
    let mut errors: Vec<CompilationError> = Vec::new();
    let (tokens, lexer_errors) = lexer::lex(contents);
    let (ast, parser_errors) = parser::parse(tokens);

    errors.extend(lexer_errors);
    errors.extend(parser_errors);
    if errors.len() > 0 {
        return Err(errors);
    }

    let directory = analysis::generate_directory(&ast);

    Ok(codegen::gen(ast, &directory)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn error(msg: &str, line: u32) -> CompilationError {
        CompilationError {
            msg: msg.to_string(),
            line,
        }
    }

    #[test]
    fn error_assign_to_function() {
        let src = String::from("fn myFunction(){} fn main() { myFunction = 5; }");
        let errors = compile(src).expect_err("Expected compilation errors from bad script!");
        assert_eq!(errors, vec![error("Cannot assign to function", 1)]);
    }

    #[test]
    fn error_undefined_variable() {
        let src = String::from("fn main() { myVariable = 5; }");
        let errors = compile(src).expect_err("Expected compilation errors from bad script!");
        assert_eq!(errors, vec![error("Undefined variable: myVariable", 1)]);
    }

    #[test]
    fn error_assign_to_literal() {
        let src = String::from("fn main() { 1 = 5; }");
        let errors = compile(src).expect_err("Expected compilation errors from bad script!");
        assert_eq!(
            errors,
            vec![
                error("Cannot assign to non-variable.", 1),
                error("Unsupported top-level statement.", 1)
            ]
        );
    }
}
