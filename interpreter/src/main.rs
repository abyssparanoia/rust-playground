use core::borrow::Borrow;

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

enum Expr {
    Number(f64),
    PrefixExpr {
        operator: String,
        right: Box<Expr>,
    },
    InfixExpr {
        left: Box<Expr>,
        operator: String,
        right: Box<Expr>,
    },
}

#[derive(PartialOrd, PartialEq)]
enum Precedence {
    // 最低
    LOWEST,
    // "+", "-"
    SUM,
    // "*", "/"
    PRODUCT,
    // 前置演算子
    PREFIX,
}

struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    fn new(input: Vec<char>) -> Lexer {
        Lexer { input, position: 0 }
    }

    fn token(&mut self) -> Option<Token> {
        use std::iter::FromIterator;

        while self.curr().is_some() && self.curr().unwrap().is_whitespace() {
            self.next();
        }

        let curr = self.curr()?;
        let token = if Self::is_number(curr) {
            let mut number = vec![*curr];
            while self.peek().is_some() && Self::is_number(self.peek().unwrap()) {
                self.next();
                number.push(*self.curr().unwrap());
            }

            String::from_iter(number)
                .parse::<f64>()
                .ok()
                .and_then(|n| Some(Token::Number(n)))
        } else {
            match curr {
                &'+' => Some(Token::Plus),
                &'-' => Some(Token::Minus),
                &'*' => Some(Token::Asterisk),
                &'/' => Some(Token::Slash),
                &'(' => Some(Token::LParen),
                &')' => Some(Token::RParen),
                _ => None,
            }
        };
        self.next();
        token
    }

    fn curr(&mut self) -> Option<&char> {
        self.input.get(self.position)
    }

    fn next(&mut self) {
        self.position += 1;
    }

    fn peek(&mut self) -> Option<&char> {
        self.input.get(self.position + 1)
    }

    fn is_number(c: &char) -> bool {
        c.is_ascii_digit() || c == &'.'
    }
}

struct Parser {
    lexer: Lexer,
    curr: Option<Token>,
    peek: Option<Token>,
}

impl Parser {
    fn new(mut lexer: Lexer) -> Parser {
        let curr = lexer.token();
        let peek = lexer.token();
        Parser { lexer, curr, peek }
    }

    fn next(&mut self) {
        self.curr = self.peek.clone();
        self.peek = self.lexer.token();
    }

    // fn parse_prefix(&mut self) -> Option<Box<Expr>> {
    //     match self.curr.as_ref()? {
    //         Token::Minus => self.
    //     }
    // }

    // fn parse_minus(&mut self) -> Option<Box<Expr>> {
    //     self.next();
    //     let number = s
    // }

    fn parse_number(&mut self) -> Option<Box<Expr>> {
        match self.curr.borrow() {
            Some(Token::Number(n)) => Some(Box::new(Expr::Number(*n))),
            _ => None,
        }
    }

    fn token_precedence(token: &Token) -> Precedence {
        match token {
            Token::Plus | Token::Minus => Precedence::SUM,
            Token::Slash | Token::Asterisk => Precedence::PRODUCT,
            _ => Precedence::LOWEST,
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_lexer() {
    let mut lexer = Lexer::new("1 + 2".chars().collect());
    assert_eq!(lexer.token(), Some(Token::Number(1_f64)));
    assert_eq!(lexer.token(), Some(Token::Plus));
    assert_eq!(lexer.token(), Some(Token::Number(2_f64)));
    assert_eq!(lexer.token(), None);
}
