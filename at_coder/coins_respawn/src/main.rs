use std::io::*;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}

struct Edge {
    to: u32,
    weight: u32,
}

fn build_edge(to: u32, weight: u32) -> Edge {
    Edge { to, weight }
}

fn main() {
    println!("Hello, world!");
}
