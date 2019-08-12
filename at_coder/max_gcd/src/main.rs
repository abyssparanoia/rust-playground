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
    // let a = 21;
    // let b = 14;

    // let ans = recurse((a, b), &|(x, y), gcd| {
    //     if y == 0 {
    //         return x;
    //     }
    //     gcd((y, x % y))
    // });

    let N: u32 = read();
    let K: u32 = read();

    let a_n: Vec<u32> = (0..N).map(|_| read()).collect();

    let mut ans = 0;

    for _ in 0..K {
        for n in 0..a_n.len() {
            let x: u32 = *a_n.get(n).unwrap();
            let y: u32 = *a_n.get(n + 1).unwrap();

            ans = recurse((x, y), &|(xt, yt), gcd| {
                if yt == 0 {
                    return xt;
                }
                gcd((yt, xt % yt))
            });
        }

        if ans != 0 {
            break;
        }
    }

    println!("{}", ans);
}
