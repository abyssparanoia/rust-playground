use std::io::*;
use std::str::FromStr;

fn read_line<T: FromStr>() -> Vec<T> {
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    s.trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}

struct Diagram {
    width: i32,
    depth: u32,
}

impl Diagram {
    fn new() -> Diagram {
        Diagram { width: 0, depth: 0 }
    }

    fn foward(mut self, token: &'static str) {
        match token {
            "\\" => {
                self.depth += 1;
                self.width += 1;
            }
            "/" => {
                self.depth -= 1;
                self.width -= 1;
            }
            _ => {}
        }
    }

    fn is_finish(self) -> bool {
        self.depth == 0 && self.width != 0
    }
}

fn main() {
    println!("Hello, world!");
}
