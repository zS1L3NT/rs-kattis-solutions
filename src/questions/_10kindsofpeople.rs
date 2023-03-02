use std::io::BufRead;

fn get_inputs() -> impl Iterator<Item = String> {
    std::io::stdin().lock().lines().map(|l| l.unwrap())
}

#[allow(dead_code)]
pub fn main() {
    let (map, r, c, queries) = parse_inputs(get_inputs());
    for (from_loc, to_loc) in queries {
        println!("{}", solve(&map, &r, &c, from_loc, to_loc));
    }
}

fn parse_inputs(
    mut inputs: impl Iterator<Item = String>,
) -> (Vec<Vec<i8>>, i16, i16, Vec<(Location, Location)>) {
    let map_metadata = inputs
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|w| w.parse::<i16>().unwrap())
        .collect::<Vec<_>>();
    let r = *map_metadata.first().unwrap() as usize;
    let c = *map_metadata.get(1).unwrap() as usize;
    let map = inputs
        .by_ref()
        .take(r)
        .map(|l| {
            l.split("")
                .skip(1)
                .take(c)
                .map(|w| w.parse::<i8>().unwrap())
                .collect::<Vec<i8>>()
        })
        .collect::<Vec<_>>();
    let n = inputs.next().unwrap().parse::<i32>().unwrap() as usize;
    let queries = inputs
        .take(n)
        .map(|l| {
            l.split_ascii_whitespace()
                .take(4)
                .map(|w| w.parse::<i16>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|ns| {
            (
                Location(*ns.first().unwrap(), *ns.get(1).unwrap()),
                Location(*ns.get(2).unwrap(), *ns.get(3).unwrap()),
            )
        })
        .collect::<Vec<_>>();

    (map, r as i16, c as i16, queries)
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Location(i16, i16);

impl Location {
    fn plus(&self, dir: &Direction, r: &i16, c: &i16) -> Option<Location> {
        match dir {
            Direction::Up => {
                if self.0 == 1 {
                    None
                } else {
                    Some(Location(self.0 - 1, self.1))
                }
            }
            Direction::Right => {
                if self.1 == *c {
                    None
                } else {
                    Some(Location(self.0, self.1 + 1))
                }
            }
            Direction::Down => {
                if self.0 == *r {
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
}

#[derive(Debug, PartialEq)]
enum User {
    Decimal,
    Binary,
    Neither,
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            User::Decimal => write!(f, "decimal"),
            User::Binary => write!(f, "binary"),
            User::Neither => write!(f, "neither"),
        }
    }
}

fn get_val<'a>(map: &'a Vec<Vec<i8>>, loc: &Location) -> &'a i8 {
    map.get((loc.0 - 1) as usize)
        .unwrap()
        .get((loc.1 - 1) as usize)
        .unwrap()
}

fn solve(map: &Vec<Vec<i8>>, r: &i16, c: &i16, from_loc: Location, to_loc: Location) -> User {
    let from_val = get_val(map, &from_loc);
    let to_val = get_val(map, &to_loc);

    if from_val != to_val {
        return User::Neither;
    }

    if traverse(map, r, c, &mut vec![from_loc], &to_loc, to_val) {
        if from_val == &0 {
            User::Binary
        } else {
            User::Decimal
        }
    } else {
        User::Neither
    }
}

fn traverse(
    map: &Vec<Vec<i8>>,
    r: &i16,
    c: &i16,
    visited_locs: &mut Vec<Location>,
    to_loc: &Location,
    to_val: &i8
) -> bool {
    let curr_loc = visited_locs.last().copied().unwrap();

    if &curr_loc == to_loc {
        return true;
    }

    if get_val(map, &curr_loc) != to_val {
        return false;
    }

    for dir in &[
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ] {
        if let Some(next_loc) = curr_loc.plus(dir, r, c) {
            if visited_locs.contains(&next_loc) {
                continue;
            }

            visited_locs.push(next_loc);
            if traverse(map, r, c, visited_locs, to_loc, to_val) {
                return true;
            }
        }
    }

    false
}

#[test]
fn sample_input_1() {
    let inputs = vec!["1 4", "1100", "2", "1 1 1 4", "1 1 1 1"];
    let (map, r, c, queries) = parse_inputs(inputs.iter().map(|l| l.to_string()));
    assert_eq!(map, vec![vec![1, 1, 0, 0]]);
    assert_eq!(r, 1);
    assert_eq!(c, 4);
    assert_eq!(
        queries,
        vec![
            (Location(1, 1), Location(1, 4)),
            (Location(1, 1), Location(1, 1))
        ]
    );
    assert_eq!(
        solve(&map, &r, &c, Location(1, 1), Location(1, 4)),
        User::Neither
    );
    assert_eq!(
        solve(&map, &r, &c, Location(1, 1), Location(1, 1)),
        User::Decimal
    );
}

#[test]
fn sample_input_2() {
    let inputs = vec![
        "10 20",
        "11111111111111111111",
        "11000000000000000101",
        "11111111111111110000",
        "11111111111111110000",
        "11000000000000000111",
        "00011111111111111111",
        "00111111111111111111",
        "10000000000000001111",
        "11111111111111111111",
        "11111111111111111111",
        "3",
        "2 3 8 16",
        "8 1 7 3",
        "1 1 10 20",
    ];
    let (map, r, c, queries) = parse_inputs(inputs.iter().map(|l| l.to_string()));
    assert_eq!(
        map,
        vec![
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0],
            vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1],
            vec![0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        ]
    );
    assert_eq!(r, 10);
    assert_eq!(c, 20);
    assert_eq!(
        queries,
        vec![
            (Location(2, 3), Location(8, 16)),
            (Location(8, 1), Location(7, 3)),
            (Location(1, 1), Location(10, 20))
        ]
    );
    for _ in 0..1000 {

        assert_eq!(
            solve(&map, &r, &c, Location(2, 3), Location(8, 16)),
            User::Binary
        );
        assert_eq!(
            solve(&map, &r, &c, Location(8, 1), Location(7, 3)),
            User::Decimal
        );
        assert_eq!(
            solve(&map, &r, &c, Location(1, 1), Location(10, 20)),
            User::Neither
        );
    }
}
