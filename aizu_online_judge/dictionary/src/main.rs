use std::collections::HashMap;
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

fn main() {
    let n: usize = read();

    let mut map: HashMap<String, String> = HashMap::new();

    for _ in 0..n {
        let command: String = read();
        let value: String = read();

        match command.as_ref() {
            "insert" => {}
            "find" => {}
            _ => {}
        }
    }

    println!("Hello, world!");
}
