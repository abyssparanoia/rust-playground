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

fn main() {
    println!("Hello, world!");
}
