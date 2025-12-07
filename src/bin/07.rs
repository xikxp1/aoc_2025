advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let mut result: u64 = 0;
    let mut ray_positions: Vec<bool> = Vec::new();
    for (i, line) in input.lines().enumerate() {
        match i {
            0 => {
                for c in line.chars() {
                    ray_positions.push(c == 'S');
                }
            }
            i if i % 2 == 0 => {
                for (j, c) in line.chars().enumerate() {
                    if c == '^' && ray_positions[j] {
                        if j > 0 && !ray_positions[j - 1] {
                            ray_positions[j - 1] = true;
                        }
                        if j < line.len() - 1 && !ray_positions[j + 1] {
                            ray_positions[j + 1] = true;
                        }
                        ray_positions[j] = false;
                        result += 1;
                    }
                }
            }
            _ => continue,
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
            i if i % 2 == 0 => {
                for (j, c) in line.chars().enumerate() {
                    if c == '^' {
                        if j > 0 {
                            ray_weights[j - 1] += ray_weights[j];
                        }
                        if j < line.len() - 1 {
                            ray_weights[j + 1] += ray_weights[j];
                        }
                        ray_weights[j] = 0;
                    }
                }
            }
            _ => continue,
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
