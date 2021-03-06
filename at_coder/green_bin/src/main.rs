use std::io::*;
use std::str::FromStr;
use std::collections::BTreeMap;


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
    println!("Hello, world!");

    let N:i32 = read();

    let v: Vec<String> = (0..N).map(|_| read()).collect();

    let mut anagramMap = BTreeMap::new();

    for s in &v {
        for i in 0..s.len() {
            
        }
    }
}
