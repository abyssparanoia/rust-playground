use std::io::*;
use std::str::FromStr;

#[derive(Clone)]
struct Card {
    suit: char,
    value: u32,
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.suit, self.value)
    }
}

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

fn bubble_sort(target_array: &mut Vec<Card>) {
    let n = target_array.len();
    let mut flag = true;

    while flag {
        flag = false;
        for j in (1..n).rev() {
            if target_array[j as usize].value < target_array[(j - 1) as usize].value {
                target_array.swap(j as usize, (j - 1) as usize);
                flag = true;
            }
        }
    }
}

fn selection_sort(target_array: &mut Vec<Card>) {
    let n = target_array.len();

    for i in 0..n {
        let mut min_j = i;
        for j in i..n {
            if target_array[j as usize].value < target_array[min_j as usize].value {
                min_j = j
            };
        }
        if min_j != i {
            target_array.swap(i as usize, min_j as usize);
        }
    }
}

fn is_equal(a: &mut Card, b: &mut Card) -> bool {
    a.value == b.value && a.suit == b.suit
}

fn is_stable(base_array: &mut Vec<Card>, target_array: &mut Vec<Card>) {
    let n = target_array.len();

    for i in 1..n - 1 as usize {
        for j in i + 1..n as usize {
            for a in 1..n - 1 as usize {
                for b in a + 1..n as usize {
                    if is_equal(&mut base_array[i], &mut target_array[j])
                        && is_equal(&mut base_array[i], &mut target_array[b])
                        && is_equal(&mut base_array[i], &mut target_array[a])
                    {
                        println!("Not stable");
                        return;
                    }
                }
            }
        }
    }
    println!("Stable")
}

fn display_array<T: std::fmt::Display>(target_array: &mut Vec<T>) {
    let n = target_array.len();

    for index in 0..n {
        if index != n - 1 {
            print!("{} ", target_array[index as usize]);
        } else {
            print!("{}", target_array[index as usize]);
        }
    }
    println!("");
}

fn main() {
    let n: u32 = read();

    let mut base_array: Vec<Card> = (0..n)
        .map(|_| read::<String>())
        .map(|input_v| Card {
            suit: input_v.chars().nth(0).unwrap(),
            value: input_v.chars().nth(1).unwrap().to_digit(10).unwrap(),
        })
        .collect();

    let mut target_array1 = base_array.to_vec();
    bubble_sort(&mut target_array1);
    display_array::<Card>(&mut target_array1);
    is_stable(&mut base_array, &mut target_array1);

    let mut target_array2 = base_array.to_vec();
    selection_sort(&mut target_array2);
    display_array::<Card>(&mut target_array2);
    is_stable(&mut base_array, &mut target_array2)
}
