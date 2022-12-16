use crate::utils;
use sorted_vec::SortedVec;
use std::collections::HashMap;
// const FILE_PATH: &str = "inputs/day16.txt";
const FILE_PATH: &str = "inputs/day16_test.txt";

/// function to translate node strings, e.g. "AA" to a number
fn bigram_to_i32(bigram: &str) -> i32 {
    1000 * bigram.as_bytes()[0] as i32 + bigram.as_bytes()[1] as i32
}
fn last_node_not_opened_in_path(node_path: &Vec<i32>) -> bool {
    for i in 1..node_path.len() {
        if node_path[i] == *node_path.last().unwrap()
            && node_path[i - 1] == *node_path.last().unwrap()
        {
            return false;
        }
    }
    true
}

pub fn result_a() -> Result<i32, &'static str> {
    let input = utils::vector_of_string_vectors_from_file(FILE_PATH);
    // node name to flow rate and connected nodes
    let mut node_connections: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut node_flows: HashMap<i32, i32> = HashMap::new();
    let mut total_flow: i32 = 0;
    let cutoff_length = 50000;

    // first number is the total blocked volume so far. The lower the better.
    // second number is the open flow. The higher the better.
    // third element, the vector, is the path taken.
    let mut paths_to_explore: SortedVec<(i32, i32, Vec<i32>)> = SortedVec::new();
    for line in input.iter() {
        let node_name = &line[1];
        let flow_rate: i32 = line[4]
            .trim_end_matches(';')
            .trim_start_matches(&['r', 'a', 't', 'e', '='])
            .parse()
            .unwrap();
        let mut connected_node_names: Vec<i32> = Vec::new();
        for connected_name in 9..line.len() {
            connected_node_names.push(bigram_to_i32(line[connected_name].trim_end_matches(',')));
        }
        let node_name_code = bigram_to_i32(node_name);
        node_flows.insert(node_name_code, flow_rate);
        node_connections.insert(node_name_code, connected_node_names);
    }
    for flow in node_flows.values() {
        total_flow += flow;
    }
    let max_released_volume = 29 * total_flow;
    let aa_connections = &node_connections[&bigram_to_i32("AA")];
    for node in aa_connections.into_iter() {
        let new_path = vec![bigram_to_i32("AA"), *node];
        // starting node has no flow
        paths_to_explore.insert((total_flow, 0, new_path));
    }

    let mut max_iter = 5;
    // flow points is the air volume that could have been released - what has been released
    for _ in 0..std::i32::MAX {
        let (total_blocked_volume, current_flow, node_path) = paths_to_explore.remove_index(0);
        if paths_to_explore.len() > cutoff_length {
            paths_to_explore.drain(cutoff_length..);
        }
        let path_len = node_path.len();
        if path_len == max_iter {
            println!("path length: {}", path_len);
            max_iter += 5;
        }
        if node_path.len() == 30 {
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

#[allow(dead_code)]
fn naive_solution() -> Result<i32, &'static str> {
    let input = utils::vector_of_string_vectors_from_file(FILE_PATH);
    // node name to flow rate and connected nodes
    let mut node_connections: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut node_flows: HashMap<i32, i32> = HashMap::new();
    let mut total_flow: i32 = 0;

    // first number is the total blocked volume so far. The lower the better.
    // second number is the open flow. The higher the better.
    // third element, the vector, is the path taken.
    let mut paths_to_explore: SortedVec<(i32, i32, Vec<i32>)> = SortedVec::new();
    for line in input.iter() {
        let node_name = &line[1];
        let flow_rate: i32 = line[4]
            .trim_end_matches(';')
            .trim_start_matches(&['r', 'a', 't', 'e', '='])
            .parse()
            .unwrap();
        let mut connected_node_names: Vec<i32> = Vec::new();
        for connected_name in 9..line.len() {
            connected_node_names.push(bigram_to_i32(line[connected_name].trim_end_matches(',')));
        }
        let node_name_code = bigram_to_i32(node_name);
        node_flows.insert(node_name_code, flow_rate);
        node_connections.insert(node_name_code, connected_node_names);
    }
    for flow in node_flows.values() {
        total_flow += flow;
    }
    let max_released_volume = 29 * total_flow;
    let aa_connections = &node_connections[&bigram_to_i32("AA")];
    for node in aa_connections.into_iter() {
        let new_path = vec![bigram_to_i32("AA"), *node];
        // starting node has no flow
        paths_to_explore.insert((total_flow, 0, new_path));
    }

    let mut max_iter = 5;
    // flow points is the air volume that could have been released - what has been released
    for i in 0..std::i32::MAX {
        let (total_blocked_volume, current_flow, node_path) = paths_to_explore.remove_index(0);
        let path_len = node_path.len();
        if path_len == max_iter {
            println!("path length: {}. iteration: {}", path_len, i);
            max_iter += 1;
        }
        if node_path.len() == 30 {
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
    let input = utils::vector_of_string_vectors_from_file(FILE_PATH);
    // node name to flow rate and connected nodes
    let mut node_connections: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut node_flows: HashMap<i32, i32> = HashMap::new();
    let time_until_collapse: usize = 26;

    for line in input.iter() {
        let node_name = &line[1];
        let flow_rate: i32 = line[4]
            .trim_end_matches(';')
            .trim_start_matches(&['r', 'a', 't', 'e', '='])
            .parse()
            .unwrap();
        let mut connected_node_names: Vec<i32> = Vec::new();
        for connected_name in 9..line.len() {
            connected_node_names.push(bigram_to_i32(line[connected_name].trim_end_matches(',')));
        }
        let node_name_code = bigram_to_i32(node_name);
        node_flows.insert(node_name_code, flow_rate);
        node_connections.insert(node_name_code, connected_node_names);
    }

    let mut total_flow: i32 = 0;
    for flow in node_flows.values() {
        total_flow += flow;
    }
    let max_released_volume = (time_until_collapse as i32 - 1) * total_flow;

    // first number is the total blocked volume so far. The lower the better.
    // second number is the open flow. The higher the better.
    // third element, the first vector, is the path taken of me.
    // fourth element, the second vector, is the path taken of me.
    let mut paths_to_explore: SortedVec<(i32, i32, Vec<i32>, Vec<i32>)> = SortedVec::new();
    let aa_connections = &node_connections[&bigram_to_i32("AA")];
    let mut elephant_possibilities: Vec<i32> = Vec::new();
    let mut my_possibilities: Vec<i32> = Vec::new();
    for node in aa_connections.into_iter() {
        my_possibilities.push(*node);
        elephant_possibilities.push(*node);
    }
    for my_pos in my_possibilities.iter() {
        for ele_pos in elephant_possibilities.iter() {
            let my_path = vec![bigram_to_i32("AA"), *my_pos];
            let ele_path = vec![bigram_to_i32("AA"), *ele_pos];
            // starting node has no flow
            paths_to_explore.insert((total_flow, 0, my_path, ele_path));
        }
    }

    let mut max_iter = 5;
    // flow points is the air volume that could have been released - what has been released
    for _ in 0..std::i32::MAX {
        let (total_blocked_volume, current_flow, ele_path, my_path) =
            paths_to_explore.remove_index(0);
        if paths_to_explore.len() > 75000 {
            paths_to_explore.drain(75000..);
        }
        let path_len = my_path.len();
        if path_len == max_iter {
            println!("path length: {}", path_len);
            max_iter += 1;
        }
        if my_path.len() == time_until_collapse {
            return Ok(max_released_volume - total_blocked_volume);
        }

        let mut elephant_possibilities_with_additional_flow: Vec<(i32, i32)> = Vec::new();
        for node in &node_connections[ele_path.last().unwrap()] {
            elephant_possibilities_with_additional_flow.push((*node, 0));
        }
        if node_flows[ele_path.last().unwrap()] > 0 && last_node_not_opened_in_path(&ele_path) {
            elephant_possibilities_with_additional_flow.push((
                *ele_path.last().unwrap(),
                node_flows[ele_path.last().unwrap()],
            ));
        }

        let mut my_possibilities_with_additional_flow: Vec<(i32, i32)> = Vec::new();
        for node in &node_connections[my_path.last().unwrap()] {
            my_possibilities_with_additional_flow.push((*node, 0));
        }
        if node_flows[my_path.last().unwrap()] > 0 && last_node_not_opened_in_path(&my_path) {
            my_possibilities_with_additional_flow.push((
                *my_path.last().unwrap(),
                node_flows[my_path.last().unwrap()],
            ));
        }

        for (my_pos, my_added_flow) in my_possibilities_with_additional_flow.iter() {
            for (ele_pos, ele_added_flow) in elephant_possibilities_with_additional_flow.iter() {
                let mut my_node_path_plus_new_node = my_path.clone();
                my_node_path_plus_new_node.push(*my_pos);
                let mut ele_node_path_plus_new_node = ele_path.clone();
                ele_node_path_plus_new_node.push(*ele_pos);

                paths_to_explore.insert((
                    total_blocked_volume + total_flow
                        - current_flow
                        - my_added_flow
                        - ele_added_flow,
                    current_flow + my_added_flow + ele_added_flow,
                    ele_node_path_plus_new_node,
                    my_node_path_plus_new_node,
                ));
            }
        }
        println!("length of possible next steps: {}", paths_to_explore.len());
    }
    Err("did not run to completion")
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
