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

fn gcd(x: u32, y: u32) -> u32 {
    let r = x % y;

    if r == 0 {
        return y;
    }

    return gcd(y, r);
}

fn main() {
    let x: u32 = read();
    let y: u32 = read();

    let result = if x > y { gcd(x, y) } else { gcd(y, x) };

    println!("{}", result)
}
