use std::collections::VecDeque;
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

fn main() {
    let n: u32 = read();

    let mut linked_list = VecDeque::<i32>::new();

    for _ in 0..n {
        let command: String = read();

        match command.as_ref() {
            "insert" => {
                let value: i32 = read();
                linked_list.push_front(value);
            }
            "delete" => {
                let value: i32 = read();
                match linked_list.iter().position(|&v| v == value) {
                    Some(index) => {
                        linked_list.remove(index).unwrap();
                    }
                    _ => {}
                };
            }
            "deleteFirst" => {
                linked_list.pop_front();
            }
            "deleteLast" => {
                linked_list.pop_back();
            }
            _ => {}
        }
    }

    let l = linked_list.len();
    for i in 0..l {
        if i != l - 1 {
            print!("{} ", linked_list[i]);
        } else {
            println!("{}", linked_list[i]);
        }
    }
}
