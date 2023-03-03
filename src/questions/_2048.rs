fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    input.trim().to_string()
}

#[allow(dead_code)]
pub fn main() {
    let mut board: Vec<_> = vec![];
    for _ in 0..4 {
        board.push(
            get_input()
                .split_whitespace()
                .map(|i| i.parse::<i32>().unwrap())
                .collect(),
        );
    }

    let direction = get_input().parse::<i32>().unwrap();

    // Don't even ask me about this code
    // I compressed most of it and now it looks like shit
    let board: Vec<Vec<i32>> = match direction {
        0 => board.iter().map(resolve_row).collect(),
        1 => (0..4)
            .map(|ri| {
                (0..4)
                    .map(|ci| resolve_row(&board.iter().map(|row| row[ci]).collect())[ri])
                    .collect()
            })
            .collect(),
        2 => board
            .iter()
            .map(|row| {
                resolve_row(&row.iter().rev().copied().collect())
                    .iter()
                    .rev()
                    .copied()
                    .collect()
            })
            .collect(),
        3 => (0..4)
            .map(|ri| {
                (0..4)
                    .map(|ci| resolve_row(&board.iter().rev().map(|row| row[ci]).collect())[ri])
                    .collect()
            })
            .rev()
            .collect(),
        _ => panic!(""),
    };

    println!(
        "{}",
        board
            .iter()
            .map(|row| row
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>()
                .join(" "))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

fn resolve_row(line: &Vec<i32>) -> Vec<i32> {
    // All possible cases for what could happen
    let line = if line[0] == line[1] {
        if line[0] == line[2] && line[0] == line[3] {
            vec![line[0] * 2, line[0] * 2, 0, 0]
        } else if line[2] == line[3] {
            vec![line[0] * 2, line[2] * 2, 0, 0]
        } else {
            vec![line[0] * 2, line[2], line[3], 0]
        }
    } else if line[0] == line[2] {
        if line[1] == 0 {
            vec![line[0] * 2, line[3], 0, 0]
        } else {
            line.to_vec()
        }
    } else if line[0] == line[3] {
        if line[1] == line[2] {
            if line[1] == 0 {
                vec![line[0] * 2, 0, 0, 0]
            } else {
                vec![line[0], line[1] * 2, line[3], 0]
            }
        } else {
            line.to_vec()
        }
    } else if line[1] == line[2] {
        vec![line[0], line[1] * 2, line[3], 0]
    } else if line[1] == line[3] {
        if line[2] == 0 {
            vec![line[0], line[1] * 2, 0, 0]
        } else {
            line.to_vec()
        }
    } else if line[2] == line[3] {
        vec![line[0], line[1], line[2] * 2, 0]
    } else {
        line.to_vec()
    };

    if line.iter().sum::<i32>() == 0 {
        line
    } else {
        let mut line = line
            .iter()
            .filter(|i| **i != 0)
            .copied()
            .collect::<Vec<_>>();
        line.resize(4, 0);
        line
    }
}
