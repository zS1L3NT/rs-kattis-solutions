pub fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    input.trim().to_string()
}

#[allow(dead_code)]
pub fn main() {
    let mut nums = get_input()
        .split_ascii_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    nums.sort();
    let result = get_input()
        .chars()
        .map(|i| nums[vec!['A', 'B', 'C'].iter().position(|j| &i == j).unwrap()].to_string())
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", result);
}
