use crate::utils;
use sorted_vec::SortedVec;
use std::collections::{HashMap, HashSet};
const FILE_PATH: &str = "inputs/day16.txt";
const TEST_FILE_PATH: &str = "inputs/day16_test.txt"; // a: 1651. b: 1707
const MODE: &str = "real"; // real or test

struct Node {
    released_pressure: i32,
    open_flow: i32,
    opened_valves: HashSet<(char, char)>,
    room: (char, char),
    prev_rooms: Vec<(char, char)>,
    time_left: i32,
}

struct TwinNode {
    released_pressure: i32,
    open_flow: i32,
    opened_valves: HashSet<(char, char)>,
    me_room: (char, char),
    me_prev_room: (char, char),
    ele_room: (char, char),
    ele_prev_room: (char, char),
    time_left: i32,
}

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
        if flow_rate > 0 {
            node_flows.insert((node_name_char1, node_name_char2), flow_rate);
        }
        node_connections.insert((node_name_char1, node_name_char2), connected_node_names);
    }
    (node_connections, node_flows)
}

#[allow(dead_code)]
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

fn impossible_to_beat_best_b(
    node: &TwinNode,
    max_released_pressure: i32,
    room_flows: &HashMap<(char, char), i32>,
) -> bool {
    let mut time_left = node.time_left;
    let mut open_flow = node.open_flow;
    let mut released_pressure = node.released_pressure;
    // Not putting the flows in increasing order here, so not strictly true if deemed impossible.
    // But the nodes with flows are not next to each other, so it still works.
    let mut flows_left_to_open: Vec<i32> = Vec::new();
    for room in room_flows.keys() {
        if !node.opened_valves.contains(room) {
            flows_left_to_open.push(room_flows[room]);
        }
    }

    let mut me_opened_room = false;
    let mut ele_opened_room = false;
    while time_left > 0 {
        released_pressure += open_flow;

        if !ele_opened_room && flows_left_to_open.len() > 0 {
            open_flow += flows_left_to_open.pop().unwrap();
            ele_opened_room = true;
        } else {
            ele_opened_room = false;
        }
        if !me_opened_room && flows_left_to_open.len() > 0 {
            open_flow += flows_left_to_open.pop().unwrap();
            me_opened_room = true;
        } else {
            me_opened_room = false;
        }
        time_left -= 1;
    }
    if released_pressure < max_released_pressure {
        true
    } else {
        false
    }
}

fn impossible_to_beat_best_a(
    node: &Node,
    max_released_pressure: i32,
    room_flows: &HashMap<(char, char), i32>,
) -> bool {
    let mut time_left = node.time_left;
    let mut open_flow = node.open_flow;
    let mut released_pressure = node.released_pressure;
    let mut flows_left_to_open: SortedVec<i32> = SortedVec::new();
    for room in room_flows.keys() {
        if !node.opened_valves.contains(room) {
            flows_left_to_open.insert(room_flows[room]);
        }
    }

    let mut opened_room = false;
    while time_left > 0 {
        released_pressure += open_flow;

        if !opened_room && flows_left_to_open.len() > 0 {
            open_flow += flows_left_to_open.pop().unwrap();
            opened_room = true;
        } else {
            opened_room = false;
        }
        time_left -= 1;
    }
    if released_pressure < max_released_pressure {
        true
    } else {
        false
    }
}

#[allow(dead_code)]
fn pprint_prev_rooms(prev_rooms: Vec<(char, char)>) -> String {
    let mut string: String = "".to_string();
    string.push(prev_rooms[0].0);
    string.push(prev_rooms[0].1);
    for room in prev_rooms[1..].iter() {
        string.push_str("-");
        string.push(room.0);
        string.push(room.1);
    }
    string
}

pub fn result_a() -> Result<i32, &'static str> {
    let (room_connections, room_flows) = get_input();
    let mut max_released_pressure = std::i32::MIN;
    let max_time: i32 = 30;

    let aa_connections = &room_connections[&('A', 'A')];

    let mut nodes_to_explore: Vec<Node> = Vec::new();
    for node in aa_connections.into_iter() {
        nodes_to_explore.push(Node {
            released_pressure: 0,
            open_flow: 0,
            opened_valves: HashSet::new(),
            room: *node,
            prev_rooms: vec![('A', 'A')],
            time_left: max_time - 1, // first minute spent moving to this node
        });
    }

    while nodes_to_explore.len() > 0 {
        let node = nodes_to_explore.pop().unwrap();
        if node.time_left == 0 {
            if node.released_pressure > max_released_pressure {
                max_released_pressure = node.released_pressure;
            }
            continue;
        }

        if impossible_to_beat_best_a(&node, max_released_pressure, &room_flows) {
            continue;
        }

        for connected_room in &room_connections[&node.room] {
            if connected_room == node.prev_rooms.last().unwrap() {
                continue;
            }
            let mut prev_rooms = node.prev_rooms.clone();
            prev_rooms.push(node.room);
            nodes_to_explore.push(Node {
                released_pressure: node.released_pressure + node.open_flow,
                open_flow: node.open_flow,
                opened_valves: node.opened_valves.clone(),
                room: *connected_room,
                prev_rooms,
                time_left: node.time_left - 1,
            });
        }

        // add node with current room opened
        if !node.opened_valves.contains(&node.room) && room_flows.contains_key(&node.room) {
            let mut new_opened_valves = node.opened_valves.clone();
            new_opened_valves.insert(node.room);
            let mut prev_rooms = node.prev_rooms.clone();
            prev_rooms.push(node.room);
            nodes_to_explore.push(Node {
                released_pressure: node.released_pressure + node.open_flow,
                open_flow: node.open_flow + room_flows[&node.room],
                opened_valves: new_opened_valves,
                room: node.room,
                prev_rooms,
                time_left: node.time_left - 1,
            });
        }
    }
    Ok(max_released_pressure)
}

pub fn result_b() -> Result<i32, &'static str> {
    let (room_connections, room_flows) = get_input();
    let mut max_released_pressure = std::i32::MIN;
    let max_time: i32 = 26;

    let aa_connections = &room_connections[&('A', 'A')];

    let mut nodes_to_explore: Vec<TwinNode> = Vec::new();
    for me_node in aa_connections.iter() {
        for ele_node in aa_connections.iter() {
            nodes_to_explore.push(TwinNode {
                released_pressure: 0,
                open_flow: 0,
                opened_valves: HashSet::new(),
                me_room: *me_node,
                me_prev_room: ('A', 'A'),
                ele_room: *ele_node,
                ele_prev_room: ('A', 'A'),
                time_left: max_time - 1, // first minute spent moving to this node
            });
        }
    }

    while nodes_to_explore.len() > 0 {
        let node = nodes_to_explore.pop().unwrap();
        if node.time_left == 0 {
            if node.released_pressure > max_released_pressure {
                max_released_pressure = node.released_pressure;
                //println!("{}", max_released_pressure);
            }
            continue;
        }

        if impossible_to_beat_best_b(&node, max_released_pressure, &room_flows) {
            continue;
        }

        let mut me_possible_rooms_and_flows: Vec<((char, char), i32)> = Vec::new();
        let mut ele_possible_rooms_and_flows: Vec<((char, char), i32)> = Vec::new();
        for me_connected_room in &room_connections[&node.me_room] {
            if *me_connected_room == node.me_prev_room {
                continue;
            }
            me_possible_rooms_and_flows.push((*me_connected_room, 0));
        }
        for ele_connected_room in &room_connections[&node.ele_room] {
            if *ele_connected_room == node.ele_prev_room {
                continue;
            }
            ele_possible_rooms_and_flows.push((*ele_connected_room, 0));
        }

        // add node with current room opened
        if !node.opened_valves.contains(&node.me_room) && room_flows.contains_key(&node.me_room) {
            me_possible_rooms_and_flows.push((node.me_room, room_flows[&node.me_room]));
        }
        if !node.opened_valves.contains(&node.ele_room) && room_flows.contains_key(&node.ele_room) {
            ele_possible_rooms_and_flows.push((node.ele_room, room_flows[&node.ele_room]));
        }

        for (me_room, me_flow) in me_possible_rooms_and_flows.iter() {
            for (ele_room, ele_flow) in ele_possible_rooms_and_flows.iter() {
                if me_room == ele_room && me_flow > &0 {
                    continue; // both agents trying to open same valve
                }
                let mut new_opened_valves = node.opened_valves.clone();
                if *me_flow > 0 && !node.opened_valves.contains(me_room) {
                    new_opened_valves.insert(*me_room);
                }
                if *ele_flow > 0 && !node.opened_valves.contains(ele_room) {
                    new_opened_valves.insert(*ele_room);
                }
                nodes_to_explore.push(TwinNode {
                    released_pressure: node.released_pressure + node.open_flow,
                    open_flow: node.open_flow + me_flow + ele_flow,
                    opened_valves: new_opened_valves,
                    me_room: *me_room,
                    ele_room: *ele_room,
                    me_prev_room: node.me_room,
                    ele_prev_room: node.ele_room,
                    time_left: node.time_left - 1,
                });
            }
        }
    }
    Ok(max_released_pressure)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_is_correct() {
        let answer = result_a().unwrap();
        assert_eq!(answer, 1792);
    }

    #[test]
    fn b_is_correct() {
        let answer = result_b().unwrap();
        assert_eq!(answer, 2587);
    }
}
