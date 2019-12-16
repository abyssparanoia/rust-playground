use std::collections::HashMap;
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
    let n: usize = read();

    let mut map: HashMap<String, i32> = HashMap::new();

    let mut ans_array: Vec<String> = vec![];

    for _ in 0..n {
        let command: String = read();
        let value: String = read();

        match command.as_ref() {
            "insert" => {
                map.insert(value, 1);
            }
            "find" => match map.get(&value) {
                Some(_v) => {
                    ans_array.push("yes".to_string());
                }
                None => {
                    ans_array.push("no".to_string());
                }
            },
            _ => {}
        }
    }

    for i in 0..ans_array.len() as usize {
        println!("{}", ans_array[i]);
    }
}
