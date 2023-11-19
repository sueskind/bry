use std::iter::Peekable;
use std::str::CharIndices;

use crate::lexer::token::{Token, TokenType};

mod token;

pub struct Lexer {
    line: usize,
    line_start_column: usize,
    column: usize,
}

impl Lexer {
    pub fn default() -> Self {
        Self { line: 0, line_start_column: 0, column: 0 }
    }

    fn create_token(&self, typ: TokenType) -> Token {
        Token { typ, line: self.line, column: self.column }
    }

    fn scan_number(
        &self, first_c: char, source_iter: &mut Peekable<CharIndices>,
    ) -> Token {
        let mut lexeme = String::from(first_c);
        while let Some((_, c)) = source_iter.peek() {
            if !c.is_numeric() {
                break;
            }

            let (_, c) = source_iter.next().unwrap();  // unwrap is fine because of peek above
            lexeme.push(c);
        }

        let num = lexeme.parse::<u32>().unwrap();  // parse should always work here
        self.create_token(TokenType::Number(num))
    }

    fn scan_identifier_or_keyword(
        &self, first_c: char, source_iter: &mut Peekable<CharIndices>,
    ) -> Token {
        let mut lexeme = String::from(first_c);
        while let Some((_, c)) = source_iter.peek() {
            if !c.is_alphanumeric() {
                break;
            }

            let (_, c) = source_iter.next().unwrap();  // unwrap is fine because of peek above
            lexeme.push(c);
        }

        match lexeme.as_str() {
            "def" => self.create_token(TokenType::Def),
            // add keywords here

            _ => self.create_token(TokenType::Identifier(lexeme))
        }
    }

    pub fn tokenize(mut self, source: &str) -> Vec<Token> {  // move self here to forbid calling twice
        let mut tokens: Vec<Token> = Vec::new();

        let mut source_iter = source.char_indices().peekable();

        while let Some((i, c)) = source_iter.next() {
            self.column = i - self.line_start_column;

            if c.is_whitespace() {
                if c == '\n' {
                    self.line += 1;
                    self.line_start_column = i + 1;
                }
                continue;
            }

            let token = match c {
                // Single-character
                '(' => self.create_token(TokenType::LeftParen),
                ')' => self.create_token(TokenType::RightParen),
                '{' => self.create_token(TokenType::LeftBrace),
                '}' => self.create_token(TokenType::RightBrace),
                '-' => self.create_token(TokenType::Minus),
                '+' => self.create_token(TokenType::Plus),
                '*' => self.create_token(TokenType::Star),
                '/' => self.create_token(TokenType::Slash),
                ':' => self.create_token(TokenType::Colon),
                ';' => self.create_token(TokenType::Semicolon),

                // Single- or Double-character
                '!' => {
                    match source_iter.peek() {
                        Some((_, '=')) => {
                            source_iter.next();
                            self.create_token(TokenType::ExclMarkEqual)
                        }
                        _ => self.create_token(TokenType::ExclMark),
                    }
                }
                '=' => {
                    match source_iter.peek() {
                        Some((_, '=')) => {
                            source_iter.next();
                            self.create_token(TokenType::EqualEqual)
                        }
                        _ => self.create_token(TokenType::Equal),
                    }
                }
                '>' => {
                    match source_iter.peek() {
                        Some((_, '=')) => {
                            source_iter.next();
                            self.create_token(TokenType::GreaterEqual)
                        }
                        _ => self.create_token(TokenType::Greater),
                    }
                }
                '<' => {
                    match source_iter.peek() {
                        Some((_, '=')) => {
                            source_iter.next();
                            self.create_token(TokenType::LessEqual)
                        }
                        _ => self.create_token(TokenType::Less),
                    }
                }

                // Longer lexemes
                c if c.is_numeric() => self.scan_number(c, &mut source_iter),
                c if c.is_alphabetic() => self.scan_identifier_or_keyword(c, &mut source_iter),

                _ => todo!()
            };
            tokens.push(token);
        }

        tokens
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::token::TokenType::*;

    use super::*;

    #[test]
    fn test_single_tokens() {
        let token_types = [
            LeftParen, RightParen, LeftBrace, RightBrace, Minus, Plus, Star, Slash, Colon,
            Semicolon, ExclMark, ExclMarkEqual, Equal, EqualEqual, Greater, GreaterEqual,
            Less, LessEqual, Def, Identifier(String::from("asd")), Number(5),
        ];
        let lexemes = [
            "(", ")", "{", "}", "-", "+", "*", "/", ":",
            ";", "!", "!=", "=", "==", ">", ">=",
            "<", "<=", "def", "asd", "5",
        ];

        for (token_type, lexeme) in token_types.into_iter().zip(lexemes.into_iter()) {
            let lexer = Lexer::default();
            let expected = Token { typ: token_type, line: 0, column: 0 };
            let actual = lexer.tokenize(lexeme);

            assert_eq!(actual.len(), 1);
            assert_eq!(&expected, actual.first().unwrap());
        }
    }

    #[test]
    fn test_whitespace() {
        let source = "           \n\n  +++  +\n+ +  ";
        let expected = vec![
            Token { typ: Plus, line: 2, column: 2 },
            Token { typ: Plus, line: 2, column: 3 },
            Token { typ: Plus, line: 2, column: 4 },
            Token { typ: Plus, line: 2, column: 7 },
            Token { typ: Plus, line: 3, column: 0 },
            Token { typ: Plus, line: 3, column: 2 },
        ];
        let actual = Lexer::default().tokenize(source);

        assert_eq!(expected, actual);
    }
}
