use crate::utils;
use sorted_vec::SortedVec;
use std::collections::HashMap;
use std::time::Instant;
const FILE_PATH: &str = "inputs/day16.txt";
//const FILE_PATH: &str = "inputs/day16_test.txt";

/// function to translate node strings, e.g. "AA" to a number
fn bigram_to_i32(bigram: &str) -> i32 {
    1000 * bigram.as_bytes()[0] as i32 + bigram.as_bytes()[1] as i32
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
fn print_char_path(char_path: &Vec<(char, char)>) -> () {
    for node in char_path.iter() {
        print!("{}{}-", node.0, node.1);
    }
    println!("");
}

pub fn result_b() -> Result<i32, &'static str> {
    let now = Instant::now();
    let input = utils::vector_of_string_vectors_from_file(FILE_PATH);
    // node name to flow rate and connected nodes
    let mut node_connections: HashMap<(char, char), Vec<(char, char)>> = HashMap::new();
    let mut node_flows: HashMap<(char, char), i32> = HashMap::new();
    let cutoff_length = 50000;
    let time_until_collapse: usize = 26;

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

    let mut total_flow: i32 = 0;
    for flow in node_flows.values() {
        total_flow += flow;
    }
    let max_released_volume = (time_until_collapse as i32 - 1) * total_flow;
    println!(
        "total flow: {}, max released volume: {}",
        total_flow, max_released_volume
    );

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

    /*
    let mut test: SortedVec<(i32, i32, i32)> = SortedVec::new();
    test.insert((1, -0, 0));
    test.insert((0, 1, 0));
    test.insert((1, -1, 0));
    test.insert((0, 1, 1));
    println!("test: {:?}", test);
    return Ok(0);
    */

    let mut max_iter: i32 = 5;
    // flow points is the air volume that could have been released - what has been released
    for _ in 0..std::i32::MAX {
        let (total_blocked_volume, current_flow, ele_path, my_path) =
            paths_to_explore.remove_index(0);
        let path_len = my_path.len() as i32;
        if path_len == max_iter {
            println!("Time elapsed: {:.1}min.", now.elapsed().as_secs_f32() / 60.);
            println!(
                "path length: {}, paths_to_explore: {}",
                path_len,
                paths_to_explore.len()
            );
            println!(
                "0:: ({},{}), 1:: ({},{}), 2:: ({},{}), n:: ({},{}) max val: ({},{})",
                paths_to_explore[0].0,
                paths_to_explore[0].1,
                paths_to_explore[1].0,
                paths_to_explore[1].1,
                paths_to_explore[2].0,
                paths_to_explore[2].1,
                paths_to_explore[20].0,
                paths_to_explore[20].1,
                paths_to_explore.last().unwrap().0,
                paths_to_explore.last().unwrap().1
            );
            max_iter += 1;
            print!("ele path: ");
            print_char_path(&ele_path);
            print!("my  path: ");
            print_char_path(&my_path);
            println!("");
        }
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
                if path_len > 5 && new_flow == 0 {
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
        // println!("length of possible next steps: {}", paths_to_explore.len());
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
