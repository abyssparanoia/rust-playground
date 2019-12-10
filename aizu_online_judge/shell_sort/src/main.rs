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

fn insertion_sort(target_array: &mut Vec<i32>, n: i32, g: i32) -> i32 {
    let mut cnt = 0;

    for i in 0..n {
        let v = target_array[i as usize];
        let mut j = i - g;

        while j >= 0 && target_array[j as usize] > v {
            target_array[(j + g) as usize] = target_array[j as usize];
            j = j - g;
            cnt += 1;
        }

        target_array[(j + g) as usize] = v
    }

    cnt
}

fn shell_sort(target_array: &mut Vec<i32>, n: i32) {
    let mut cnt = 0;

    let mut g: Vec<i32> = vec![];

    if n == 1 {
        g.push(1)
    } else {
        let mut h = 1;
        while h < n {
            g.push(h);
            h = 3 * h + 1;
        }
        g.reverse()
    }
    let m = g.len();

    for i in 0..m {
        let new_cnt = insertion_sort(target_array, n, g[i as usize]);
        cnt += new_cnt;
    }

    println!("{}", m);
    for index in 0..m {
        if index != m - 1 {
            print!("{} ", g[index as usize]);
        } else {
            println!("{}", g[index as usize]);
        }
    }
    println!("{}", cnt);
}

fn main() {
    let n: i32 = read();

    let mut target_array: Vec<i32> = (0..n).map(|_| read::<i32>()).collect();

    shell_sort(&mut target_array, n);

    for index in 0..n {
        println!("{}", target_array[index as usize]);
    }
}
