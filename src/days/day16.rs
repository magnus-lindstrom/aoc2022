use crate::utils;
use petgraph::Graph;
use std::collections::HashMap;
const FILE_PATH: &str = "inputs/day16_test.txt";

pub fn result_a() -> Result<i32, &'static str> {
    let input = utils::vector_of_string_vectors_from_file(FILE_PATH);
    //let nodes: HashMap<String,
    let mut graph: Graph<&str, (), petgraph::prelude::Undirected> =
        Graph::<&str, (), petgraph::Undirected>::new_undirected();
    let mut connects_with: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut node_name_to_node_index: HashMap<&str, Graph<&str, (), petgraph::prelude::Undirected>::NodeIndex> = HashMap::new();
    for line in input.iter() {
        let node_name = &line[1];
        let _flow_rate: i32 = line[4]
            .trim_end_matches(';')
            .trim_start_matches(&['r', 'a', 't', 'e', '='])
            .parse()
            .unwrap();
        let mut connected_node_names: Vec<&str> = Vec::new();
        for connected_name in 9..line.len() {
            connected_node_names.push(line[connected_name].trim_end_matches(','));
        }
        connects_with.insert(node_name, connected_node_names);
        let node = graph.add_node(node_name);
        node_name_to_node_index.insert(node_name, node);
    }

    for node_name in connects_with.keys() {
        let node = graph.add_node(node_name);
        for connected_node in connects_with[node_name].iter() {
            let connected_node = graph.add
        }
    }

    let a = graph.add_node("a");
    let b = graph.add_node("b");
    let c = graph.add_node("c");
    let d = graph.add_node("d");
    let e = graph.add_node("e");
    graph.extend_with_edges(&[(a, b), (b, c), (c, d), (d, e)]);

    for start in graph.node_indices() {
        println!("---{}---", start.index());
        println!("{:?}", petgraph::algo::dijkstra(&graph, start, None, |_| 1));
    }
    Ok(0)
}

pub fn result_b() -> Result<i32, &'static str> {
    let _input = utils::vector_of_string_vectors_from_file(FILE_PATH);
    //let nodes: HashMap<String,
    let mut graph: Graph<&str, (), petgraph::prelude::Undirected> =
        Graph::<&str, (), petgraph::Undirected>::new_undirected();
    let a = graph.add_node("a");
    let b = graph.add_node("b");
    let c = graph.add_node("c");
    let d = graph.add_node("d");
    let e = graph.add_node("e");
    graph.extend_with_edges(&[(a, b), (b, c), (c, d), (d, e)]);

    for _start in graph.node_indices() {
        // println!("---{}---", start.index());
        // println!("{:?}", petgraph::algo::dijkstra(&graph, start, None, |_| 1));
    }
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
