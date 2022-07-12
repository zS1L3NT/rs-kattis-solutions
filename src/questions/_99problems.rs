use std::io::{stdin, stdout, Write};

#[allow(dead_code)]
pub fn main() {
    let mut input = String::new();
    stdout().flush().unwrap();
    stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    let num = input.parse::<i32>().unwrap();
    if num % 100 >= 49 || num / 100 == 0 {
        println!("{}", (((num / 100) + 1) * 100) - 1);
    } else {
        println!("{}", ((num / 100) * 100) - 1);
    }
}
