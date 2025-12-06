advent_of_code::solution!(6);

#[derive(Debug)]
enum Op {
    Add,
    Mul,
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut numbers: Vec<u64> = Vec::new();
    let mut ops: Vec<Op> = Vec::new();
    let mut is_op = true;
    for line in input.lines().rev() {
        let tokens = line.split_whitespace().collect::<Vec<&str>>();
        if is_op {
            for token in tokens {
                if token == "*" {
                    ops.push(Op::Mul);
                    numbers.push(1u64);
                } else if token == "+" {
                    ops.push(Op::Add);
                    numbers.push(0u64);
                }
            }
            is_op = false;
            continue;
        }
        for (i, token) in tokens.iter().enumerate() {
            if let Ok(num) = token.parse::<u64>() {
                match ops[i] {
                    Op::Mul => numbers[i] *= num,
                    Op::Add => numbers[i] += num,
                }
            }
        }
    }
    Some(numbers.iter().sum::<u64>())
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();

    let op_line = lines.last().unwrap();
    let num_lines: Vec<&str> = lines.iter().take(lines.len() - 1).copied().collect();

    let ops: Vec<Op> = op_line
        .split_whitespace()
        .filter_map(|token| match token {
            "*" => Some(Op::Mul),
            "+" => Some(Op::Add),
            _ => None,
        })
        .collect();

    let mut row_token_positions: Vec<Vec<(usize, usize)>> = Vec::new();
    for line in &num_lines {
        let mut positions = Vec::new();
        let mut start = 0;
        for token in line.split_whitespace() {
            if let Some(pos) = line[start..].find(token) {
                let token_start = start + pos;
                let token_end = token_start + token.len();
                positions.push((token_start, token_end));
                start = token_end;
            }
        }
        row_token_positions.push(positions);
    }

    let mut grand_total = 0u64;

    for problem_idx in 0..ops.len() {
        let op = &ops[ops.len() - 1 - problem_idx];
        let mut problem_numbers: Vec<u64> = Vec::new();

        let mut min_start = usize::MAX;
        let mut max_end = 0;

        for positions in &row_token_positions {
            if let Some(&(start, end)) = positions.iter().rev().nth(problem_idx) {
                min_start = min_start.min(start);
                max_end = max_end.max(end);
            }
        }

        for char_pos in (min_start..max_end).rev() {
            let mut num_str = String::new();

            for line in num_lines.iter().rev() {
                if let Some(ch) = line.chars().nth(char_pos)
                    && ch.is_ascii_digit()
                {
                    num_str.push(ch);
                }
            }

            if !num_str.is_empty() {
                if num_str.len() > 1 {
                    num_str = num_str.chars().rev().collect();
                }
                if let Ok(num) = num_str.parse::<u64>() {
                    problem_numbers.push(num);
                }
            }
        }

        if !problem_numbers.is_empty() {
            let result = match op {
                Op::Add => problem_numbers.iter().sum::<u64>(),
                Op::Mul => problem_numbers.iter().product::<u64>(),
            };
            grand_total += result;
        }
    }

    Some(grand_total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
