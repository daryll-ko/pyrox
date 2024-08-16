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

    /* control flow */
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
}

pub struct Token<'a> {
    kind: TokenKind,
    lexeme: &'a str,
    literal: (),
    line_number: u32,
}

impl Token<'_> {
    pub fn new(kind: TokenKind, lexeme: &str, literal: (), line_number: u32) -> Token {
        Token {
            kind,
            lexeme,
            literal,
            line_number,
        }
    }
}
