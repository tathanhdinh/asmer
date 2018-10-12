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
    Plus, Minus, Tidle,
    Slash,
    BackSlash,
    LParen, RParen, LBrac, RBrac, LCurly, RCurly,
    Star, Dot, Comma, Dollar, Equal, EqualEqual,
}