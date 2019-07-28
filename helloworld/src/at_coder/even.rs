
fn main() {
    let a: u32 = 20;
    let b: u32 = 27;

    let ans = if (a*b) % 2 == 0 { "Even" } else {"Odd"};
    println!("{}",ans );
}