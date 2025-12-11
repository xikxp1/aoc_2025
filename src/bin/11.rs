use std::collections::{HashMap, HashSet};

use petgraph::{Directed, Graph, algo::toposort, graph::NodeIndex};

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

fn count_paths_with_required(
    graph: &Graph<String, (), Directed>,
    start: NodeIndex,
    end: NodeIndex,
    required: &[NodeIndex],
) -> u64 {
    let mut required_nodes = required.to_vec();
    required_nodes.sort_by_key(|n| n.index());

    let required_index: HashMap<NodeIndex, usize> = required_nodes
        .iter()
        .copied()
        .enumerate()
        .map(|(i, node)| (node, i))
        .collect();

    let required_count = required_nodes.len();
    let state_size = 1usize << required_count;
    let full_mask = state_size - 1;

    let order = toposort(graph, None).expect("graph contains a cycle");

    let mut ways = vec![vec![0u64; state_size]; graph.node_count()];

    let start_mask = required_index.get(&start).map(|i| 1usize << i).unwrap_or(0);
    ways[start.index()][start_mask] = 1;

    for node in order {
        let node_idx = node.index();
        for mask in 0..state_size {
            let count = ways[node_idx][mask];
            if count == 0 {
                continue;
            }
            for neighbor in graph.neighbors(node) {
                let mut next_mask = mask;
                if let Some(&bit) = required_index.get(&neighbor) {
                    next_mask |= 1 << bit;
                }
                ways[neighbor.index()][next_mask] += count;
            }
        }
    }

    ways[end.index()][full_mask]
}

pub fn part_one(input: &str) -> Option<u64> {
    let parsed_graph = parse_graph(input, "you", "out", vec![]);
    Some(count_paths_with_required(
        &parsed_graph.graph,
        parsed_graph.start_node,
        parsed_graph.end_node,
        &[],
    ))
}

pub fn part_two(input: &str) -> Option<u64> {
    let parsed_graph = parse_graph(
        input,
        "svr",
        "out",
        vec!["fft".to_string(), "dac".to_string()],
    );
    let mut required: Vec<_> = parsed_graph.to_visit.iter().copied().collect();
    required.sort_by_key(|n| n.index());
    Some(count_paths_with_required(
        &parsed_graph.graph,
        parsed_graph.start_node,
        parsed_graph.end_node,
        &required,
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
        assert_eq!(result, Some(2));
    }
}
