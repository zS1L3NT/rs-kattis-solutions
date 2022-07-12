use std::io::{stdin, stdout, Write};

pub fn main() {
    let mut input = String::new();
    stdout().flush().unwrap();
    stdin().read_line(&mut input).unwrap();
    println!(
        "{}",
        get_possible_variations(&input.trim().chars().collect::<Vec<_>>())
    );
}

fn get_possible_variations(chars: &Vec<char>) -> usize {
    match chars.iter().position(|char| char == &'?') {
        Some(index) => {
            let mut vec0 = chars.clone();
            let mut vec1 = chars.clone();
            vec0[index] = '0';
            vec1[index] = '1';
            (get_possible_variations(&vec0) + get_possible_variations(&vec1)) % 1000000007
        }
        None => count_iterations(chars) % 1000000007,
    }
}

fn count_iterations(chars: &Vec<char>) -> usize {
    let mut finished = 0;
    let mut moves = 0;
    let len = chars.len();

    for i in 0..len {
        if chars[len - i - 1] == '1' {
            moves += i - finished;
            finished += 1;
        }
    }

    moves
}
