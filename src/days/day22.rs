use std::collections::HashMap;
const TEST_FILE_PATH: &str = "inputs/day22_test.txt";
const FILE_PATH: &str = "inputs/day22.txt";
const SETTING: &str = "test";

#[derive(Debug, Hash, Eq, PartialEq)]
enum Tile {
    Nothing,
    Empty,
    Wall,
}

#[derive(Debug, Eq, PartialEq)]
enum Facing {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
struct Position {
    tile: (i64, i64),
    facing: Facing,
}
impl Position {
    fn turn_right(&mut self) -> () {
        if self.facing == Facing::Up {
            self.facing = Facing::Right;
        } else if self.facing == Facing::Right {
            self.facing = Facing::Down;
        } else if self.facing == Facing::Down {
            self.facing = Facing::Left;
        } else if self.facing == Facing::Left {
            self.facing = Facing::Up;
        }
    }

    fn turn_left(&mut self) -> () {
        if self.facing == Facing::Up {
            self.facing = Facing::Left;
        } else if self.facing == Facing::Left {
            self.facing = Facing::Down;
        } else if self.facing == Facing::Down {
            self.facing = Facing::Right;
        } else if self.facing == Facing::Right {
            self.facing = Facing::Up;
        }
    }

    fn take_step_return_result(
        &mut self,
        map: &HashMap<(i64, i64), Tile>,
        row_bounds: &HashMap<i64, (i64, i64)>,
        col_bounds: &HashMap<i64, (i64, i64)>,
    ) -> bool {
        let mut next_tile: Option<(i64, i64)> = None;
        if self.facing == Facing::Up {
            if self.tile.0 == col_bounds[&self.tile.1].0 {
                next_tile = Some((col_bounds[&self.tile.1].1, self.tile.1));
            } else {
                next_tile = Some((self.tile.0 - 1, self.tile.1));
            }
        } else if self.facing == Facing::Down {
            if self.tile.0 == col_bounds[&self.tile.1].1 {
                next_tile = Some((col_bounds[&self.tile.1].0, self.tile.1));
            } else {
                next_tile = Some((self.tile.0 + 1, self.tile.1));
            }
        } else if self.facing == Facing::Right {
            if self.tile.1 == row_bounds[&self.tile.0].1 {
                next_tile = Some((self.tile.0, row_bounds[&self.tile.0].0));
            } else {
                next_tile = Some((self.tile.0, self.tile.1 + 1));
            }
        } else if self.facing == Facing::Left {
            if self.tile.1 == row_bounds[&self.tile.0].0 {
                next_tile = Some((self.tile.0, row_bounds[&self.tile.0].1));
            } else {
                next_tile = Some((self.tile.0, self.tile.1 - 1));
            }
        }

        if map[&next_tile.unwrap()] == Tile::Wall {
            return false;
        } else if map[&next_tile.unwrap()] == Tile::Empty {
            self.tile = next_tile.unwrap();
        } else {
            panic!("tried to walk to a 'nothing' tile");
        }
        true
    }
}

fn get_starting_column(map: &HashMap<(i64, i64), Tile>) -> i64 {
    for j in 1..std::i64::MAX {
        if map[&(1, j)] == Tile::Empty {
            return j;
        }
    }
    panic!("did not find starting pos");
}

fn get_map_plus_path_plus_map_bounds() -> (
    HashMap<(i64, i64), Tile>,
    Vec<String>,
    HashMap<i64, (i64, i64)>,
    HashMap<i64, (i64, i64)>,
    i64,
    i64,
) {
    let input: Vec<String>;
    if SETTING == "real" {
        input = crate::utils::file_path_to_vec_of_strings_preserve_whitespace(FILE_PATH);
    } else {
        input = crate::utils::file_path_to_vec_of_strings_preserve_whitespace(TEST_FILE_PATH);
    }
    let mut map: HashMap<(i64, i64), Tile> = HashMap::new();
    let mut row_bounds: HashMap<i64, (i64, i64)> = HashMap::new();
    let mut col_bounds: HashMap<i64, (i64, i64)> = HashMap::new();
    let mut nr_rows = 0;
    let mut nr_cols = 0;
    let mut path_index: Option<usize> = None;
    for (i_line, line) in input.iter().enumerate() {
        if line.is_empty() {
            path_index = Some(i_line + 1);
            break;
        }
        nr_rows += 1;
        let mut row_left_bound: Option<i64> = None;
        let mut row_right_bound: i64 = std::i64::MAX;
        for (j_ch, ch) in line.chars().enumerate() {
            if j_ch + 1 > nr_cols {
                nr_cols = j_ch + 1;
            }
            if ch == ' ' {
                map.insert((i_line as i64 + 1, j_ch as i64 + 1), Tile::Nothing);
            } else {
                if row_left_bound == None {
                    row_left_bound = Some(j_ch as i64 + 1);
                }
                row_right_bound = j_ch as i64 + 1;
                if ch == '.' {
                    map.insert((i_line as i64 + 1, j_ch as i64 + 1), Tile::Empty);
                } else if ch == '#' {
                    map.insert((i_line as i64 + 1, j_ch as i64 + 1), Tile::Wall);
                } else {
                    panic!("unexpected tile read");
                }
            }
        }
        row_bounds.insert(
            i_line as i64 + 1,
            (row_left_bound.unwrap(), row_right_bound),
        );
    }

    for j_col in 0..nr_cols {
        let mut col_upper_bound: Option<i64> = None;
        let mut col_lower_bound: i64 = 0;
        for i_row in 0..nr_rows {
            if map.contains_key(&(i_row as i64 + 1, j_col as i64 + 1))
                && map[&(i_row as i64 + 1, j_col as i64 + 1)] != Tile::Nothing
            {
                if col_upper_bound == None {
                    col_upper_bound = Some(i_row + 1);
                }
                col_lower_bound = i_row + 1;
            }
        }
        col_bounds.insert(
            j_col as i64 + 1,
            (col_upper_bound.unwrap(), col_lower_bound),
        );
    }

    let path: &str = &input[path_index.unwrap()];

    let mut path_vec: Vec<String> = Vec::new();
    let mut i = 0;
    while i < path.len() {
        let i_ch = path.chars().nth(i).unwrap();
        if i_ch != 'R' && i_ch != 'L' {
            for j in (i + 1)..path.len() {
                let j_ch = path.chars().nth(j).unwrap();
                if j_ch == 'R' || j_ch == 'L' {
                    path_vec.push(path[i..j].to_string());
                    i = j - 1;
                    break;
                } else if j == path.len() - 1 {
                    path_vec.push(path[i..].to_string());
                    i = j - 1;
                    break;
                }
            }
        } else {
            path_vec.push(path.chars().nth(i).unwrap().to_string());
        }
        i += 1;
    }
    (
        map,
        path_vec,
        row_bounds,
        col_bounds,
        nr_rows,
        nr_cols as i64,
    )
}

fn draw_pos_in_map(
    map: &HashMap<(i64, i64), Tile>,
    row_bounds: &HashMap<i64, (i64, i64)>,
    nr_rows: i64,
    position: &Position,
) -> () {
    let position_ch = match position.facing {
        Facing::Up => '^',
        Facing::Right => '>',
        Facing::Down => 'v',
        Facing::Left => '<',
    };
    let mut string: String = "".to_string();
    for i_row in 1..=nr_rows {
        for j_col in 1..=row_bounds[&i_row].1 {
            if position.tile == (i_row, j_col) {
                string.push(position_ch);
            } else if map[&(i_row, j_col)] == Tile::Nothing {
                string.push(' ');
            } else if map[&(i_row, j_col)] == Tile::Empty {
                string.push('.');
            } else if map[&(i_row, j_col)] == Tile::Wall {
                string.push('#');
            } else {
            }
        }
        string.push('\n');
    }
    println!("{}", string);
}

pub fn result_a() -> Result<i64, &'static str> {
    let (map, path_vec, row_bounds, col_bounds, nr_rows, _nr_cols) =
        get_map_plus_path_plus_map_bounds();
    let mut position: Position = Position {
        tile: (1, get_starting_column(&map)),
        facing: Facing::Right,
    };
    //draw_pos_in_map(&map, &row_bounds, nr_rows, &position);

    for instruction in path_vec.iter() {
        println!("{}", instruction);
        if instruction == "R" {
            position.turn_right();
        } else if instruction == "L" {
            position.turn_left();
        } else {
            let nr_steps: i64 = instruction.parse().unwrap();
            for _ in 0..nr_steps {
                let successful = position.take_step_return_result(&map, &row_bounds, &col_bounds);
                if !successful {
                    break;
                }
            }
        }
        //draw_pos_in_map(&map, &row_bounds, nr_rows, &position);
    }

    let facing_value = match position.facing {
        Facing::Right => 0,
        Facing::Down => 1,
        Facing::Left => 2,
        Facing::Up => 3,
    };

    println!("final position: {:?}", position.tile);
    Ok(position.tile.0 * 1000 + position.tile.1 * 4 + facing_value)
}

pub fn result_b() -> Result<i64, &'static str> {
    Ok(0)
}

/*
#[cfg(test)]
mod tests {
use super::*;

#[test]
fn result_a_is_correct() {
let answer = result_a().unwrap();
assert_eq!(answer, 0);
}

#[test]
fn result_b_is_correct() {
let answer = result_b().unwrap();
assert_eq!(answer, 0);
}
}
*/
