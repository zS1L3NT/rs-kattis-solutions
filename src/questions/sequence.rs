use std::io::{stdin, stdout, Write};

pub fn main() {
    let mut input = String::new();
    stdout().flush().unwrap();
    stdin().read_line(&mut input).unwrap();
    println!(
        "{}",
        get_possible_variations(input.trim().to_string())
            .iter()
            .map(|variation| count_iterations(&mut variation.to_string()))
            .sum::<usize>()
            % 1000000007
    );
}

fn get_possible_variations(string: String) -> Vec<String> {
    match string.find(|char| char == '?') {
        Some(_) => [
            get_possible_variations(string.replacen("?", "0", 1)),
            get_possible_variations(string.replacen("?", "1", 1)),
        ]
        .concat(),
        None => vec![string],
    }
}

fn count_iterations(string: &mut String) -> usize {
    let mut finished = 0;
    let mut moves = 0;
    let chars = string.chars().collect::<Vec<_>>();
    let len = chars.len();

    for i in 0..len {
        if chars[len - i - 1] == '1' {
            moves += i - finished;
            finished += 1;
        }
    }

    moves
}
