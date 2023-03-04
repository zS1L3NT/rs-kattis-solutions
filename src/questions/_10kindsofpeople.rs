enum Direction {
    Up,
    Right,
    Down,
    Left,
}

struct Location(i16, i16);

impl Location {
    fn plus(&self, dir: &Direction, r: usize, c: usize) -> Option<Location> {
        match dir {
            Direction::Up => {
                if self.0 == 1 {
                    None
                } else {
                    Some(Location(self.0 - 1, self.1))
                }
            }
            Direction::Right => {
                if self.1 == c as i16 {
                    None
                } else {
                    Some(Location(self.0, self.1 + 1))
                }
            }
            Direction::Down => {
                if self.0 == r as i16 {
                    None
                } else {
                    Some(Location(self.0 + 1, self.1))
                }
            }
            Direction::Left => {
                if self.1 == 1 {
                    None
                } else {
                    Some(Location(self.0, self.1 - 1))
                }
            }
        }
    }

    fn get_val<'a, T>(&self, map: &'a Vec<Vec<T>>) -> &'a T {
        map.get(self.0 as usize - 1)
            .unwrap()
            .get(self.1 as usize - 1)
            .unwrap()
    }

    fn get_mut_val<'a, T>(&self, map: &'a mut Vec<Vec<T>>) -> &'a mut T {
        map.get_mut(self.0 as usize - 1)
            .unwrap()
            .get_mut(self.1 as usize - 1)
            .unwrap()
    }
}

fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    input.trim().to_string()
}

fn group(
    group_id: i32,
    groups: &mut Vec<Vec<i32>>,
    map: &Vec<Vec<i8>>,
    r: usize,
    c: usize,
    curr_loc: Location,
) {
    *curr_loc.get_mut_val(groups) = group_id;
    let mut queue = vec![curr_loc];

    while !queue.is_empty() {
        let curr_loc = queue.remove(0);

        for dir in &[
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ] {
            if let Some(next_loc) = curr_loc.plus(dir, r, c) {
                if curr_loc.get_val(map) == next_loc.get_val(map)
                    && curr_loc.get_val(groups) != next_loc.get_val(groups)
                {
                    *next_loc.get_mut_val(groups) = group_id;
                    queue.push(next_loc);
                }
            }
        }
    }
}

#[allow(dead_code)]
pub fn main() {
    let map_sizes = get_input()
        .split_ascii_whitespace()
        .map(|w| w.parse::<i16>().unwrap())
        .collect::<Vec<_>>();
    let (r, c) = (
        *map_sizes.first().unwrap() as usize,
        *map_sizes.get(1).unwrap() as usize,
    );

    let mut map = vec![];
    let mut groups = vec![];
    for _ in 0..r {
        let row = get_input()
            .split("")
            .skip(1)
            .take(c)
            .map(|w| w.parse::<i8>().unwrap())
            .collect::<Vec<i8>>();
        map.push(row);
        groups.push(vec![0; c]);
    }

    let mut group_id = 1;
    for curr_r in 1..=r {
        for curr_c in 1..=c {
            let curr_loc = Location(curr_r as i16, curr_c as i16);
            if curr_loc.get_val(&groups) == &0 {
                group(group_id, &mut groups, &map, r, c, curr_loc);
                group_id += 1;
            }
        }
    }

    let n = get_input().parse::<i32>().unwrap();
    for _ in 0..n {
        let query = get_input()
            .split_ascii_whitespace()
            .map(|w| w.parse::<i16>().unwrap())
            .collect::<Vec<_>>();
        let start_loc = Location(*query.first().unwrap(), *query.get(1).unwrap());
        let end_loc = Location(*query.get(2).unwrap(), *query.get(3).unwrap());

        if start_loc.get_val(&map) == end_loc.get_val(&map)
            && start_loc.get_val(&groups) == end_loc.get_val(&groups)
        {
            if start_loc.get_val(&map) == &0 {
                println!("binary");
            } else {
                println!("decimal");
            }
        } else {
            println!("neither");
        }
    }
}
