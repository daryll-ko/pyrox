use std::fmt::Display;

#[derive(Debug)]
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

pub struct Token {
    kind: TokenKind,
    lexeme: String,
    literal: (),
    line_number: usize,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, literal: (), line_number: usize) -> Token {
        Token {
            kind,
            lexeme,
            literal,
            line_number,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.kind, self.lexeme)
    }
}
