use std::{str::Chars, iter::Peekable}

// cf. include/llvm/MC/MCParser/MCAsmLexer.h#class AsmToken
enum TokenKind {
    // Markers
    Eof, Error,

    // String values
    Identifier,
    String,

    // Integer values
    Integer,
    BigNum,

    // Real values
    Real,

    // Comments
    Comment,
    HashDirective,

    // No-value
    EndOfStatement,
    Colon,
    Space,
    Plus, Minus,
    Tidle,
    Slash, BackSlash,       // '/', '\'
    LParen, RParen,         // '(', ')'
    LBrac, RBrac,           // '[', ']'
    LCurly, RCurly,         // '{', '}'

    Star, Dot, Comma, Dollar,
    Equal, EqualEqual,

    Pipe, PipePipe,         // '|', '||'
    Caret,                  // '^'
    Amp, AmpAmp,            // '&', '&&'
    Exclaim, ExclaimEqual,  // '!', '!='
    Percent, Hash,          // '%', '#'

    Less, LessEqual, LessLess, LessGreater
    Greater, GreaterEqual, GreaterGreater,

    At                     // '@'
}


// cf. lexer.rs (Writing an interpreter in Rust)
pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        Lexer { input: input.chars().peekable() }
    }
}