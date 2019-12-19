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

struct KochCurve {
    x: f32,
    y: f32,
}

impl KochCurve {
    fn new() -> KochCurve {
        KochCurve { x: 0.0, y: 0.0 }
    }

    fn koch(&mut self, n: usize, a: &mut KochCurve, b: &mut KochCurve) {
        if n == 0 {
            return;
        }

        let mut s: KochCurve = KochCurve::new();
        let mut t: KochCurve = KochCurve::new();
        let mut u: KochCurve = KochCurve::new();

        let th: f32 = std::f32::consts::PI * 60.0 / 180.0;

        s.x = (2.0 * a.x + 1.0 * b.x) / 3.0;
        s.y = (2.0 * a.y + 1.0 * b.y) / 3.0;
        t.x = (1.0 * a.x + 2.0 * b.x) / 3.0;
        t.y = (1.0 * a.y + 2.0 * b.y) / 3.0;

        u.x = (t.x - s.x) * th.cos() - (t.y - s.y) * th.sin() + s.x;
        u.y = (t.x - s.x) * th.sin() + (t.y - s.y) * th.cos() + s.y;

        KochCurve::new().koch(n - 1, a, &mut s);
        println!("{} {}", s.x, s.y);
        KochCurve::new().koch(n - 1, &mut s, &mut u);
        println!("{} {}", u.x, u.y);
        KochCurve::new().koch(n - 1, &mut u, &mut t);
        println!("{} {}", t.x, t.y);
        KochCurve::new().koch(n - 1, &mut t, b);
    }
}

fn main() {
    let n: usize = read();

    let mut a: KochCurve = KochCurve::new();
    let mut b: KochCurve = KochCurve::new();

    a.x = 0.0;
    a.y = 0.0;
    b.x = 100.0;
    b.y = 0.0;

    println!("{} {}", a.x, a.y);
    KochCurve::new().koch(n, &mut a, &mut b);
    println!("{} {}", b.x, b.y);
}
