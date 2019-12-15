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

type Area = (isize, isize);

fn main() {
    let token_list: Vec<char> = read_line();

    let index_stack: Vec<i32> = vec![];
    let area_stack: Vec<Area> = vec![];

    println!("Hello, world!");
}
