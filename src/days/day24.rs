use sorted_vec::ReverseSortedVec;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};

const FILE_PATH: &str = "inputs/day24.txt";
const TEST_FILE_PATH: &str = "inputs/day24_test.txt";

#[derive(Eq, PartialEq)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

fn get_future_winds(
    initial_winds: HashMap<(i32, i32), Dir>,
    height: i32,
    width: i32,
) -> HashMap<((i32, i32), i32), char> {
    let mut future_winds: HashMap<((i32, i32), i32), char> = HashMap::new();
    for t in 0..1000 {
        for wind_coordinate in initial_winds.keys() {
            match initial_winds[wind_coordinate] {
                Dir::Up => {
                    let mut row = wind_coordinate.0 - 1; // do not consider walls
                    row = (((row - t) % height) + height) % height;
                    if future_winds.contains_key(&((row + 1, wind_coordinate.1), t)) {
                        let ch = match future_winds[&((row + 1, wind_coordinate.1), t)] {
                            '^' | '>' | 'v' | '<' => '2',
                            '2' => '3',
                            '3' => '4',
                            _ => panic!("too many winds"),
                        };
                        future_winds.insert(((row + 1, wind_coordinate.1), t), ch);
                    } else {
                        future_winds.insert(((row + 1, wind_coordinate.1), t), '^');
                    }
                }
                Dir::Right => {
                    let mut col = wind_coordinate.1 - 1;
                    col = (col + t) % width;
                    if future_winds.contains_key(&((wind_coordinate.0, col + 1), t)) {
                        let ch = match future_winds[&((wind_coordinate.0, col + 1), t)] {
                            '^' | '>' | 'v' | '<' => '2',
                            '2' => '3',
                            '3' => '4',
                            _ => panic!("too many winds"),
                        };
                        future_winds.insert(((wind_coordinate.0, col + 1), t), ch);
                    } else {
                        future_winds.insert(((wind_coordinate.0, col + 1), t), '>');
                    }
                }
                Dir::Down => {
                    let mut row = wind_coordinate.0 - 1; // do not consider walls
                    row = (row + t) % height;
                    if future_winds.contains_key(&((row + 1, wind_coordinate.1), t)) {
                        let ch = match future_winds[&((row + 1, wind_coordinate.1), t)] {
                            '^' | '>' | 'v' | '<' => '2',
                            '2' => '3',
                            '3' => '4',
                            _ => panic!("too many winds"),
                        };
                        future_winds.insert(((row + 1, wind_coordinate.1), t), ch);
                    } else {
                        future_winds.insert(((row + 1, wind_coordinate.1), t), 'v');
                    }
                }
                Dir::Left => {
                    let mut col = wind_coordinate.1 - 1;
                    col = (((col - t) % width) + width) % width;
                    if future_winds.contains_key(&((wind_coordinate.0, col + 1), t)) {
                        let ch = match future_winds[&((wind_coordinate.0, col + 1), t)] {
                            '^' | '>' | 'v' | '<' => '2',
                            '2' => '3',
                            '3' => '4',
                            _ => panic!("too many winds"),
                        };
                        future_winds.insert(((wind_coordinate.0, col + 1), t), ch);
                    } else {
                        future_winds.insert(((wind_coordinate.0, col + 1), t), '<');
                    }
                }
            }
        }
    }
    future_winds
}

#[allow(dead_code)]
fn print_future_winds(
    future_winds: &HashMap<((i32, i32), i32), char>,
    height: i32,
    width: i32,
) -> () {
    let mut string: String = "".to_string();
    for t in 0..1 {
        for j_col in 0..width + 2 {
            if j_col == 1 {
                string.push('.');
            } else {
                string.push('#');
            }
        }
        string.push('\n');

        for i_row in 1..=height {
            string.push('#');
            for j_col in 1..=width {
                if future_winds.contains_key(&((i_row, j_col), t)) {
                    string.push(future_winds[&((i_row, j_col), t)]);
                } else {
                    string.push('.');
                }
            }
            string.push_str("#\n");
        }

        for j_col in 0..width + 2 {
            if j_col == width {
                string.push('.');
            } else {
                string.push('#');
            }
        }
        string.push_str("\n\n");
    }
    println!("{}", string);
}

fn get_valid_positions(height: i32, width: i32) -> HashSet<(i32, i32)> {
    let mut positions: HashSet<(i32, i32)> = HashSet::new();
    positions.insert((0, 1));
    positions.insert((height + 1, width));
    for row in 1..=height {
        for col in 1..=width {
            positions.insert((row, col));
        }
    }
    positions
}

#[allow(dead_code)]
fn print_pos(
    pos: (i32, i32),
    t: i32,
    future_winds: &HashMap<((i32, i32), i32), char>,
    height: i32,
    width: i32,
) -> () {
    let mut string: String = "".to_string();

    for j_col in 0..width + 2 {
        if j_col == pos.1 && pos.0 == 0 {
            string.push('E');
        } else if j_col == 1 {
            string.push('.');
        } else {
            string.push('#');
        }
    }
    string.push('\n');

    for i_row in 1..=height {
        string.push('#');
        for j_col in 1..=width {
            if future_winds.contains_key(&((i_row, j_col), t)) {
                string.push(future_winds[&((i_row, j_col), t)]);
            } else if i_row == pos.0 && j_col == pos.1 {
                string.push('E');
            } else {
                string.push('.');
            }
        }
        string.push_str("#\n");
    }

    for j_col in 0..width + 2 {
        if j_col == pos.1 && pos.0 == height + 1 {
            string.push('E');
        } else if j_col == width {
            string.push('.');
        } else {
            string.push('#');
        }
    }
    string.push_str("\n\n");
    println!("{}", string);
}

fn main(round_trips: usize, mode: &str) -> Result<i32, &'static str> {
    let input: Vec<String>;
    if mode == "real" {
        input = crate::utils::file_path_to_vec_of_strings_preserve_whitespace(FILE_PATH);
    } else if mode == "test" {
        input = crate::utils::file_path_to_vec_of_strings_preserve_whitespace(TEST_FILE_PATH);
    } else {
        panic!("unknown mode");
    }
    let height = input.len() as i32 - 2; // len of vertical dist
    let width = input[0].len() as i32 - 2; // len of horizontal dist
    let mut initial_winds: HashMap<(i32, i32), Dir> = HashMap::new();
    for (i_row, line) in input.iter().enumerate() {
        for (j_col, ch) in line.chars().enumerate() {
            let dir = match ch {
                '^' => Some(Dir::Up),
                '>' => Some(Dir::Right),
                'v' => Some(Dir::Down),
                '<' => Some(Dir::Left),
                '.' => None,
                '#' => None,
                _ => panic!("unexpected char in valley"),
            };
            if dir.is_some() {
                initial_winds.insert((i_row as i32, j_col as i32), dir.unwrap());
            }
        }
    }
    let valid_positions = get_valid_positions(height, width);
    let future_winds = get_future_winds(initial_winds, height, width);

    let mut total_time = 0;
    let point_a: (i32, i32) = (0, 1);
    let point_b: (i32, i32) = (height + 1, width);
    for (i, (start, end)) in vec![(point_a, point_b), (point_b, point_a), (point_a, point_b)]
        .iter()
        .enumerate()
    {
        if i == round_trips {
            break;
        }

        let mut visited_positions: HashSet<((i32, i32), i32)> = HashSet::new();
        // time, pos
        let mut nodes: ReverseSortedVec<(i32, (i32, i32))> = ReverseSortedVec::new();
        nodes.insert(Reverse((total_time, *start)));

        while nodes.len() > 0 {
            let (t, pos) = nodes.pop().unwrap().0;
            if visited_positions.contains(&(pos, t)) {
                continue;
            } else {
                visited_positions.insert((pos, t));
            }
            if pos == *end {
                total_time = t;
                break;
            }

            for possible_pos in vec![
                (pos.0, pos.1),
                (pos.0 - 1, pos.1),
                (pos.0 + 1, pos.1),
                (pos.0, pos.1 - 1),
                (pos.0, pos.1 + 1),
            ]
            .iter()
            {
                if !future_winds.contains_key(&(*possible_pos, t + 1))
                    && valid_positions.contains(possible_pos)
                {
                    nodes.insert(Reverse((t + 1, *possible_pos)));
                }
            }
        }
    }
    return Ok(total_time);
}

pub fn result_a() -> Result<i32, &'static str> {
    main(1, "real")
}

pub fn result_b() -> Result<i32, &'static str> {
    main(3, "real")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_is_correct() {
        let answer = result_a().unwrap();
        assert_eq!(answer, 247);
    }

    #[test]
    fn b_is_correct() {
        let answer = result_b().unwrap();
        assert_eq!(answer, 728);
    }
}
