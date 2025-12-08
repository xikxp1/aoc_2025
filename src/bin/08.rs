use std::collections::HashMap;

advent_of_code::solution!(8);

// change for solve run
const TARGET_CONNECTIONS: usize = 10;
const TOP_CIRCUITS: usize = 3;

fn distance(box1: &[u64], box2: &[u64]) -> u64 {
    let dx = box1[0] as i64 - box2[0] as i64;
    let dy = box1[1] as i64 - box2[1] as i64;
    let dz = box1[2] as i64 - box2[2] as i64;
    (dx * dx + dy * dy + dz * dz) as u64
}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        UnionFind {
            parent: (0..size).collect(),
            rank: vec![0; size],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false;
        }

        if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y;
        } else if self.rank[root_x] > self.rank[root_y] {
            self.parent[root_y] = root_x;
        } else {
            self.parent[root_y] = root_x;
            self.rank[root_x] += 1;
        }
        true
    }

    fn get_circuit_sizes(&mut self) -> Vec<usize> {
        let mut circuit_sizes: HashMap<usize, usize> = HashMap::new();
        for i in 0..self.parent.len() {
            let root = self.find(i);
            *circuit_sizes.entry(root).or_insert(0) += 1;
        }
        let mut sizes: Vec<usize> = circuit_sizes.values().copied().collect();
        sizes.sort_by(|a, b| b.cmp(a));
        sizes
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let n = input.lines().count();
    let mut boxes: Vec<Vec<u64>> = Vec::with_capacity(n);

    for line in input.lines() {
        let coords: Vec<u64> = line.split(',').map(|s| s.parse::<u64>().unwrap()).collect();
        boxes.push(coords);
    }

    let mut distances: Vec<((usize, usize), u64)> = Vec::with_capacity((n * (n - 1)) / 2);
    for i in 0..n {
        for j in (i + 1)..n {
            let dist = distance(&boxes[i], &boxes[j]);
            distances.push(((i, j), dist));
        }
    }

    distances.sort_by_key(|(_, d)| *d);

    let mut uf = UnionFind::new(n);
    let mut connections = 0;

    for ((i, j), _dist) in distances {
        uf.union(i, j);
        connections += 1;
        if connections == TARGET_CONNECTIONS {
            break;
        }
    }

    let sizes = uf.get_circuit_sizes();
    if sizes.len() >= TOP_CIRCUITS {
        Some((sizes[0] * sizes[1] * sizes[2]) as u64)
    } else {
        None
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let n = input.lines().count();
    let mut boxes: Vec<Vec<u64>> = Vec::with_capacity(n);

    for line in input.lines() {
        let coords: Vec<u64> = line.split(',').map(|s| s.parse::<u64>().unwrap()).collect();
        boxes.push(coords);
    }

    let mut distances: Vec<((usize, usize), u64)> = Vec::with_capacity((n * (n - 1)) / 2);
    for i in 0..n {
        for j in (i + 1)..n {
            let dist = distance(&boxes[i], &boxes[j]);
            distances.push(((i, j), dist));
        }
    }

    distances.sort_by_key(|(_, d)| *d);

    let mut uf = UnionFind::new(n);
    let mut dist_1_idx = 0;
    let mut dist_2_idx = 0;

    for ((i, j), _dist) in distances {
        if uf.union(i, j) && uf.get_circuit_sizes().len() == 1 {
            dist_1_idx = i;
            dist_2_idx = j;
            break;
        }
    }

    Some(boxes[dist_1_idx][0] * boxes[dist_2_idx][0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
