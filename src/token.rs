use std::fmt::Display;

pub enum TokenKind {
    /* Delimiters */
    LeftParen,  // (
    RightParen, // )
    LeftBrace,  // {
    RightBrace, // }

    /* Characters */
    Comma,    // ,
    Dot,      // .
    Minus,    // -
    Plus,     // +
    Question, // ?
    Star,     // *

    /* Comparison */
    EqualEqual,   // ==
    NotEqual,     // =/=
    Greater,      // >
    GreaterEqual, // >=
    Less,         // <
    LessEqual,    // <=

    /* Literals */
    Identifier, // x, s, etc.
    String,     // "hahaha"
    Number,     // 10101010

    /* Logic */
    Aint, // a'int
    And,  // and
    Or,   // or

    /* Control Flow */
    If,        // if
    Nyoom,     // nyoom
    Otherwise, // otherwise
    Provided,  // provided

    /* Booleans */
    Yes, // yes
    No,  // no

    /* Functions */
    Gimme,  // gimme
    Minion, // minion

    /* Variables */
    Hey, // hey
    Is,  // =

    /* Printing */
    Whats, // what's

    /* End of File */
    EOF,
}

pub struct Token<'a> {
    kind: TokenKind,
    lexeme: &'a str,
    literal: (),
    line_number: usize,
}

impl Token<'_> {
    pub fn new(kind: TokenKind, lexeme: &str, literal: (), line_number: usize) -> Token {
        Token {
            kind,
            lexeme,
            literal,
            line_number,
        }
    }
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.lexeme)
    }
}
