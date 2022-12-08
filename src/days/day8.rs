use crate::utils;
use std::collections::HashSet;

const FILE_PATH: &str = "inputs/day8.txt";

fn print_visible_forest(visible_forest: &HashSet<(usize, usize)>) -> () {
    let side_length_of_forest: usize = 99;
    for i_row in 0..side_length_of_forest {
        // let j_col = 97;
        for j_col in 0..side_length_of_forest {
            match visible_forest.get(&(i_row, j_col)) {
                Some(_) => println!("({}, {})", i_row, j_col),
                None => (),
            }
        }
    }
}

/// Rows are i, starting from the top of the forest with i = 0
/// Columns are j, starting from the left side of the forest with j = 0
pub fn result_a() -> Result<i32, &'static str> {
    let forest: Vec<Vec<i32>> = utils::file_path_to_nr_matrix(FILE_PATH);
    let side_length_of_forest: usize = forest.len();
    // println!("{}", side_length_of_forest);
    let mut visible_forest: HashSet<(usize, usize)> = HashSet::new();
    for i_row in 0..side_length_of_forest {
        // from left to right
        let mut highest_seen_tree: i32 = -1;
        for j_col in 0..side_length_of_forest {
            if forest[i_row][j_col] > highest_seen_tree {
                visible_forest.insert((i_row, j_col));
                highest_seen_tree = forest[i_row][j_col];
            }
        }

        // from right to left
        let mut highest_seen_tree: i32 = -1;
        for j_col in (0..side_length_of_forest).rev() {
            if forest[i_row][j_col] > highest_seen_tree {
                visible_forest.insert((i_row, j_col));
                highest_seen_tree = forest[i_row][j_col];
            }
        }
    }

    for j_col in 0..side_length_of_forest {
        // from top to bottom
        let mut highest_seen_tree: i32 = -1;
        for i_row in 0..side_length_of_forest {
            if forest[i_row][j_col] > highest_seen_tree {
                visible_forest.insert((i_row, j_col));
                highest_seen_tree = forest[i_row][j_col];
            }
        }

        // from bottom to top
        let mut highest_seen_tree: i32 = -1;
        for i_row in (0..side_length_of_forest).rev() {
            if forest[i_row][j_col] > highest_seen_tree {
                visible_forest.insert((i_row, j_col));
                highest_seen_tree = forest[i_row][j_col];
            }
        }
    }
    // print_visible_forest(&visible_forest);
    // println!("{:?}", forest[97]);
    Ok(visible_forest.len() as i32)
}

pub fn result_b() -> Result<i32, &'static str> {
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
