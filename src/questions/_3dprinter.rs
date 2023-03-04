fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    input.trim().to_string()
}

#[allow(dead_code)]
pub fn main() {
    let n = get_input().parse::<i32>().unwrap();

    let mut printers = 1;
    let mut days = 0;
    loop {
        if printers >= n {
            println!("{}", days + 1);
            break;
        }

        printers += 2_i32.pow(days);
        days += 1;
    }
}