use crate::utils;
use sorted_vec::SortedVec;
use std::collections::HashMap;
const FILE_PATH: &str = "inputs/day16.txt";
const TEST_FILE_PATH: &str = "inputs/day16_test.txt"; // a: 1651. b: 1707
const MODE: &str = "real"; // real or test

fn get_input() -> (
    HashMap<(char, char), Vec<(char, char)>>,
    HashMap<(char, char), i32>,
) {
    let input: Vec<Vec<String>>;
    if MODE == "real" {
        input = utils::vector_of_string_vectors_from_file(FILE_PATH);
    } else if MODE == "test" {
        input = utils::vector_of_string_vectors_from_file(TEST_FILE_PATH);
    } else {
        panic!("unimplemented mode");
    }
    // node name to flow rate and connected nodes
    let mut node_connections: HashMap<(char, char), Vec<(char, char)>> = HashMap::new();
    let mut node_flows: HashMap<(char, char), i32> = HashMap::new();

    for line in input.iter() {
        let node_name = &line[1];
        let node_name_char1: char = node_name.chars().next().unwrap();
        let node_name_char2: char = node_name.chars().nth(1).unwrap();
        let flow_rate: i32 = line[4]
            .trim_end_matches(';')
            .trim_start_matches(&['r', 'a', 't', 'e', '='])
            .parse()
            .unwrap();
        let mut connected_node_names: Vec<(char, char)> = Vec::new();
        for connected_name in 9..line.len() {
            let char1: char = line[connected_name].chars().next().unwrap();
            let char2: char = line[connected_name].chars().nth(1).unwrap();
            connected_node_names.push((char1, char2));
        }
        node_flows.insert((node_name_char1, node_name_char2), flow_rate);
        node_connections.insert((node_name_char1, node_name_char2), connected_node_names);
    }
    (node_connections, node_flows)
}

fn node_not_opened_in_either_path(
    node: (char, char),
    node_path_1: &Vec<(char, char)>,
    node_path_2: &Vec<(char, char)>,
) -> bool {
    for i in 1..node_path_1.len() {
        if node_path_1[i] == node && node_path_1[i - 1] == node {
            return false;
        }
    }
    for i in 1..node_path_2.len() {
        if node_path_2[i] == node && node_path_2[i - 1] == node {
            return false;
        }
    }
    true
}

fn last_node_not_opened_in_path(node_path: &Vec<(char, char)>) -> bool {
    for i in 1..node_path.len() {
        if node_path[i] == *node_path.last().unwrap()
            && node_path[i - 1] == *node_path.last().unwrap()
        {
            return false;
        }
    }
    true
}

#[allow(dead_code)]
fn print_char_path(char_path: &Vec<(char, char)>) -> () {
    for node in char_path.iter() {
        print!("{}{}-", node.0, node.1);
    }
    println!("");
}

/*
fn get_pruned_connections(
    node_connections: &mut HashMap<(char, char), Vec<(char, char)>>,
    node_flows: HashMap<(char, char), i32>,
) -> HashMap<(char, char), i32> {
    let mut connections_and_step_length: HashMap<(char, char), i32>;

    connections_and_step_length
}
*/

pub fn result_a() -> Result<i32, &'static str> {
    let (node_connections, node_flows) = get_input();
    let cutoff_length = 500;

    let mut total_flow: i32 = 0;
    for flow in node_flows.values() {
        total_flow += flow;
    }
    let max_released_volume = 29 * total_flow;
    let aa_connections = &node_connections[&('A', 'A')];

    // first number is the total blocked volume so far. The lower the better.
    // second number is the open flow. The higher the better.
    // third element, the vector, is the path taken.
    let mut paths_to_explore: SortedVec<(i32, i32, Vec<(char, char)>)> = SortedVec::new();
    for node in aa_connections.into_iter() {
        paths_to_explore.insert((total_flow, 0, vec![*node]));
    }

    for _ in 0..std::i32::MAX {
        let (total_blocked_volume, current_flow, node_path) = paths_to_explore.remove_index(0);
        if paths_to_explore.len() > cutoff_length {
            paths_to_explore.drain(cutoff_length..);
        }
        if node_path.len() == 29 {
            return Ok(max_released_volume - total_blocked_volume);
        }
        for node in &node_connections[node_path.last().unwrap()] {
            let mut node_path_plus_new_node = node_path.clone();
            node_path_plus_new_node.push(*node);
            paths_to_explore.insert((
                total_blocked_volume + (total_flow - current_flow),
                current_flow,
                node_path_plus_new_node,
            ));
        }
        if node_flows[node_path.last().unwrap()] > 0 && last_node_not_opened_in_path(&node_path) {
            let mut node_path_plus_new_node = node_path.clone();
            node_path_plus_new_node.push(*node_path.last().unwrap());
            let new_flow = current_flow + node_flows[node_path.last().unwrap()];
            paths_to_explore.insert((
                total_blocked_volume + (total_flow - new_flow),
                new_flow,
                node_path_plus_new_node,
            ));
        }
    }
    Err("did not run to completion")
}

pub fn result_b() -> Result<i32, &'static str> {
    let (node_connections, node_flows) = get_input();
    let cutoff_length = 40000;
    let ignore_zero_flow_paths_after_n_iter: i32 = 5;
    let time_until_collapse: usize = 26;

    let mut total_flow: i32 = 0;
    for flow in node_flows.values() {
        total_flow += flow;
    }
    let max_released_volume = (time_until_collapse as i32 - 1) * total_flow;

    // first number is the total blocked volume so far. The lower the better.
    // second number is the NEGATIVE open flow. The LOWER the better.
    // third element, the first vector, is the path taken of me.
    // fourth element, the second vector, is the path taken of me.
    let mut paths_to_explore: SortedVec<(i32, i32, Vec<(char, char)>, Vec<(char, char)>)> =
        SortedVec::new();
    let aa_connections = &node_connections[&('A', 'A')];
    let mut elephant_possibilities: Vec<(char, char)> = Vec::new();
    let mut my_possibilities: Vec<(char, char)> = Vec::new();
    for node in aa_connections.into_iter() {
        my_possibilities.push(*node);
        elephant_possibilities.push(*node);
    }
    for my_pos in my_possibilities.iter() {
        for ele_pos in elephant_possibilities.iter() {
            let my_path = vec![('A', 'A'), *my_pos];
            let ele_path = vec![('A', 'A'), *ele_pos];
            // starting node has no flow
            paths_to_explore.insert((total_flow, 0, my_path, ele_path));
        }
    }

    // flow points is the air volume that could have been released - what has been released
    for _ in 0..std::i32::MAX {
        let (total_blocked_volume, current_flow, ele_path, my_path) =
            paths_to_explore.remove_index(0);
        let path_len = my_path.len() as i32;
        if paths_to_explore.len() > cutoff_length {
            paths_to_explore.drain(cutoff_length..);
        }
        if my_path.len() == time_until_collapse {
            return Ok(max_released_volume - total_blocked_volume);
        }

        let mut elephant_possibilities_with_additional_flow: Vec<((char, char), i32)> = Vec::new();
        for node in &node_connections[ele_path.last().unwrap()] {
            elephant_possibilities_with_additional_flow.push((*node, 0));
        }
        if node_flows[ele_path.last().unwrap()] > 0
            && node_not_opened_in_either_path(*ele_path.last().unwrap(), &my_path, &ele_path)
        {
            elephant_possibilities_with_additional_flow.push((
                *ele_path.last().unwrap(),
                node_flows[ele_path.last().unwrap()],
            ));
        }

        let mut my_possibilities_with_additional_flow: Vec<((char, char), i32)> = Vec::new();
        for node in &node_connections[my_path.last().unwrap()] {
            my_possibilities_with_additional_flow.push((*node, 0));
        }
        if node_flows[my_path.last().unwrap()] > 0
            && node_not_opened_in_either_path(*my_path.last().unwrap(), &my_path, &ele_path)
        {
            my_possibilities_with_additional_flow.push((
                *my_path.last().unwrap(),
                node_flows[my_path.last().unwrap()],
            ));
        }

        for (my_pos, my_added_flow) in my_possibilities_with_additional_flow.iter() {
            for (ele_pos, ele_added_flow) in elephant_possibilities_with_additional_flow.iter() {
                // both I and the elephant can not open the same valve at the same time.
                if my_added_flow > &0 && my_added_flow == ele_added_flow {
                    continue;
                }
                let new_flow = current_flow - my_added_flow - ele_added_flow; // flow is kept
                                                                              // negative
                if path_len > ignore_zero_flow_paths_after_n_iter && new_flow == 0 {
                    continue;
                }
                let mut my_node_path_plus_new_node = my_path.clone();
                my_node_path_plus_new_node.push(*my_pos);
                let mut ele_node_path_plus_new_node = ele_path.clone();
                ele_node_path_plus_new_node.push(*ele_pos);

                paths_to_explore.insert((
                    total_blocked_volume + total_flow + new_flow,
                    new_flow,
                    ele_node_path_plus_new_node,
                    my_node_path_plus_new_node,
                ));
            }
        }
    }
    Err("did not run to completion")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result_a_is_correct() {
        let answer = result_a().unwrap();
        assert_eq!(answer, 1792);
    }

    #[test]
    fn result_b_is_correct() {
        let answer = result_b().unwrap();
        assert_eq!(answer, 2587);
    }
}
