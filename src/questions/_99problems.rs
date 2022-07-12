fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    input.trim().to_string()
}

#[allow(dead_code)]
pub fn main() {
    let num = get_input().parse::<i32>().unwrap();
    if num % 100 >= 49 || num / 100 == 0 {
        println!("{}", (((num / 100) + 1) * 100) - 1);
    } else {
        println!("{}", ((num / 100) * 100) - 1);
    }
}
