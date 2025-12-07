use std::collections::HashSet;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let mut result: u64 = 0;
    let mut ray_positions: HashSet<usize> = HashSet::new();
    for (i, line) in input.lines().enumerate() {
        match i {
            0 => {
                for (j, c) in line.chars().enumerate() {
                    if c == 'S' {
                        ray_positions.insert(j);
                        break;
                    }
                }
            }
            _ => {
                let mut new_positions: HashSet<usize> = ray_positions.clone();
                for (j, c) in line.chars().enumerate() {
                    if c == '^' && ray_positions.contains(&j) {
                        if j > 0 && !ray_positions.contains(&(j - 1)) {
                            new_positions.insert(j - 1);
                        }
                        if j < line.len() - 1 && !ray_positions.contains(&(j + 1)) {
                            new_positions.insert(j + 1);
                        }
                        new_positions.remove(&j);
                        result += 1;
                    }
                }
                ray_positions = new_positions;
            }
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut ray_weights: Vec<u64> = Vec::new();
    for (i, line) in input.lines().enumerate() {
        match i {
            0 => {
                for c in line.chars() {
                    ray_weights.push(if c == 'S' { 1 } else { 0 });
                }
            }
            _ => {
                let mut new_weights: Vec<u64> = ray_weights.clone();
                for (j, c) in line.chars().enumerate() {
                    if c == '^' {
                        new_weights[j] = 0;
                        if j > 0 {
                            new_weights[j - 1] += ray_weights[j];
                        }
                        if j < line.len() - 1 {
                            new_weights[j + 1] += ray_weights[j];
                        }
                    }
                }
                ray_weights = new_weights;
            }
        }
    }
    Some(ray_weights.iter().sum::<u64>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
