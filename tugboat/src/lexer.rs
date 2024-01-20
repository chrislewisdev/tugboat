use super::*;
use phf::phf_map;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenKind {
    // Single-characters
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    Semicolon,
    // One-or-single characters
    Equals,
    //Keywords
    Fn,
    Unsigned8,
    // Multi-character
    Identifier,
    Literal,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    kind: TokenKind,
    lexeme: String,
    value: Option<u8>,
    line: u32,
}

use TokenKind::*;

static SINGLE_CHAR_TOKENS: phf::Map<char, TokenKind> = phf_map! {
    '{' => LeftBrace,
    '}' => RightBrace,
    '(' => LeftParen,
    ')' => RightParen,
    ';' => Semicolon,
    '=' => Equals,
};

static KEYWORDS: phf::Map<&'static str, TokenKind> = phf_map! {
    "fn" => Fn,
    "u8" => Unsigned8,
};

pub fn lex(code: String) -> Vec<Token> {
    let mut queue: VecDeque<_> = code.chars().collect();
    let mut tokens: Vec<Token> = Vec::new();
    let mut line = 1;

    let mut add = |kind: TokenKind, lexeme: String, value: Option<u8>, line: u32| {
        tokens.push(build_token(kind, lexeme, value, line))
    };

    while queue.len() > 0 {
        let next_char = queue.pop_front();
        match next_char {
            Some(c) if SINGLE_CHAR_TOKENS.contains_key(&c) => {
                add(
                    SINGLE_CHAR_TOKENS.get(&c).unwrap().clone(),
                    String::from(c),
                    None,
                    line,
                );
            }
            Some(c @ '0'..='9') => {
                let mut literal = String::from(c);
                while match queue.get(0) {
                    Some(nc) if is_digit(nc) => true,
                    _ => false,
                } {
                    literal.push(queue.pop_front().unwrap());
                }

                let value: u8 = literal.parse().unwrap();
                add(Literal, literal, Some(value), line);
            },
            Some(c @ 'a'..='z' | c @ 'A'..='Z') => {
                // Build up the rest of our identifier
                let mut identifier = String::from(c);
                while match queue.get(0) {
                    Some(nc) if is_identifier(nc) => true,
                    _ => false,
                } {
                    identifier.push(queue.pop_front().unwrap());
                }

                if KEYWORDS.contains_key(&identifier) {
                    add(
                        KEYWORDS.get(&identifier).unwrap().clone(),
                        identifier,
                        None,
                        line,
                    );
                } else {
                    add(Identifier, identifier, None, line);
                }
            }
            Some('\n') => {
                line += 1;
            }
            _ => {}
        }
    }

    tokens
}

fn is_digit(c: &char) -> bool {
    match c {
        '0'..='9' => true,
        _ => false,
    }
}

fn is_identifier(c: &char) -> bool {
    match c {
        'A'..='Z' => true,
        'a'..='z' => true,
        '0'..='9' => true,
        '_' => true,
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

    #[test]
    fn lex_single_char() {
        let result = lex(String::from("{}();"));
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
        let result = lex(String::from("fn u8"));
        assert_eq!(
            result,
            vec![token(Fn, "fn", None, 1), token(Unsigned8, "u8", None, 1),]
        );
    }

    #[test]
    fn lex_identifiers() {
        let result = lex(String::from("myVar something"));
        assert_eq!(
            result,
            vec![token(Identifier, "myVar", None, 1), token(Identifier, "something", None, 1)]
        );
    }

    #[test]
    fn lex_basic_script() {
        let result = lex(String::from("u8 variable;\nfn main() {\nvariable = 5;\n}\n"));
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
                token(Literal, "5", Some(5), 3),
                token(Semicolon, ";", None, 3),
                token(RightBrace, "}", None, 4)
            ]
        );
    }
}
