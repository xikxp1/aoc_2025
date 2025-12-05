advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let mut total: usize = 0;
    let mut segments = Vec::new();
    let mut parse_segments = true;
    for line in input.lines() {
        if line.is_empty() {
            parse_segments = false;
            continue;
        }
        if parse_segments {
            if let Some((start, end)) = line.split_once('-') {
                let start = start.parse::<u64>().unwrap();
                let end = end.parse::<u64>().unwrap();
                segments.push((start, end));
            }
        } else {
            let number = line.parse::<u64>().unwrap();
            for segment in &segments {
                if number >= segment.0 && number <= segment.1 {
                    total += 1;
                    break;
                }
            }
        }
    }
    Some(total as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut segments: Vec<(u64, u64)> = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            break;
        }
        if let Some((start, end)) = line.split_once('-') {
            let start = start.parse::<u64>().unwrap();
            let end = end.parse::<u64>().unwrap();
            segments.push((start, end));
        }
    }
    if segments.is_empty() {
        return Some(0);
    }
    segments.sort_unstable_by_key(|segment| segment.0);
    let mut merged: Vec<(u64, u64)> = Vec::with_capacity(segments.len());
    for (start, end) in segments {
        if let Some(last) = merged.last_mut()
            && start <= last.1.saturating_add(1)
        {
            last.1 = last.1.max(end);
        } else {
            merged.push((start, end));
        }
    }
    let total: u64 = merged.into_iter().map(|(start, end)| end - start + 1).sum();
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
