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
    let n: u32 = read();

    let mut target_array: Vec<u32> = (0..n).map(|_| read::<u32>()).collect();

    let mut flag = true;
    let mut count = 0;

    while flag {
        flag = false;
        for j in (1..n).rev() {
            if target_array[j as usize] < target_array[(j - 1) as usize] {
                let temp = target_array[j as usize];
                target_array[j as usize] = target_array[(j - 1) as usize];
                target_array[(j - 1) as usize] = temp;
                flag = true;
                count += 1;
            }
        }
    }

    for index in 0..n {
        if index != n - 1 {
            print!("{} ", target_array[index as usize]);
        } else {
            print!("{}", target_array[index as usize]);
        }
    }
    println!("");
    println!("{}", count);
}
