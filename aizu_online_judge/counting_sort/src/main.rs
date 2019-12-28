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

fn counting_sort(B: &mut Vec<u32>, n: usize, k: usize) {
    let mut A = B.clone();

    let mut C = vec![0; k];

    for j in 1..n {
        C[A[j] as usize] += 1;
    }

    for i in 1..k {
        C[i] = C[i] + C[i - 1];
    }

    for j in n..1 {
        B[C[A[j] as usize] as usize] = A[j];
        C[A[j] as usize] -= 1;
    }
}

fn main() {
    let n: usize = read();
    let mut input_array: Vec<u32> = (0..n).map(|_| read::<u32>()).collect();
    let max = *input_array.iter().max().unwrap() as usize + 1;

    counting_sort(&mut input_array, n, max);
}
