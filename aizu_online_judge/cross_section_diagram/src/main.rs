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

type Area = (usize, usize);

fn main() {
    let token_str: String = read();

    let mut index_stack: Vec<usize> = vec![];
    let mut area_stack: Vec<Area> = vec![];

    let mut sum: usize = 0;

    for (i, token) in token_str.chars().enumerate() {
        match &token.to_string()[..] {
            "\\" => {
                index_stack.push(i);
            }
            "/" => {
                let j = index_stack.pop();
                match j {
                    Some(j) => {
                        let mut a = i - j;
                        sum += a;
                        loop {
                            let area_tuple = area_stack.pop();
                            match area_tuple {
                                Some(area) => {
                                    let (k, b) = area;
                                    if j < k {
                                        a += b;
                                    } else {
                                        area_stack.push((k, b));
                                        break;
                                    };
                                }
                                _ => {
                                    break;
                                }
                            };
                        }

                        area_stack.push((j, a));
                    }
                    _ => {}
                };
            }
            _ => {}
        }
    }

    println!("{}", sum);

    let area_stack_len = area_stack.len();
    if area_stack_len != 0 {
        print!("{} ", area_stack_len);
    } else {
        print!("{}", area_stack_len);
    }
    for i in 0..area_stack_len {
        let area = area_stack[i];
        if i != area_stack_len - 1 {
            print!("{} ", area.1);
        } else {
            print!("{}", area.1);
        }
    }
    println!("");
}
