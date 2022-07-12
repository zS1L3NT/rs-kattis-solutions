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
    let str_iter = string.chars().into_iter();
    let qms = string.chars().filter(|&c| c == '?').count() as u32;
    let mut variations = vec![];

    for i in 0..(u32::pow(2, qms) as usize) {
        let mut replaced = 0;
        let binary = format!("{:b}", i).chars().collect::<Vec<_>>();
        let binary = [vec!['0'; (qms - binary.len() as u32) as usize], binary].concat();

        variations.push(
            str_iter
                .clone()
                .map(|c| {
                    if c == '?' {
                        replaced += 1;
                        binary[replaced - 1]
                    } else {
                        c
                    }
                })
                .collect::<String>(),
        );
    }

    variations
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
