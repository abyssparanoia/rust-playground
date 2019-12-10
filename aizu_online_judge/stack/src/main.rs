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

    let mut stack: Vec<i32> = vec![];
    for i in 0..token_array.len() {
        let token = token_array[i as usize];
        match token.to_digit(10) {
            Some(token) => stack.push(token as i32),
            _ => {
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                match &*token.to_string() {
                    "+" => stack.push(y + x),
                    "-" => stack.push(y - x),
                    "*" => stack.push(y * x),
                    _ => println!("failed"),
                }
            }
        }
    }
    println!("{}", stack[0])
}
