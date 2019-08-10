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
    let s: String = read();
    let mut array = vec![1; s.len()];

    for _ in 0..10i32.pow(5) {
        for (index, c) in s.chars().enumerate() {
            if array[index] == 0 {
                continue;
            }

            array[index] = array[index] - 1;

            if c == 'R' {
                array[index + 1] += 1;
            } else {
                array[index - 1] += 1;
            }
        }
    }

    println!("{:?}", array)
}
