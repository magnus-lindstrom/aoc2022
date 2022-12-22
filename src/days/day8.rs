use crate::utils;
use std::collections::{HashMap, HashSet};

const FILE_PATH: &str = "inputs/day8.txt";

#[allow(dead_code)]
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

fn get_visibility_hashmap() -> HashMap<(usize, usize), i32> {
    let mut map: HashMap<(usize, usize), i32> = HashMap::new();
    for i in 0..100 {
        for j in 0..100 {
            map.insert((i, j), 1);
        }
    }
    map
}

fn get_max_visibility(visibility_matrix: HashMap<(usize, usize), i32>) -> i32 {
    let mut max_visibility: i32 = 0;
    for i in 0..100 {
        for j in 0..100 {
            let vis = visibility_matrix[&(i, j)];
            if vis > max_visibility {
                max_visibility = vis;
            }
        }
    }
    max_visibility
}

/// Rows are i, starting from the top of the forest with i = 0
/// Columns are j, starting from the left side of the forest with j = 0
pub fn result_a() -> Result<i32, &'static str> {
    let forest: Vec<Vec<i32>> = utils::file_path_to_nr_matrix(FILE_PATH);
    let side_length_of_forest: usize = forest.len();
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
    let forest: Vec<Vec<usize>> = utils::file_path_to_nr_matrix(FILE_PATH);
    let side_length_of_forest: usize = forest.len();
    // println!("{}", side_length_of_forest);
    let mut visibility: HashMap<(usize, usize), i32> = get_visibility_hashmap();
    let mut lookbehind: Vec<i32>;
    let mut tree_height: i32;
    for i_row in 0..side_length_of_forest {
        // from left to right
        lookbehind = vec![0; 10];
        for j_col in 0..side_length_of_forest {
            tree_height = forest[i_row][j_col] as i32;
            visibility
                .entry((i_row, j_col))
                .and_modify(|e| *e *= lookbehind[tree_height as usize]);
            for height in 0..10 {
                if height <= tree_height.try_into().unwrap() {
                    lookbehind[height] = 1;
                } else {
                    lookbehind[height] += 1;
                }
            }
        }

        // from right to left
        lookbehind = vec![0; 10];
        for j_col in (0..side_length_of_forest).rev() {
            tree_height = forest[i_row][j_col] as i32;
            visibility
                .entry((i_row, j_col))
                .and_modify(|e| *e *= lookbehind[tree_height as usize]);
            for height in 0..10 {
                if height <= tree_height.try_into().unwrap() {
                    lookbehind[height] = 1;
                } else {
                    lookbehind[height] += 1;
                }
            }
        }
    }

    for j_col in 0..side_length_of_forest {
        // from top to bottom
        lookbehind = vec![0; 10];
        for i_row in 0..side_length_of_forest {
            tree_height = forest[i_row][j_col] as i32;
            visibility
                .entry((i_row, j_col))
                .and_modify(|e| *e *= lookbehind[tree_height as usize]);
            for height in 0..10 {
                if height <= tree_height.try_into().unwrap() {
                    lookbehind[height] = 1;
                } else {
                    lookbehind[height] += 1;
                }
            }
        }

        // from bottom to top
        lookbehind = vec![0; 10];
        for i_row in (0..side_length_of_forest).rev() {
            tree_height = forest[i_row][j_col] as i32;
            visibility
                .entry((i_row, j_col))
                .and_modify(|e| *e *= lookbehind[tree_height as usize]);
            for height in 0..10 {
                if height <= tree_height.try_into().unwrap() {
                    lookbehind[height] = 1;
                } else {
                    lookbehind[height] += 1;
                }
            }
        }
    }
    let max_visibility = get_max_visibility(visibility);
    Ok(max_visibility)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_is_correct() {
        let answer = result_a().unwrap();
        assert_eq!(answer, 1719);
    }

    #[test]
    fn b_is_correct() {
        let answer = result_b().unwrap();
        assert_eq!(answer, 590824);
    }
}
