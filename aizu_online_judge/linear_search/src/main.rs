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
    let s_array: Vec<i32> = (0..n).map(|_| read::<i32>()).collect();
    let q: usize = read();
    let t_array: Vec<i32> = (0..q).map(|_| read::<i32>()).collect();

    let mut cnt = 0;

    for t in t_array {
        match s_array.clone().into_iter().find(|&v| v == t) {
            Some(_v) => cnt += 1,
            _ => {}
        }
    }

    println!("{}", cnt);
}
