#[derive(Debug, PartialEq, Clone)]
enum Token {
    Number(f64),
    Plus,     // +
    Minus,    // -
    Asterisk, // *
    Slash,    // /
    LParen,   // (
    RParen,   // )
}
