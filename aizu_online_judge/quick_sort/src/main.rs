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

struct Trump {
    initilize_idx: usize,
    number: u32,
    token: String,
}

fn paritition(array: &mut Vec<Trump>, p: usize, r: usize) -> usize {
    let x = array[r].number;
    let mut i: i32 = (p as i32) - 1;

    for j in p..r {
        if array[j].number <= x {
            i += 1;
            array.swap(i as usize, j);
        }
    }
    array.swap((i + 1) as usize, r);

    (i + 1) as usize
}

fn quick_sort(array: &mut Vec<Trump>, p: usize, r: usize) {
    if p < r {
        let q = paritition(array, p, r);
        quick_sort(array, p, q - 1);
        quick_sort(array, q + 1, r);
    }
}

fn main() {
    let n: usize = read();

    let mut array: Vec<Trump> = (0..n)
        .map(|idx| {
            let token: String = read();
            let number: u32 = read();
            Trump {
                token,
                number,
                initilize_idx: idx,
            }
        })
        .collect();

    quick_sort(&mut array, 0, n - 1);

    let mut stable_flag = true;

    for idx in 1..n {
        if array[idx].number == array[idx - 1].number
            && array[idx].initilize_idx < array[idx - 1].initilize_idx
        {
            stable_flag = false;
            break;
        }
    }

    if stable_flag {
        println!("Stable");
    } else {
        println!("Not stable");
    }

    for idx in 0..n {
        println!("{} {}", array[idx].token, array[idx].number);
    }
}
