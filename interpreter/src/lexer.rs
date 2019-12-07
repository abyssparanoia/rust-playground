
struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {

    fn new(input: Vec<char>) -> Lexer {
        Lexer {input,position:0}
    }
}