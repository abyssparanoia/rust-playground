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

fn merge(array: &mut Vec<u32>, left: usize, mid: usize, right: usize) {
    let n1 = mid - left;
    let n2 = right - mid;

    let mut l_array: Vec<u32> = (0..n1).map(|idx| array[left + idx]).collect();
    let mut r_array: Vec<u32> = (0..n2).map(|idx| array[mid + idx]).collect();

    l_array[n1] = std::u32::MAX;
    r_array[n2] = std::u32::MAX;

    let mut idx_i: usize = 0;
    let mut idx_j: usize = 0;

    for k in left..right {
        if l_array[idx_i] <= r_array[idx_j] {
            array[k] = l_array[idx_i];
            idx_i += 1;
        } else {
            array[k] = r_array[idx_j];
            idx_j += 1;
        }
    }
}

fn mergeSort(array: &mut Vec<u32>, left: usize, right: usize) {
    if left + 1 < right {
        let mid = (left + right) / 2;
        mergeSort(array, left, mid);
        mergeSort(array, mid, right);
        merge(array, left, mid, right);
    }
}

fn main() {
    println!("Hello, world!");
}
