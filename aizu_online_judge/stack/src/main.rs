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

fn main() {
    let token_array = read_line::<char>();

    for i in 0..token_array.len() {
        println!("{}", token_array[i as usize])
    }
}
