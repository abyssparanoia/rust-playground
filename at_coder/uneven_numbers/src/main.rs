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

    let mut num = 1;

    loop {
        let temp = 10i32.pow(num) as f32;
        let result = (n as f32) / temp;
        if result <= 1.0 {
            break;
        }
        num += 1;
    }

    let mut ans = 0;

    for i in 1..num + 1 {
        if i == 1 {
            ans += 9;
            continue;
        }

        if i % 2 == 0 {
            continue;
        }

        if i == num {
            ans += n - 10i32.pow(i - 1) + 1;
            continue;
        }

        ans += 10i32.pow(i) - 10i32.pow(i - 1) - 1;
    }

    println!("{}", ans)
}
