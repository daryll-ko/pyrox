use crate::token::Token;
use crate::token::TokenKind::*;

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
    pub fn scan_for_token(&mut self) -> () {}
    pub fn scan_for_tokens(&mut self) -> Vec<Token> {
        let mut tokens = vec![];

        while !self.at_end() {
            self.start = self.current;
            self.scan_for_token();
        }

        tokens.push(Token::new(EOF, "", (), self.line));
        tokens
    }
    pub fn at_end(&self) -> bool {
        self.current >= self.code.len()
    }
}
