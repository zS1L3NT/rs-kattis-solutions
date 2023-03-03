fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    input.trim().to_string()
}

#[allow(dead_code)]
pub fn main() {
    println!(
        "{}",
        (0..get_input().parse::<i32>().unwrap())
            .map(|_| {
                let data = get_input()
                    .split_ascii_whitespace()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
                let students = data[0] as f32;
                let results = data.iter().skip(1).copied().collect::<Vec<i32>>();
                let average = (results.iter().sum::<i32>() as f32) / students;
                let above_average =
                    results.iter().filter(|i| (**i) as f32 > average).count() as f32;
                format!("{:.3}%", (above_average / students) * 100.0)
            })
            .collect::<Vec<_>>()
            .join("\n")
    );
}
