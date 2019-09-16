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

fn bubble_sort(mut target_array: Vec<u32>) {
    let n = target_array.len();
    let mut flag = true;

    while flag {
        flag = false;
        for j in (1..n).rev() {
            if target_array[j as usize] < target_array[(j - 1) as usize] {
                let temp = target_array[j as usize];
                target_array[j as usize] = target_array[(j - 1) as usize];
                target_array[(j - 1) as usize] = temp;
                flag = true;
            }
        }
    }
}

fn selection_sort(mut target_array: Vec<u32>) {
    let n = target_array.len();

    for i in 0..n {
        let mut min_j = i;
        for j in i..n {
            if target_array[j as usize] < target_array[min_j as usize] {
                min_j = j
            };
        }
        if min_j != i {
            let temp = target_array[i as usize];
            target_array[i as usize] = target_array[min_j as usize];
            target_array[min_j as usize] = temp;
        }
    }
}

fn main() {
    println!("Hello, world!");
}
