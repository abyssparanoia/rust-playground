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
    let n: i32 = read();

    let mut base_array: Vec<i32> = (0..n).map(|_| read::<i32>()).collect();

    for i in 0..n {
        let v = base_array[i as usize];
        let mut j = i - 1;

        while j >= 0 && base_array[j as usize] > v {
            base_array[(j + 1) as usize] = base_array[j as usize];
            j = j - 1;
        }

        base_array[(j + 1) as usize] = v;

        for index in 0..n {
            if index != n - 1 {
                print!("{} ", base_array[index as usize]);
            } else {
                print!("{}", base_array[index as usize]);
            }
        }
        println!("");
    }
}
