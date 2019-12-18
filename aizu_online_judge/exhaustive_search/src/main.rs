use std::collections::HashSet;
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

fn make_hash_set(a: Vec<i32>) -> HashSet<i32> {
    let mut hash_set: HashSet<i32> = HashSet::new();
    let length: usize = a.len();
    let hash_length: usize = 2usize.pow(length as u32);
    for i in 0..hash_length {
        let mut sum: i32 = 0;
        for j in 0..length {
            if (1 & i >> j) == 1 {
                sum += a[j];
            }
        }
        hash_set.insert(sum);
    }
    hash_set
}

fn main() {
    let n: usize = read();

    let mut a: Vec<i32> = (0..n).map(|_| read::<i32>()).collect();

    let hash_set: HashSet<i32> = make_hash_set(a.clone());
    // let q: usize = read();

    // for _i in 0..q {
    //     let m: i32 = read();
    //     let mut sum: i32 = 0;
    //     let mut idx: usize = 0;
    //     loop {
    //         sum += a[idx];

    //         if sum <
    //     }
    // }

    println!("Hello, world!");
}
