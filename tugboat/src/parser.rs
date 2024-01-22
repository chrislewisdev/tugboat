use super::Expr::*;
use super::Stmt::*;
use super::*;
use lexer::TokenKind;
use lexer::TokenKind::*;

pub fn parse(tokens: Vec<Token>) -> (Vec<Stmt>, Vec<CompilationError>) {
    let mut queue: VecDeque<_> = tokens.into_iter().collect();
    let mut statements: Vec<Stmt> = Vec::new();
    let mut errors: Vec<CompilationError> = Vec::new();

    while queue.len() > 0 {
        let result = declaration(&mut queue);
        if let Ok(stmt) = result {
            statements.push(stmt);
        } else if let Err(err) = result {
            // Consider what else to do in error case e.g. synchronise here
            errors.push(err);
        }
    }

    (statements, errors)
}

fn error(line: u32, msg: &'static str) -> CompilationError {
    CompilationError {
        line,
        msg: msg.to_string(),
    }
}

fn peek(queue: &mut VecDeque<Token>) -> Result<&Token, CompilationError> {
    queue
        .get(0)
        .ok_or(error(0, "Expected a token in the parse queue."))
}

fn next(queue: &mut VecDeque<Token>) -> Result<Token, CompilationError> {
    queue
        .pop_front()
        .ok_or(error(0, "Expected a token in the parse queue."))
}

fn next_if(queue: &mut VecDeque<Token>, kind: TokenKind) -> Result<bool, CompilationError> {
    if peek(queue)?.kind == kind {
        next(queue)?;
        Ok(true)
    } else {
        Ok(false)
    }
}

fn expect(
    queue: &mut VecDeque<Token>,
    kind: TokenKind,
    msg: &'static str,
) -> Result<Token, CompilationError> {
    let token = next(queue)?;

    if token.kind == kind {
        Ok(token)
    } else {
        Err(error(token.line, msg))
    }
}

fn declaration(queue: &mut VecDeque<Token>) -> Result<Stmt, CompilationError> {
    let stmt = match peek(queue)?.kind {
        Fn => function(queue),
        Unsigned8 => variable(queue),
        _ => statement(queue),
    };

    stmt
}

fn function(queue: &mut VecDeque<Token>) -> Result<Stmt, CompilationError> {
    expect(queue, Fn, "Expected 'fn' keyword.")?;
    let name = expect(queue, Identifier, "Expected identifier after 'fn'.")?;

    let mut arguments: Vec<Token> = Vec::new();
    expect(queue, LeftParen, "Expected '(' after function name.")?;
    while peek(queue)?.kind != RightParen {
        arguments.push(expect(queue, Identifier, "Expected parameter name.")?);
        next_if(queue, Comma)?;
    }
    expect(queue, RightParen, "Expected ')' after argument list.")?;

    // TODO: Parse body

    Ok(Stmt::Halt)
}

fn variable(queue: &mut VecDeque<Token>) -> Result<Stmt, CompilationError> {
    expect(queue, Unsigned8, "Expected 'u8' keyword.")?;
    let name = expect(queue, Identifier, "Expected variable name.")?;
    expect(queue, Semicolon, "Expected ';' after variable declaration.")?;

    Ok(Stmt::Variable { name })
}

fn statement(queue: &mut VecDeque<Token>) -> Result<Stmt, CompilationError> {
    let stmt: Result<Stmt, CompilationError> = match peek(queue)?.kind {
        TokenKind::Halt => {
            next(queue)?;
            Ok(Stmt::Halt)
        }
        _ => expression_statement(queue),
    };

    stmt
}

fn expression_statement(queue: &mut VecDeque<Token>) -> Result<Stmt, CompilationError> {
    let expr = expression(queue)?;

    if peek(queue)?.kind == Equals {
        let equals = next(queue)?;
        let value = expression(queue)?;
        if let Expr::Variable { name } = expr {
            Ok(Assign {
                target: name,
                value,
            })
        } else {
            Err(error(equals.line, "Cannot assign to non-variable."))
        }
    } else {
        expect(queue, Semicolon, "Expected ';' after statement.")?;
        Ok(Expression { expr })
    }
}

fn expression(queue: &mut VecDeque<Token>) -> Result<Expr, CompilationError> {
    let token = next(queue)?;
    let expr = match token.kind {
        Number => Ok(Literal {
            value: get_value(token)?,
        }),
        Identifier => Ok(Expr::Variable { name: token }),
        _ => Err(error(
            token.line,
            "Expected number or identifier in expression.",
        )),
    };

    expr
}

fn get_value(token: Token) -> Result<u8, CompilationError> {
    token
        .value
        .ok_or(error(token.line, "Expected a value in number literal."))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn token(kind: TokenKind) -> Token {
        Token {
            kind,
            lexeme: String::from(""),
            value: None,
            line: 0,
        }
    }

    #[test]
    fn variable_ok() {
        let mut tokens: VecDeque<_> =
            vec![token(Unsigned8), token(Identifier), token(Semicolon)].into();
        let result = variable(&mut tokens).unwrap();
        assert!(matches!(result, Stmt::Variable { .. }));
    }

    #[test]
    fn variable_err() {
        let mut tokens: VecDeque<_> = vec![token(Unsigned8), token(Identifier)].into();
        let result = variable(&mut tokens);
        assert!(matches!(result, Err { .. }));
    }
}
