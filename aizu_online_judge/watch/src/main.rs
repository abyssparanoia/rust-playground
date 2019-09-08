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
    let n: u32 = read();

    let h: u32 = n / 3600;

    let m: u32 = n / 60 - h * 60;

    let s: u32 = n - h * 3600 - m * 60;

    println!("{}:{}:{}", h, m, s)
}
