use std::io::BufRead;

fn get_inputs() -> Vec<String> {
    std::io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect()
}

#[allow(dead_code)]
fn main() {
    let inputs = get_inputs();
    let metadata = inputs.get(0).unwrap().split(' ').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let n = *metadata.first().unwrap() as usize;
    let s = *metadata.get(1).unwrap();
    let m = *metadata.get(2).unwrap();
    let squares = inputs.get(1).unwrap().split(' ').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let (fate, hops) = play(squares[..n].to_vec(), s, m);
    println!("{fate}");
    println!("{hops}");
}

fn play(squares: Vec<i32>, start: i32, magic: i32) -> (String, usize) {
    let mut visited_indexes = vec![];
    let mut index = start - 1;

    while !visited_indexes.contains(&index) {
        if squares.get(index as usize) == Some(&magic) {
            return ("magic".to_string(), visited_indexes.len());
        }

        visited_indexes.push(index);
        if let Some(value) = squares.get(index as usize) {
            index += value;
        } else {
            return (if index < 0 { "left".to_string() } else { "right".to_string() }, visited_indexes.len() - 1)
        }
    }

    ("cycle".to_string(), visited_indexes.len())
}

#[test]
fn test_sample_1() {
    assert_eq!(play(vec![-9, 1, 42, -2, -3, -3], 4, 42), ("magic".to_string(), 2))
}

#[test]
fn test_sample_2() {
    assert_eq!(play(vec![7, 5, 4, 2, 13, -2, -3, 6], 2, 13), ("cycle".to_string(), 4))
}

#[test]
fn test_custom_1() {
    assert_eq!(play(vec![1, -10], 2, 1), ("left".to_string(), 1))
}

#[test]
fn test_custom_2() {
    assert_eq!(play(vec![10, 1], 1, 1), ("right".to_string(), 1))
}

#[test]
fn test_custom_3() {
    assert_eq!(play(vec![10], 1, 10), ("magic".to_string(), 0))
}