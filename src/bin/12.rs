advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u64> {
    let n = input.lines().count();
    let mut result = 0u64;
    let mut shape_sizes = vec![0u64; n];
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        match line.contains("x") {
            true => {
                let (area, shape_cnt) = line.split_once(':').unwrap();
                let area = area.trim();
                let shape_cnt = shape_cnt.trim();
                let (area_width, area_height) = area.split_once('x').unwrap();
                let area = area_width.parse::<u64>().unwrap() * area_height.parse::<u64>().unwrap();
                let shape_cnt = shape_cnt
                    .split_whitespace()
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();
                let mut current_area = 0u64;
                for (i, cnt) in shape_cnt.iter().enumerate() {
                    current_area += cnt * shape_sizes[i];
                }
                // println!("current_area: {}, area: {}", current_area, area);
                if current_area <= area {
                    result += 1;
                }
            }
            false => {
                let (shape_idx, size) = line.split_once(':').unwrap();
                let shape_idx = shape_idx.trim();
                let size = size.trim();
                shape_sizes[shape_idx.parse::<usize>().unwrap()] = size.parse::<u64>().unwrap();
            }
        }
    }
    Some(result)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // TODO: I've implemented the solution, but it doesn't work for every input
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
