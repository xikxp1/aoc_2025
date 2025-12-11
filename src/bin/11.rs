use std::collections::{HashMap, HashSet};

use petgraph::{Directed, Graph, graph::NodeIndex};

advent_of_code::solution!(11);

#[derive(Debug)]
struct ParsedGraph {
    graph: Graph<String, (), Directed>,
    start_node: NodeIndex,
    end_node: NodeIndex,
    to_visit: HashSet<NodeIndex>,
}

fn parse_graph(
    input: &str,
    start_node_str: &str,
    end_node_str: &str,
    to_visit_str: Vec<String>,
) -> ParsedGraph {
    let mut all_nodes = HashMap::<String, Option<NodeIndex>>::new();
    let mut all_edges = HashSet::<(String, String)>::new();
    let mut graph = Graph::<String, (), Directed>::new();
    let mut start_node: Option<NodeIndex> = None;
    let mut end_node: Option<NodeIndex> = None;
    let mut to_visit = HashSet::new();
    for line in input.lines() {
        if let Some((device, outputs)) = line.split_once(':') {
            let from_node = device.to_string();
            all_nodes.insert(from_node, None);
            let outputs = outputs.split_whitespace().collect::<Vec<&str>>();
            for output in outputs {
                all_nodes.insert(output.to_string(), None);
                all_edges.insert((device.to_string(), output.to_string()));
            }
        }
    }
    for (node, index) in all_nodes.iter_mut() {
        let idx = graph.add_node(node.clone());
        *index = Some(idx);
        if node == start_node_str {
            start_node = Some(idx);
        } else if node == end_node_str {
            end_node = Some(idx);
        } else if to_visit_str.contains(node) {
            to_visit.insert(idx);
        }
    }
    for (u, v) in all_edges {
        let from = all_nodes
            .get(&u)
            .copied()
            .flatten()
            .expect("m()issing source node");
        let to = all_nodes
            .get(&v)
            .copied()
            .flatten()
            .expect("missing target node");
        graph.add_edge(from, to, ());
    }
    ParsedGraph {
        graph,
        start_node: start_node.unwrap(),
        end_node: end_node.unwrap(),
        to_visit,
    }
}

fn count_paths(
    graph: &Graph<String, (), Directed>,
    from: NodeIndex,
    to: NodeIndex,
    visited: &mut HashSet<NodeIndex>,
    to_visit: &mut HashSet<NodeIndex>,
) -> u64 {
    if from == to {
        return to_visit.is_empty() as u64;
    }

    visited.insert(from);

    let mut total_paths = 0;

    for neighbor in graph.neighbors(from) {
        if !visited.contains(&neighbor) {
            let removed = to_visit.remove(&neighbor);
            total_paths += count_paths(graph, neighbor, to, visited, to_visit);
            if removed {
                to_visit.insert(neighbor);
            }
        }
    }

    visited.remove(&from);

    total_paths
}

pub fn part_one(input: &str) -> Option<u64> {
    let parsed_graph = parse_graph(input, "you", "out", vec![]);
    let mut visited = HashSet::new();
    let mut to_visit = HashSet::new();
    Some(count_paths(
        &parsed_graph.graph,
        parsed_graph.start_node,
        parsed_graph.end_node,
        &mut visited,
        &mut to_visit,
    ))
}

pub fn part_two(input: &str) -> Option<u64> {
    let parsed_graph = parse_graph(
        input,
        "svr",
        "out",
        vec!["fft".to_string(), "dac".to_string()],
    );
    println!("{:?}", parsed_graph);
    let mut visited = HashSet::new();
    Some(count_paths(
        &parsed_graph.graph,
        parsed_graph.start_node,
        parsed_graph.end_node,
        &mut visited,
        &mut parsed_graph.to_visit.clone(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        // TODO: optimize for part two
        assert_eq!(result, Some(2));
    }
}
