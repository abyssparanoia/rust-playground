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

    let a: Vec<i32> = (0..n).map(|_| read::<i32>()).collect();

    let hash_set: HashSet<i32> = make_hash_set(a.clone());
    let mut a_unique: Vec<i32> = hash_set.iter().map(|&v| v).collect();
    a_unique.sort();

    let q: usize = read();

    for _i in 0..q {
        let m: i32 = read();
        let result = binary_search(a_unique.clone(), m);
        if result == -1 {
            println!("no");
        } else {
            println!("yes");
        }
    }
}
