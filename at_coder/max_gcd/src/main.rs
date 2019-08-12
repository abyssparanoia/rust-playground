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

fn recurse<X, Y>(x: X, f: &Fn(X, &Fn(X) -> Y) -> Y) -> Y {
    f(x, &|x: X| recurse(x, &f))
}

fn main() {
    let a = 21;
    let b = 14;

    let ans = recurse((a, b), &|(x, y), gcd| {
        if y == 0 {
            return x;
        }
        gcd((y, x % y))
    });

    println!("{}", ans);
}
