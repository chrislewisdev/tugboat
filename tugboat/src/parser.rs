use super::*;
use lexer::TokenKind;
use lexer::TokenKind::*;

pub fn parse(tokens: Vec<Token>) -> (Vec<Declaration>, Vec<CompilationError>) {
    let mut queue: VecDeque<_> = tokens.into_iter().collect();
    let mut declarations: Vec<Declaration> = Vec::new();
    let mut errors: Vec<CompilationError> = Vec::new();

    while queue.len() > 0 {
        let result = declaration(&mut queue);
        if let Ok(dec) = result {
            declarations.push(dec);
        } else if let Err(err) = result {
            // TODO: Consider what else to do in error case e.g. synchronise here
            errors.push(err);
        }
    }

    (declarations, errors)
}

fn error(line: u32, msg: &'static str) -> CompilationError {
    CompilationError {
        line,
        msg: msg.to_string(),
    }
}

fn peek(queue: &VecDeque<Token>) -> Result<&Token, CompilationError> {
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

fn declaration(queue: &mut VecDeque<Token>) -> Result<Declaration, CompilationError> {
    let token = next(queue)?;
    match token.kind {
        Fn => function(queue),
        Unsigned8 => variable(queue),
        _ => Err(error(token.line, "Unsupported top-level statement.")),
    }
}

fn function(queue: &mut VecDeque<Token>) -> Result<Declaration, CompilationError> {
    let name = expect(queue, Identifier, "Expected identifier after 'fn'.")?;

    let mut arguments: Vec<Token> = Vec::new();
    expect(queue, LeftParen, "Expected '(' after function name.")?;
    while peek(queue)?.kind != RightParen {
        arguments.push(expect(queue, Identifier, "Expected parameter name.")?);
        next_if(queue, Comma)?;
    }
    expect(queue, RightParen, "Expected ')' after argument list.")?;

    expect(queue, LeftBrace, "Expected '{' after function declaration.")?;
    let body = block(queue)?;

    Ok(Declaration::Function {
        name,
        arguments,
        body,
    })
}

fn variable(queue: &mut VecDeque<Token>) -> Result<Declaration, CompilationError> {
    let mut size = 1;
    if peek(queue)?.kind == LeftBracket {
        expect(queue, LeftBracket, "Expected '[' beginning array definition.")?;
        let size_token = expect(queue, Number, "Expected array size specifier.")?;
        expect(queue, RightBracket, "Expected ']' ending array definition.")?;

        size = get_value(size_token)?;
    }

    let name = expect(queue, Identifier, "Expected variable name.")?;
    expect(queue, Semicolon, "Expected ';' after variable declaration.")?;

    Ok(Declaration::Variable { name, size })
}

fn statement(queue: &mut VecDeque<Token>) -> Result<Stmt, CompilationError> {
    let stmt: Result<Stmt, CompilationError> = match peek(queue)?.kind {
        TokenKind::Halt => {
            next(queue)?;
            expect(queue, Semicolon, "Expected ';' after halt.")?;
            Ok(Stmt::Halt)
        }
        TokenKind::While => while_loop(queue),
        _ => expression_statement(queue),
    };

    stmt
}

fn while_loop(queue: &mut VecDeque<Token>) -> Result<Stmt, CompilationError> {
    next(queue)?; // Consume the opening keyword
    expect(queue, LeftParen, "Expected '(' after while.")?;

    let condition = expression(queue)?;

    expect(queue, RightParen, "Expected ')' after while condition.")?;
    expect(queue, LeftBrace, "Expected '{' at beginning of while body.")?;

    let body = block(queue)?;

    Ok(Stmt::While { condition, body })
}

fn expression_statement(queue: &mut VecDeque<Token>) -> Result<Stmt, CompilationError> {
    let expr = expression(queue)?;

    if peek(queue)?.kind == Equals {
        let equals = next(queue)?;
        let value = expression(queue)?;
        expect(queue, Semicolon, "Expected ';' after statement.")?;
        if let Expr::Variable { name } = expr {
            Ok(Stmt::Assign {
                target: name,
                value,
            })
        } else {
            Err(error(equals.line, "Cannot assign to non-variable."))
        }
    } else {
        expect(queue, Semicolon, "Expected ';' after statement.")?;
        Ok(Stmt::Expression { expr })
    }
}

fn block(queue: &mut VecDeque<Token>) -> Result<Vec<Stmt>, CompilationError> {
    let mut statements: Vec<Stmt> = Vec::new();

    // We expect that the opening '{' has been consumed before calling this
    while peek(queue)?.kind != RightBrace {
        statements.push(statement(queue)?);
    }

    expect(queue, RightBrace, "Expected '}' at end of block.")?;

    Ok(statements)
}

fn expression(queue: &mut VecDeque<Token>) -> Result<Expr, CompilationError> {
    let token = next(queue)?;
    let expr = match token.kind {
        True => Ok(Expr::Literal { value: 1 }),
        False => Ok(Expr::Literal { value: 0 }),
        Number => Ok(Expr::Literal {
            value: get_value(token)?,
        }),
        Identifier => {
            if peek(queue)?.kind == LeftBracket {
                expect(queue, LeftBracket, "Expected '[' beginning index expression.")?;
                let index = expression(queue)?;
                expect(queue, RightBracket, "Expected ']' ending index expression.")?;
                Ok(Expr::Indexed { name: token, index: Box::new(index) })
            } else {
                Ok(Expr::Variable { name: token })
            }
        }
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
        let mut tokens: VecDeque<_> = vec![token(Identifier), token(Semicolon)].into();
        let result = variable(&mut tokens).unwrap();
        assert!(matches!(result, Declaration::Variable { .. }));
    }

    #[test]
    fn variable_err() {
        let mut tokens: VecDeque<_> = vec![token(Unsigned8), token(Identifier)].into();
        let result = variable(&mut tokens);
        assert!(matches!(result, Err { .. }));
    }

    #[test]
    fn expression_indexed() {
        let mut tokens: VecDeque<_> = vec![token(Identifier), token(LeftBracket), token(Identifier), token(RightBracket)].into();
        let result = expression(&mut tokens).unwrap();
        assert!(matches!(result, Expr::Indexed { .. }))
    }

    #[test]
    fn parse_basic() {
        let (tokens, _) = lexer::lex(String::from(
            "u8 variable;\nfn main(a, b, c) {\nvariable = 5;\n}\n",
        ));
        let (ast, errors) = parse(tokens);

        assert_eq!(errors, vec![]);
        assert!(matches!(
            ast[..],
            [Declaration::Variable { .. }, Declaration::Function { .. }]
        ));
        // TODO: Assert that function body/arguments are sane
    }

    #[test]
    fn parse_while() {
        let (tokens, _) = lexer::lex(String::from("while (true) { halt; }"));
        let mut queue: VecDeque<Token> = tokens.into_iter().collect();
        let _stmt = while_loop(&mut queue).unwrap();

        //TODO: Figure out some asserts here
    }
}
