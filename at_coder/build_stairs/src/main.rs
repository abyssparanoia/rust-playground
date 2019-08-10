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

    let h: Vec<i32> = (0..n).map(|_| read::<i32>()).collect();

    let yes = h[..].windows(2).all(|w| {
        let h_current = w[0];
        let h_next = w[1];

        h_next - h_current >= -1
    });
    println!("{}", if yes { "Yes" } else { "No" })
}
