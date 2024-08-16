use crate::token::Token;

pub struct Scanner<'a> {
    pub code: &'a str,
}

impl Scanner<'_> {
    pub fn new(code: &str) -> Scanner {
        Scanner { code }
    }
    pub fn scan_for_tokens(&self) -> Vec<Token> {
        vec![]
    }
}
