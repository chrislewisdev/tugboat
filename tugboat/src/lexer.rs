use super::*;
use phf::phf_map;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenKind {
    // Single-characters
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Semicolon,
    Comma,
    Star,
    Plus,
    Minus,
    Exclamation,
    // One-or-two characters
    Slash,
    Equals,
    EqualsEquals,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    //Keywords
    Fn,
    Unsigned8,
    While,
    True,
    False,
    Halt,
    // Multi-character
    Identifier,
    Number,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub value: Option<u8>,
    pub line: u32,
}

use TokenKind::*;

static SINGLE_CHAR_TOKENS: phf::Map<char, TokenKind> = phf_map! {
    '{' => LeftBrace,
    '}' => RightBrace,
    '(' => LeftParen,
    ')' => RightParen,
    '[' => LeftBracket,
    ']' => RightBracket,
    ';' => Semicolon,
    '=' => Equals,
    ',' => Comma,
    '*' => Star,
    '!' => Exclamation,
    '>' => Greater,
    '<' => Less,
    '/' => Slash,
    '+' => Plus,
    '-' => Minus,
};

static KEYWORDS: phf::Map<&'static str, TokenKind> = phf_map! {
    "fn" => Fn,
    "u8" => Unsigned8,
    "true" => True,
    "false" => False,
    "while" => While,
    "halt" => Halt,
};

pub fn lex(code: String) -> (Vec<Token>, Vec<CompilationError>) {
    let mut queue: VecDeque<_> = code.chars().collect();
    let mut tokens: Vec<Token> = Vec::new();
    let mut errors: Vec<CompilationError> = Vec::new();
    let mut line = 1;

    let mut add = |kind: TokenKind, lexeme: String, value: Option<u8>, line: u32| {
        tokens.push(build_token(kind, lexeme, value, line))
    };
    let mut error = |msg: String, line: u32| {
        errors.push(CompilationError { msg, line });
    };

    while queue.len() > 0 {
        match queue.pop_front() {
            Some('=') if is_char('=', queue.get(0)) => {
                queue.pop_front();
                add(EqualsEquals, String::from("=="), None, line);
            }
            Some('>') if is_char('=', queue.get(0)) => {
                queue.pop_front();
                add(GreaterEqual, String::from(">="), None, line);
            }
            Some('<') if is_char('=', queue.get(0)) => {
                queue.pop_front();
                add(LessEqual, String::from("<="), None, line);
            }
            Some('/') if is_char('/', queue.get(0)) => {
                while !is_char('\n', queue.get(0)) {
                    queue.pop_front();
                }
            }
            Some(c) if SINGLE_CHAR_TOKENS.contains_key(&c) => {
                let kind = SINGLE_CHAR_TOKENS.get(&c).unwrap().clone();
                add(kind, String::from(c), None, line);
            }
            // TODO: Let's break these larger branches off into functions
            Some('\'') => {
                let mut literal = String::new();
                while !is_char('\'', queue.get(0)) {
                    literal.push(queue.pop_front().unwrap());
                }
                queue.pop_front();

                if literal.len() == 1 {
                    let character_literal = literal.chars().next().unwrap();
                    let number_literal = u8::try_from(u32::from(character_literal));
                    if let Ok(n) = number_literal {
                        add(Number, literal, Some(n), line);
                    } else {
                        let msg =
                            format!("Failed to convert character to u8: '{}'", character_literal);
                        error(msg, line);
                    }
                } else {
                    let msg = format!(
                        "Character literal should be exactly one character: '{}'",
                        literal
                    );
                    error(msg, line);
                }
            }
            Some(c @ '0'..='9') => {
                let mut literal = String::from(c);
                while is_digit(queue.get(0)) {
                    literal.push(queue.pop_front().unwrap());
                }

                let parse_result = literal.parse::<u8>();
                match parse_result {
                    Ok(value) => {
                        add(Number, literal, Some(value), line);
                    }
                    Err(err) => {
                        let msg = format!("Failed to parse literal: {}", err.to_string());
                        error(msg, line);
                    }
                }
            }
            Some(c @ 'a'..='z' | c @ 'A'..='Z') => {
                let mut identifier = String::from(c);
                while is_identifier(queue.get(0)) {
                    identifier.push(queue.pop_front().unwrap());
                }

                if KEYWORDS.contains_key(&identifier) {
                    let kind = KEYWORDS.get(&identifier).unwrap().clone();
                    add(kind, identifier, None, line);
                } else {
                    add(Identifier, identifier, None, line);
                }
            }
            Some('\n') => {
                line += 1;
            }
            Some(' ' | '\t' | '\r') => {}
            Some(c) => {
                error(format!("Unexpected character: {}", c), line);
            }
            None => {}
        }
    }

    (tokens, errors)
}

fn is_char(target: char, subject: Option<&char>) -> bool {
    match subject {
        Some(c) if *c == target => true,
        _ => false,
    }
}

fn is_digit(c: Option<&char>) -> bool {
    match c {
        Some('0'..='9') => true,
        _ => false,
    }
}

fn is_identifier(c: Option<&char>) -> bool {
    match c {
        Some('A'..='Z') => true,
        Some('a'..='z') => true,
        Some('0'..='9') => true,
        Some('_') => true,
        _ => false,
    }
}

fn build_token(kind: TokenKind, lexeme: String, value: Option<u8>, line: u32) -> Token {
    Token {
        kind,
        lexeme,
        value,
        line,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn token(kind: TokenKind, lexeme: &str, value: Option<u8>, line: u32) -> Token {
        build_token(kind, lexeme.to_string(), value, line)
    }

    fn error(msg: &str, line: u32) -> CompilationError {
        CompilationError {
            msg: msg.to_string(),
            line,
        }
    }

    #[test]
    fn lex_single_char() {
        let (result, _) = lex(String::from("{}();"));
        assert_eq!(
            result,
            vec![
                token(LeftBrace, "{", None, 1),
                token(RightBrace, "}", None, 1),
                token(LeftParen, "(", None, 1),
                token(RightParen, ")", None, 1),
                token(Semicolon, ";", None, 1),
            ]
        );
    }

    #[test]
    fn lex_keywords() {
        let (result, _) = lex(String::from("fn u8"));
        assert_eq!(
            result,
            vec![token(Fn, "fn", None, 1), token(Unsigned8, "u8", None, 1),]
        );
    }

    #[test]
    fn lex_identifiers() {
        let (result, _) = lex(String::from("myVar something"));
        assert_eq!(
            result,
            vec![
                token(Identifier, "myVar", None, 1),
                token(Identifier, "something", None, 1)
            ]
        );
    }

    #[test]
    fn lex_basic_script() {
        let (result, _) = lex(String::from(
            "u8 variable;\nfn main() {\nvariable = 5;\n}\n",
        ));
        assert_eq!(
            result,
            vec![
                token(Unsigned8, "u8", None, 1),
                token(Identifier, "variable", None, 1),
                token(Semicolon, ";", None, 1),
                token(Fn, "fn", None, 2),
                token(Identifier, "main", None, 2),
                token(LeftParen, "(", None, 2),
                token(RightParen, ")", None, 2),
                token(LeftBrace, "{", None, 2),
                token(Identifier, "variable", None, 3),
                token(Equals, "=", None, 3),
                token(Number, "5", Some(5), 3),
                token(Semicolon, ";", None, 3),
                token(RightBrace, "}", None, 4)
            ]
        );
    }

    #[test]
    fn lex_big_number() {
        let (_, errors) = lex(String::from("65536"));
        assert_eq!(
            errors,
            vec![error(
                "Failed to parse literal: number too large to fit in target type",
                1
            )]
        );
    }

    #[test]
    fn lex_equals_equals() {
        let (result, _) = lex(String::from("== = =="));
        assert_eq!(
            result,
            vec![
                token(EqualsEquals, "==", None, 1),
                token(Equals, "=", None, 1),
                token(EqualsEquals, "==", None, 1)
            ]
        );
    }

    #[test]
    fn lex_comments_and_comparisons() {
        let (result, _) = lex(String::from("// This is a comment\n> >= < <="));
        assert_eq!(
            result,
            vec![
                token(Greater, ">", None, 2),
                token(GreaterEqual, ">=", None, 2),
                token(Less, "<", None, 2),
                token(LessEqual, "<=", None, 2),
            ]
        );
    }

    #[test]
    fn lex_character_literals() {
        let (result, _) = lex(String::from("'a' '0' 'G'"));
        assert_eq!(
            result,
            vec![
                token(Number, "a", Some(97), 1),
                token(Number, "0", Some(48), 1),
                token(Number, "G", Some(71), 1),
            ]
        );
    }

    #[test]
    fn lex_character_literal_errors() {
        let (_, errors) = lex(String::from("'aaa' ''"));
        assert_eq!(
            errors,
            vec![
                error(
                    "Character literal should be exactly one character: 'aaa'",
                    1
                ),
                error("Character literal should be exactly one character: ''", 1),
            ]
        );
    }
}
