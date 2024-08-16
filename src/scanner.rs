use crate::error::error;
use crate::token::Token;
use crate::token::{TokenKind, TokenKind::*};

pub struct Scanner<'a> {
    code: &'a str,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner<'_> {
    pub fn new(code: &str) -> Scanner {
        Scanner {
            code,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_for_tokens(&mut self) -> Vec<Token> {
        let mut tokens = vec![];

        while !self.at_end() {
            self.start = self.current;
            if let Some(token) = self.scan_for_token() {
                tokens.push(token)
            }
        }

        tokens.push(Token::new(EOF, "".to_string(), (), self.line));
        tokens
    }

    fn at_end(&self) -> bool {
        self.current >= self.code.len()
    }

    fn scan_for_token(&mut self) -> Option<Token> {
        match self.peek_and_step() {
            '(' => self.form_token(LeftParen),
            ')' => self.form_token(RightParen),
            '{' => self.form_token(LeftBrace),
            '}' => self.form_token(RightBrace),
            ',' => self.form_token(Comma),
            '.' => self.form_token(Dot),
            '-' => self.form_token(Minus),
            '+' => self.form_token(Plus),
            '?' => self.form_token(Question),
            '*' => self.form_token(Star),
            '=' => {
                if self.probe('=') {
                    self.form_token(EqualEqual)
                } else {
                    self.form_token(Is)
                }
            }
            '>' => {
                if self.probe('=') {
                    self.form_token(GreaterEqual)
                } else {
                    self.form_token(Greater)
                }
            }
            '<' => {
                if self.probe('=') {
                    self.form_token(LessEqual)
                } else {
                    self.form_token(Less)
                }
            }
            ' ' | '\r' | '\t' => None,
            '\n' => {
                self.line += 1;
                None
            }
            other => {
                error(self.line, &format!("invalid character '{other}'"));
                None
            }
        }
    }

    fn peek(&mut self) -> char {
        if self.at_end() {
            '\0'
        } else {
            self.code.as_bytes()[self.current] as char
        }
    }

    fn peek_and_step(&mut self) -> char {
        let res = self.peek();
        self.current += 1;
        res
    }

    fn probe(&mut self, expected: char) -> bool {
        if self.peek() != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn form_token(&self, kind: TokenKind) -> Option<Token> {
        Some(Token::new(
            kind,
            self.current_lexeme().to_string(),
            (),
            self.line,
        ))
    }

    fn form_token_with_literal(&self, kind: TokenKind, literal: ()) -> Option<Token> {
        Some(Token::new(
            kind,
            self.current_lexeme().to_string(),
            literal,
            self.line,
        ))
    }

    fn current_lexeme(&self) -> &str {
        &self.code[self.start..self.current]
    }
}
