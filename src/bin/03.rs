advent_of_code::solution!(3);

const MAX_DIGITS: usize = 12;

pub fn part_one(input: &str) -> Option<u64> {
    let mut total: u64 = 0;
    for token in input.split_whitespace() {
        if token.is_empty() {
            continue;
        }
        let mut first_index: Option<usize> = None;
        let mut last_index: Option<usize> = None;
        let mut first_digit: u8 = 0;
        let mut last_digit: u8 = 0;
        for digit in (0..=9).rev() {
            first_index = find_digit(token, digit, 0, token.len() - 1);
            if first_index.is_some() {
                first_digit = digit;
                break;
            }
        }
        if first_index.is_none() {
            continue;
        }
        for digit in (0..=9).rev() {
            last_index = find_digit(token, digit, first_index.unwrap() + 1, token.len());
            if last_index.is_some() {
                last_digit = digit;
                break;
            }
        }
        if last_index.is_none() {
            continue;
        }
        total += first_digit as u64 * 10 + last_digit as u64;
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut total: u64 = 0;
    for token in input.split_whitespace() {
        if token.is_empty() {
            continue;
        }
        let mut sum: u64 = 0;
        let mut start_idx: usize = 0;
        for i in (0..MAX_DIGITS).rev() {
            for digit in (0..=9).rev() {
                let index = find_digit(token, digit, start_idx, token.len() - i);
                if let Some(x) = index {
                    // println!("digit: {}, x: {}", digit, x);
                    sum = sum * 10 + digit as u64;
                    start_idx = x + 1;
                    break;
                }
            }
            if start_idx >= token.len() {
                break;
            }
        }
        total += sum;
    }
    Some(total)
}

fn find_digit(input: &str, digit: u8, start: usize, end: usize) -> Option<usize> {
    for (i, c) in input.chars().enumerate().skip(start).take(end - start) {
        if c.to_digit(10).unwrap() == digit as u32 {
            return Some(i);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
