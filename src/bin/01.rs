advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut cur: i64 = 50;
    let mut count_zero: u64 = 0;

    for token in input.split_whitespace() {
        if token.is_empty() {
            continue;
        }

        let dir = token.chars().next().unwrap();
        let num_str = &token[1..];
        let x = num_str.parse::<i64>().unwrap();

        match dir {
            'L' => {
                cur = (cur - x) % 100;
                if cur < 0 {
                    cur += 100;
                }
            }
            'R' => {
                cur = (cur + x) % 100;
            }
            _ => continue,
        }

        if cur == 0 {
            count_zero += 1;
        }
    }

    Some(count_zero)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut cur: i64 = 50;
    let mut count_zero: u64 = 0;

    for token in input.split_whitespace() {
        if token.is_empty() {
            continue;
        }

        let dir = token.chars().next().unwrap();
        let num_str = &token[1..];
        let x = num_str.parse::<i64>().unwrap();

        let hits = match dir {
            'L' => {
                let first_hit = if cur == 0 { 100 } else { cur };
                if x < first_hit {
                    0
                } else {
                    1 + (x - first_hit) / 100
                }
            }
            'R' => {
                let first_hit = if cur == 0 { 100 } else { 100 - cur };
                if x < first_hit {
                    0
                } else {
                    1 + (x - first_hit) / 100
                }
            }
            _ => continue,
        };

        count_zero += hits as u64;

        cur = match dir {
            'L' => (cur - x).rem_euclid(100),
            'R' => (cur + x).rem_euclid(100),
            _ => cur,
        };
    }

    Some(count_zero)
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
        assert_eq!(result, Some(6));
    }
}
