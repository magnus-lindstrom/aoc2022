use crate::utils;
use std::collections::{HashMap, HashSet};
const FILE_PATH: &str = "inputs/day12.txt";

#[derive(Debug)]
struct Position {
    position: (i32, i32),
    distance_from_start: i32,
    distance_to_end: i32,
}

fn dist_to_end(x: (i32, i32), end: (i32, i32)) -> i32 {
    let mut dist: i32 = 0;
    dist += (x.0 - end.0).abs();
    dist += (x.1 - end.1).abs();
    dist
}

fn get_best_node_index(possible_positions: &Vec<Position>) -> usize {
    let mut best_index: Option<usize> = None;
    let mut lowest_distance: i32 = std::i32::MAX;
    // print!("positions to choose from: ");
    for (i_position, position) in possible_positions.iter().enumerate() {
        // print!("{:?} ", position.position);
        if position.distance_from_start + position.distance_to_end < lowest_distance {
            lowest_distance = position.distance_from_start + position.distance_to_end;
            best_index = Some(i_position);
        }
    }
    // println!("");
    best_index.unwrap()
}

/// A* algorithm
pub fn result_a() -> Result<i32, &'static str> {
    let input = utils::file_path_to_vec_of_char_vecs(FILE_PATH);

    let map_size: (i32, i32) = (input[0].len() as i32, input.len() as i32);
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
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            height_map.insert((x as i32, y as i32), char_height_mapping[&input[y][x]]);
            if input[y][x] == 'S' {
                start_init = Some((x as i32, y as i32));
            } else if input[y][x] == 'E' {
                end_init = Some((x as i32, y as i32));
            }
        }
    }
    let start: (i32, i32) = start_init.unwrap();
    let end: (i32, i32) = end_init.unwrap();

    let mut possible_steps: Vec<Position> = Vec::new();
    let mut been_there: HashSet<(i32, i32)> = HashSet::new();
    let mut is_in_possible_steps: HashSet<(i32, i32)> = HashSet::new();

    let mut positions_visited: Vec<Position> = Vec::new();
    positions_visited.push(Position {
        position: start,
        distance_from_start: 0,
        distance_to_end: dist_to_end(start, end),
    });
    been_there.insert(start);

    let viable_steps: Vec<(i32, i32)> = vec![(0, -1), (0, 1), (-1, 0), (1, 0)];

    for _ in 0..std::i32::MAX {
        let current_position: &Position = positions_visited.last().unwrap();
        if current_position.position == end {
            return Ok(current_position.distance_from_start);
        }

        for step in viable_steps.iter() {
            let node: (i32, i32) = (
                current_position.position.0 + step.0,
                current_position.position.1 + step.1,
            );
            if been_there.contains(&node)
                || is_in_possible_steps.contains(&node)
                || node.0 < 0
                || node.1 < 0
                || node.0 >= map_size.0
                || node.1 >= map_size.1
            {
                continue;
            }
            let height_diff: i32 = height_map[&node] - height_map[&current_position.position];
            if height_diff > max_height_diff {
                continue;
            }
            let distance_from_start: i32 = current_position.distance_from_start + 1;
            // TODO: maybe being discarded due to height diff?
            possible_steps.push(Position {
                position: node,
                distance_from_start,
                distance_to_end: dist_to_end(node, end),
            });
            is_in_possible_steps.insert(node);
        }
        let best_node_index: usize = get_best_node_index(&possible_steps);
        let new_position = possible_steps.remove(best_node_index);
        been_there.insert(new_position.position);
        positions_visited.push(new_position);
    }
    Err("ran out of iterations")
}

/// Reverse the search direction here, from E to any a
/// The heuristic falls off now, just set it to 0 to ignore that part (instead of rewriting)
pub fn result_b() -> Result<i32, &'static str> {
    let mut input = utils::file_path_to_vec_of_char_vecs(FILE_PATH);

    let map_size: (i32, i32) = (input[0].len() as i32, input.len() as i32);
    let max_height_diff: i32 = -1;

    let mut char_height_mapping: HashMap<char, i32> = HashMap::new();
    let mut i: i32 = 1;
    for ch in 'a'..='z' {
        char_height_mapping.insert(ch, i);
        i += 1;
    }
    char_height_mapping.insert('S', char_height_mapping[&'a']);
    char_height_mapping.insert('E', char_height_mapping[&'z']);

    let mut start_init: Option<(i32, i32)> = None;

    let mut height_map: HashMap<(i32, i32), i32> = HashMap::new();
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            height_map.insert((x as i32, y as i32), char_height_mapping[&input[y][x]]);
            if input[y][x] == 'E' {
                start_init = Some((x as i32, y as i32));
            } else if input[y][x] == 'S' {
                input[y][x] = 'a';
            }
        }
    }
    let start: (i32, i32) = start_init.unwrap();

    let mut possible_steps: Vec<Position> = Vec::new();
    let mut been_there: HashSet<(i32, i32)> = HashSet::new();
    let mut is_in_possible_steps: HashSet<(i32, i32)> = HashSet::new();

    let mut positions_visited: Vec<Position> = Vec::new();
    positions_visited.push(Position {
        position: start,
        distance_from_start: 0,
        distance_to_end: 0,
    });
    been_there.insert(start);

    let viable_steps: Vec<(i32, i32)> = vec![(0, -1), (0, 1), (-1, 0), (1, 0)];

    for _ in 0..std::i32::MAX {
        let current_position: &Position = positions_visited.last().unwrap();
        if input[current_position.position.1 as usize][current_position.position.0 as usize] == 'a'
        {
            return Ok(current_position.distance_from_start);
        }

        for step in viable_steps.iter() {
            let node: (i32, i32) = (
                current_position.position.0 + step.0,
                current_position.position.1 + step.1,
            );
            if been_there.contains(&node)
                || is_in_possible_steps.contains(&node)
                || node.0 < 0
                || node.1 < 0
                || node.0 >= map_size.0
                || node.1 >= map_size.1
            {
                continue;
            }
            let height_diff: i32 = height_map[&node] - height_map[&current_position.position];
            if height_diff < max_height_diff {
                continue;
            }
            let distance_from_start: i32 = current_position.distance_from_start + 1;
            // TODO: maybe being discarded due to height diff?
            possible_steps.push(Position {
                position: node,
                distance_from_start,
                distance_to_end: 0,
            });
            is_in_possible_steps.insert(node);
        }
        let best_node_index: usize = get_best_node_index(&possible_steps);
        let new_position = possible_steps.remove(best_node_index);
        been_there.insert(new_position.position);
        positions_visited.push(new_position);
    }
    Err("ran out of iterations")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_is_correct() {
        let answer = result_a().unwrap();
        assert_eq!(answer, 504);
    }

    #[test]
    fn b_is_correct() {
        let answer = result_b().unwrap();
        assert_eq!(answer, 500);
    }
}
