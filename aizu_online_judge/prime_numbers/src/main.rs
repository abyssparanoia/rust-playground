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

fn is_prime_number(n: u32) -> bool {
    let max = (n as f32).sqrt() as u32 + 1;

    for i in 2..max {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

fn main() {
    let n = read();

    let input_array: Vec<u32> = (0..n).map(|_| read::<u32>()).collect();

    let mut cnt = 0;
    for v in input_array {
        cnt = if is_prime_number(v) { cnt + 1 } else { cnt };
    }

    println!("{}", cnt)
}
