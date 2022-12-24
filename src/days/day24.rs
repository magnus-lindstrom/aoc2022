use std::cmp::Reverse;
use sorted_vec::ReverseSortedVec;
use std::collections::{HashSet,HashMap};

const FILE_PATH: &str = "inputs/day24.txt";
//const FILE_PATH: &str = "inputs/day24_test.txt";
//const FILE_PATH: &str = "inputs/day24_scratch.txt";

#[derive(Eq, PartialEq)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

fn get_future_winds(initial_winds: HashMap<(i32, i32), Dir>, height: i32, width: i32)
-> HashMap<((i32, i32), i32), char> {
    let mut future_winds: HashMap<((i32, i32), i32), char> = HashMap::new();
    for t in 0..1000 {
        for wind_coordinate in initial_winds.keys() {
            match initial_winds[wind_coordinate] {
                Dir::Up => {
                    let mut row = wind_coordinate.0 - 1; // do not consider walls
                    row = (((row - t) % height) + height) % height;
                    if future_winds.contains_key(&((row+1, wind_coordinate.1), t)) {
                        let ch = match future_winds[&((row+1, wind_coordinate.1), t)] {
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
                    if future_winds.contains_key(&((wind_coordinate.0, col+1), t)) {
                        let ch = match future_winds[&((wind_coordinate.0, col+1), t)] {
                            '^' | '>' | 'v' | '<' => '2',
                            '2' => '3',
                            '3' => '4',
                            _ => panic!("too many winds"),
                        };
                        future_winds.insert(((wind_coordinate.0, col+1), t), ch);
                    } else {
                        future_winds.insert(((wind_coordinate.0, col+1), t), '>');
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
fn print_future_winds(future_winds: &HashMap<((i32, i32), i32), char>, height: i32, width: i32)
-> () {
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
    positions.insert((0,1));
    positions.insert((height+1, width));
    for row in 1..=height {
        for col in 1..=width {
            positions.insert((row, col));
        }
    }
    positions
}

fn impossible_to_beat_best(pos: (i32, i32), end: (i32, i32), t: i32, best_yet: i32) -> bool {
    let dist_to_end = 2 * crate::utils::manhattan_dist(pos, end);
    if t + dist_to_end as i32 > best_yet {
        true
    } else {
        false
    }
}

#[allow(dead_code)]
fn print_pos(pos: (i32, i32), t: i32, future_winds: &HashMap<((i32, i32), i32), char>, height: i32, width: i32)
-> () {
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
        if j_col == pos.1 && pos.0 == height+1 {
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

pub fn result_a() -> Result<i32, &'static str> {
    let input = crate::utils::file_path_to_vec_of_strings_preserve_whitespace(FILE_PATH);
    let height = input.len() as i32 - 2; // len of vertical dist
    let width = input[0].len() as i32 - 2; // len of horizontal dist
    let end: (i32, i32) = (height + 1, width);
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
    let mut visited_positions: HashSet<((i32, i32), i32)> = HashSet::new();
    let future_winds = get_future_winds(initial_winds, height, width);

    // dist to end, time, pos
    let mut nodes: ReverseSortedVec<(i32, (i32, i32))> = ReverseSortedVec::new();
    nodes.insert(Reverse((0, (0,1))));

    let mut best_yet: i32 = std::i32::MAX;
    while nodes.len() > 0 {
        let (t, pos) = nodes.pop().unwrap().0;
        if visited_positions.contains(&(pos, t)) {
            continue;
        } else {
            visited_positions.insert((pos, t));
        }
        // println!("nodes.len(): {}", nodes.len());
        // println!("{:?}", nodes);
        // println!("t: {}", t);
        if impossible_to_beat_best(pos, end, t, best_yet) {
            continue;
            //break;
        }
        //print_pos(pos, t, &future_winds, height, width);
        if pos == end {
            if t < best_yet {
                best_yet = t;
            }
            continue;
        }

        for possible_pos in vec![
            (pos.0, pos.1),
            (pos.0 - 1, pos.1),
            (pos.0 + 1, pos.1),
            (pos.0, pos.1 - 1),
            (pos.0, pos.1 + 1)
        ].iter() {
            if !future_winds.contains_key(&(*possible_pos, t+1))
                && valid_positions.contains(possible_pos) {
                nodes.insert(Reverse((t+1, *possible_pos)));
            }
        }
    }
    return Ok(best_yet)
}

pub fn result_b() -> Result<i32, &'static str> {
    let input = crate::utils::file_path_to_vec_of_strings_preserve_whitespace(FILE_PATH);
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
    let point_a: (i32, i32) = (0,1);
    let point_b: (i32, i32) = (height + 1, width);
    for (start, end) in vec![(point_a, point_b), (point_b, point_a), (point_a, point_b)].iter() {

        let mut visited_positions: HashSet<((i32, i32), i32)> = HashSet::new();
        // dist to end, time, pos
        let mut nodes: ReverseSortedVec<(i32, (i32, i32))> = ReverseSortedVec::new();
        nodes.insert(Reverse((total_time, *start)));

        let mut best_yet: i32 = 5000;
        while nodes.len() > 0 {
            let (t, pos) = nodes.pop().unwrap().0;
            if visited_positions.contains(&(pos, t)) {
                continue;
            } else {
                visited_positions.insert((pos, t));
            }
            if impossible_to_beat_best(pos, *end, t, best_yet) {
                continue;
            }
            if pos == *end {
                if t < best_yet {
                    best_yet = t;
                }
                continue;
            }

            for possible_pos in vec![
                (pos.0, pos.1),
                (pos.0 - 1, pos.1),
                (pos.0 + 1, pos.1),
                (pos.0, pos.1 - 1),
                (pos.0, pos.1 + 1)
            ].iter() {
                if !future_winds.contains_key(&(*possible_pos, t+1))
                    && valid_positions.contains(possible_pos) {
                    nodes.insert(Reverse((t+1, *possible_pos)));
                }
            }
        }
        total_time += best_yet - total_time;
    }
    return Ok(total_time)
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
