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

fn binary_search_recrusive(array: Vec<i32>, key: i32, low: i32, high: i32) -> i32 {
    let mid = (low + high) / 2;

    if low > high {
        -1
    } else if array[mid as usize] == key {
        mid
    } else if key < array[mid as usize] {
        binary_search_recrusive(array, key, low, mid - 1)
    } else {
        binary_search_recrusive(array, key, mid + 1, high)
    }
}

fn binary_search(array: Vec<i32>, key: i32) -> i32 {
    let len = array.len() as i32;
    binary_search_recrusive(array, key, 0, len - 1)
}

fn main() {
    let n: usize = read();
    let mut s_array: Vec<i32> = (0..n).map(|_| read::<i32>()).collect();
    let q: usize = read();
    let t_array: Vec<i32> = (0..q).map(|_| read::<i32>()).collect();

    s_array.sort();

    let mut cnt = 0;

    for t in t_array {
        let res = binary_search(s_array.clone(), t);
        if res != -1 {
            cnt += 1;
        }
    }

    println!("{}", cnt);
}
