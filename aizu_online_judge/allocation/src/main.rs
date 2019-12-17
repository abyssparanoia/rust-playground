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

fn check(w: Vec<u32>, n: usize, k: usize, p: u32) -> bool {
    let mut idx: usize = 0;

    for _i in 0..k {
        let mut sum: u32 = 0;
        while w[idx] + sum <= p {
            sum += w[idx];
            idx += 1;
            if idx == n {
                return true;
            }
        }
    }
    false
}

fn main() {
    let n: usize = read();
    let k: usize = read();

    let w: Vec<u32> = (0..n).map(|_| read::<u32>()).collect();
}
