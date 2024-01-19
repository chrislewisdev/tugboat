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
            Some('\n') => {
                line += 1;
            }
            Some(c @ 'A'..='z') => {}
            _ => {}
        }
    }

    tokens
}

fn is_alpha(c: char) -> bool {
    match c {
        'A'..='z' => true,
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
}
