use crate::utils;
use std::collections::HashMap;
const FILE_PATH: &str = "inputs/day12.txt";
const TEST_FILE_PATH: &str = "inputs/day12_test.txt";

fn dist_to_end(from: (usize, usize), end: (usize, usize)) -> usize {
    let mut dist: usize = 0;
    dist += from.0.abs_diff(end.0);
    dist += from.1.abs_diff(end.1);
    dist
}

fn get_best_node_so_far(distance_to_goal_map: &HashMap<(usize, usize), usize>) -> (usize, usize) {
    *distance_to_goal_map
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k)
        .unwrap()
}

pub fn result_a() -> Result<usize, &'static str> {
    let input = utils::file_path_to_vec_of_char_vecs(TEST_FILE_PATH);

    let map_size: usize = input.len();

    let mut char_height_mapping: HashMap<char, i32> = HashMap::new();
    let mut i: i32 = 1;
    for ch in 'a'..='z' {
        char_height_mapping.insert(ch, i);
        i += 1;
    }
    char_height_mapping.insert('S', char_height_mapping[&'a']);
    char_height_mapping.insert('E', char_height_mapping[&'z']);

    let mut start_init: Option<(usize, usize)> = None;
    let mut end_init: Option<(usize, usize)> = None;

    let mut height_map: HashMap<(usize, usize), i32> = HashMap::new();
    for i_row in 0..input.len() {
        for j_col in 0..input[0].len() {
            height_map.insert((i_row, j_col), char_height_mapping[&input[i_row][j_col]]);
            if input[i_row][j_col] == 'S' {
                start_init = Some((i_row, j_col));
            } else if input[i_row][j_col] == 'E' {
                end_init = Some((i_row, j_col));
            }
        }
    }
    let start: (usize, usize) = start_init.unwrap();
    let end: (usize, usize) = end_init.unwrap();

    let mut distance_to_goal_map: HashMap<(usize, usize), usize> = HashMap::new();

    let current_node: (usize, usize) = start;
    let current_distance: usize = 0;

    distance_to_goal_map.insert(
        current_node,
        current_distance + dist_to_end(current_node, end),
    );
    let viable_steps: Vec<(i32, i32)> = vec![(0, -1), (0, 1), (-1, 0), (1, 0)];

    for _ in 0..std::i32::MAX {
        if distance_to_goal_map[&current_node] == 0 {
            return Ok(current_distance);
        }

        for step in viable_steps.iter() {
            let node: (usize, usize) = (
                current_node.0 + step.0 as usize,
                current_node.1 + step.1 as usize,
            );
            if node.0 >= map_size || node.1 >= map_size {
                continue;
            }
            distance_to_goal_map.insert(node, current_distance + dist_to_end(current_node, end));
        }
        let best_node = get_best_node_so_far(&distance_to_goal_map);
        if best_node == current_node {
            return Err("Did not find the end");
        }
    }
    Err("ran out of iterations")
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
