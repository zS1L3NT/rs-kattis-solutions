pub fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    input.trim().to_string()
}

#[allow(dead_code)]
pub fn main() {
    let willing = get_input();
    let required = get_input();
    println!(
        "{}",
        if willing.len() >= required.len() {
            "go"
        } else {
            "no"
        }
    )
}
