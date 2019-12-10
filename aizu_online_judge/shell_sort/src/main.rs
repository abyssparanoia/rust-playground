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

fn insertion_sort(mut input_array: Vec<i32>, n: i32, g: i32) -> i32 {
    let mut cnt = 0;

    for i in 0..n {
        let v = input_array[i as usize];
        let mut j = i - g;

        while j >= 0 && input_array[j as usize] > v {
            input_array[(j + g) as usize] = input_array[j as usize];
            j = j - g;
            cnt += 1;
        }

        input_array[(j + g) as usize] = v
    }

    cnt
}

fn main() {
    let n: i32 = read();

    let mut input_array: Vec<i32> = (0..n).map(|_| read::<i32>()).collect();

    println!("Hello, world!");
}
