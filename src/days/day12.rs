use crate::utils;
use std::collections::HashMap;
const FILE_PATH: &str = "inputs/day12.txt";
const TEST_FILE_PATH: &str = "inputs/day12_test.txt";

#[derive(Debug)]
struct Position {
    position: (i32, i32),
    distance_from_start: i32,
    distance_to_end: i32,
}

fn dist_to_end(x: (i32, i32), end: (i32, i32)) -> i32 {
    let mut dist: i32 = 0;
    dist += (x.0 + end.0).abs();
    dist += (x.1 + end.1).abs();
    dist
}

fn get_best_node_index(possible_positions: &Vec<Position>) -> usize {
    let mut best_index: Option<usize> = None;
    let mut lowest_distance: i32 = std::i32::MAX;
    for (i_position, position) in possible_positions.iter().enumerate() {
        if position.distance_from_start + position.distance_to_end < lowest_distance {
            lowest_distance = position.distance_from_start + position.distance_to_end;
            best_index = Some(i_position);
        }
    }
    best_index.unwrap()
}

pub fn result_a() -> Result<i32, &'static str> {
    let input = utils::file_path_to_vec_of_char_vecs(TEST_FILE_PATH);

    let map_size: i32 = input.len() as i32;
    let max_height_diff: i32 = 1;

    let mut char_height_mapping: HashMap<char, i32> = HashMap::new();
    let mut i: i32 = 1;
    for ch in 'a'..='z' {
        char_height_mapping.insert(ch, i);
        i += 1;
    }
    char_height_mapping.insert('S', char_height_mapping[&'a']);
    char_height_mapping.insert('E', char_height_mapping[&'z']);

    let mut start_init: Option<(i32, i32)> = None;
    let mut end_init: Option<(i32, i32)> = None;

    let mut height_map: HashMap<(i32, i32), i32> = HashMap::new();
    for i_row in 0..input.len() {
        for j_col in 0..input[0].len() {
            height_map.insert(
                (i_row as i32, j_col as i32),
                char_height_mapping[&input[i_row][j_col]],
            );
            if input[i_row][j_col] == 'S' {
                start_init = Some((i_row as i32, j_col as i32));
            } else if input[i_row][j_col] == 'E' {
                end_init = Some((i_row as i32, j_col as i32));
            }
        }
    }
    let start: (i32, i32) = start_init.unwrap();
    let end: (i32, i32) = end_init.unwrap();

    let mut possible_steps: Vec<Position> = Vec::new();

    let mut positions_visited: Vec<Position> = Vec::new();
    positions_visited.push(Position {
        position: start,
        distance_from_start: 0,
        distance_to_end: dist_to_end(start, end),
    });

    let viable_steps: Vec<(i32, i32)> = vec![(0, -1), (0, 1), (-1, 0), (1, 0)];

    for _ in 0..3 {
        let current_position: &Position = positions_visited.last().unwrap();
        if current_position.position == end {
            return Ok(current_position.distance_from_start);
        }

        for step in viable_steps.iter() {
            let node: (i32, i32) = (
                current_position.position.0 + step.0,
                current_position.position.1 + step.1,
            );
            if node.0 < 0 || node.1 < 0 || node.0 >= map_size || node.1 >= map_size {
                continue;
            }
            let distance: i32 = current_position.distance_from_start + 1;
            // TODO: maybe being discarded due to height diff?
            let height_diff: i32 = height_map[&node] - height_map[&current_position.position];
            if height_diff > max_height_diff {
                continue;
            }
            possible_steps.push(Position {
                position: node,
                distance_from_start: distance,
                distance_to_end: dist_to_end(node, end),
            });
        }
        let best_node_index: usize = get_best_node_index(&possible_steps);
        let new_position = possible_steps.remove(best_node_index);
        println!("{:?}", &possible_steps);
        println!("{:?}", &new_position);
        positions_visited.push(new_position);
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
