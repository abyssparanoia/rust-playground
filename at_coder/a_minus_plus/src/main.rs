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
    let a: i32 = read();
    let b: i32 = read();

    let sum = a + b;
    let sub = a - b;
    let mul = a * b;

    let ans = if sum > sub && sum > mul {
        sum
    } else if sub > sum && sub > mul {
        sub
    } else {
        mul
    };

    println!("{}", ans)
}
