use super::*;
use lexer::TokenKind::*;
use super::Stmt::*;

pub fn parse(tokens: Vec<Token>) -> (Vec<Stmt>, Vec<CompilationError>) {
    let mut queue: VecDeque<_> = tokens.into_iter().collect();
    let mut statements: Vec<Stmt> = Vec::new();
    let mut errors: Vec<CompilationError> = Vec::new();

    while queue.len() > 0 {
        statements.push(declaration(&mut queue));
    }

    (statements, errors)
}

fn declaration(queue: &mut VecDeque<Token>) -> Stmt {
    //let stmt = match queue.get(0)? {
    //    Token { kind: Fn, .. } => function(queue),
    //    _ => statement(queue),
    //};

    //stmt
    Expression
}

fn function(queue: &mut VecDeque<Token>) -> Stmt {
    Expression
}

fn statement(queue: &mut VecDeque<Token>) -> Stmt {
   Expression 
}
