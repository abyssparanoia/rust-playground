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
    let t: i64 = read();

    let r_t: Vec<u32> = (0..t).map(|_| read::<u32>()).collect();

    let mut min = r_t[0] - r_t[(t - 1) as usize];
    for i in 0..t {
        println!("min: {}", min);
        if i == t - 1 {
            break;
        }
        for j in (i + 1)..t {
            let result = r_t[i as usize] - r_t[j as usize];
            println!("result: {}", result);
            if result < min {
                min = result;
            }
        }
    }
    println!("{}", min)
}
