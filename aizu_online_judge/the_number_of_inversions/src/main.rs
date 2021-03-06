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

fn array_reader<T: FromStr>(n: usize) -> Vec<T> {
    (0..n).map(|_| read::<T>()).collect()
}

fn merge(array: &mut Vec<u32>, left: usize, mid: usize, right: usize) -> usize {
    let mut sum: usize = 0;

    let n1 = mid - left;
    let n2 = right - mid;
    let mut l_array: Vec<u32> = (0..n1).map(|idx| array[left + idx]).collect();
    let mut r_array: Vec<u32> = (0..n2).map(|idx| array[mid + idx]).collect();

    l_array[n1 - 1] = std::u32::MAX;
    r_array[n2 - 1] = std::u32::MAX;

    let mut idx_i: usize = 0;
    let mut idx_j: usize = 0;

    for k in left..right {
        if l_array[idx_i] <= r_array[idx_j] {
            array[k] = l_array[idx_i];
            idx_i += 1;
        } else {
            array[k] = r_array[idx_j];
            idx_j += 1;
            sum += 1;
        }
    }
    sum
}

fn merge_sort(array: &mut Vec<u32>, left: usize, right: usize) -> usize {
    let mut sum: usize = 0;
    if left + 1 < right {
        let mid = (left + right) / 2;
        let sum_1 = merge_sort(array, left, mid);
        let sum_2 = merge_sort(array, mid, right);
        let sum_3 = merge(array, left, mid, right);
        sum += sum_1 + sum_2 + sum_3;
    }
    sum
}

fn main() {
    let n: usize = read();

    let mut array = array_reader::<u32>(n);

    let result = merge_sort(&mut array, 0, n);

    println!("{}", result);
}
