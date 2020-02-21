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

fn paritition(array: &mut Vec<u32>, p: usize, r: usize) -> usize {
    let x = array[r];
    let mut i: i32 = (p as i32) - 1;

    for j in p..r {
        if array[j] <= x {
            i += 1;
            array.swap(i as usize, j);
        }
    }
    array.swap((i + 1) as usize, r);

    (i + 1) as usize
}

fn main() {
    println!("Hello, world!");
}
