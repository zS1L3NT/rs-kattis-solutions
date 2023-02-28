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
    println!("{}", parse(get_inputs()).join("\n"));
}

fn parse(input: Vec<String>) -> Vec<String> {
    let mut variables = std::collections::HashMap::<String, i32>::new();
    let mut output = vec![];

    for line in input {
        if line.is_empty() {
            continue;
        }

        let mut words = line.split(' ');
        match words.next() {
            Some("def") => {
                let name = words.next().unwrap().to_string();
                let value = words.next().unwrap().parse::<i32>().unwrap();

                variables.insert(name, value);
            }
            Some("calc") => {
                let mut value = variables.get(words.next().unwrap()).copied();

                while value.is_some() {
                    if let Some(operator) = words.next() {
                        if operator == "=" {
                            break;
                        }
                        value =
                            variables
                                .get(words.next().unwrap())
                                .map(|next_value| match operator {
                                    "+" => value.unwrap() + next_value,
                                    "-" => value.unwrap() - next_value,
                                    _ => panic!("Unknown operator"),
                                });
                    } else {
                        break;
                    }
                }

                let variable =
                    variables
                        .iter()
                        .find_map(|(k, v)| if Some(*v) == value { Some(k) } else { None });
                output.push(format!(
                    "{} {}",
                    &line[5..],
                    variable.unwrap_or(&"unknown".to_string())
                ))
            }
            Some("clear") => variables.clear(),
            Some(_) | None => panic!("What the ???"),
        }
    }

    output
}

#[test]
fn test_sample() {
    assert_eq!(
        parse(vec![
            "def foo 3".to_string(),
            "calc foo + bar =".to_string(),
            "def bar 7".to_string(),
            "def programming 10".to_string(),
            "calc foo + bar =".to_string(),
            "def is 4".to_string(),
            "def fun 8".to_string(),
            "calc programming - is + fun =".to_string(),
            "def fun 1".to_string(),
            "calc programming - is + fun =".to_string(),
            "clear".to_string(),
        ]),
        vec![
            "foo + bar = unknown".to_string(),
            "foo + bar = 10".to_string(),
            "programming - is + fun = 10".to_string(),
            "programming - is + fun = 7".to_string(),
        ]
    );
}
