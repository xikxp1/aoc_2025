advent_of_code::solution!(2);

const MAX_DIGITS: usize = 20;
const MAX_BASE: usize = 10;

pub fn part_one(input: &str) -> Option<u64> {
    let ranges = parse_ranges(input)?;
    if ranges.is_empty() {
        return Some(0);
    }

    let merged = merge_ranges(ranges);
    let pow10 = build_pow10_table();
    let total = merged
        .into_iter()
        .map(|(start, end)| sum_invalid_in_range(start, end, &pow10))
        .sum::<u128>();

    u64::try_from(total).ok()
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges = parse_ranges(input)?;
    if ranges.is_empty() {
        return Some(0);
    }

    let merged = merge_ranges(ranges);
    let pow10 = build_pow10_table();
    let total = merged
        .into_iter()
        .map(|(start, end)| sum_repeated_range(start, end, &pow10))
        .sum::<u128>();

    u64::try_from(total).ok()
}

fn build_pow10_table() -> [u128; MAX_DIGITS + 1] {
    let mut pow10 = [1u128; MAX_DIGITS + 1];
    for i in 1..pow10.len() {
        pow10[i] = pow10[i - 1] * 10;
    }
    pow10
}

fn parse_ranges(input: &str) -> Option<Vec<(u64, u64)>> {
    let mut ranges = Vec::new();
    for token in input.split(|c: char| c == ',' || c.is_whitespace()) {
        if token.is_empty() {
            continue;
        }

        let (start, end) = token.split_once('-')?;
        let start = start.parse::<u64>().ok()?;
        let end = end.parse::<u64>().ok()?;
        if start <= end {
            ranges.push((start, end));
        } else {
            ranges.push((end, start));
        }
    }

    Some(ranges)
}

fn merge_ranges(mut ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    if ranges.is_empty() {
        return ranges;
    }

    ranges.sort_unstable_by_key(|&(start, _)| start);
    let mut merged: Vec<(u64, u64)> = Vec::with_capacity(ranges.len());

    for (start, end) in ranges {
        if let Some((_, last_end)) = merged.last_mut() {
            if start <= last_end.saturating_add(1) {
                if end > *last_end {
                    *last_end = end;
                }
            } else {
                merged.push((start, end));
            }
        } else {
            merged.push((start, end));
        }
    }

    merged
}

fn sum_invalid_in_range(start: u64, end: u64, pow10: &[u128; MAX_DIGITS + 1]) -> u128 {
    if start > end {
        return 0;
    }

    let mut total = 0u128;
    let mut pow10_prev = 1u128; // 10^0

    for k in 1..=MAX_BASE {
        let pow10 = pow10[k];
        let step = pow10 + 1;
        let start_n = pow10_prev * step;

        let range_end = end as u128;
        if start_n > range_end {
            break;
        }

        let last_n = (pow10 - 1) * step;
        if last_n < start as u128 {
            pow10_prev = pow10;
            continue;
        }

        let count = pow10 - pow10_prev;
        if count == 0 {
            pow10_prev = pow10;
            continue;
        }

        let range_start = start as u128;
        let effective_start = range_start.max(start_n);
        let effective_end = range_end.min(last_n);
        if effective_start > effective_end {
            pow10_prev = pow10;
            continue;
        }

        let i_start = if effective_start <= start_n {
            0
        } else {
            (effective_start - start_n).div_ceil(step)
        };
        if i_start >= count {
            pow10_prev = pow10;
            continue;
        }

        let i_end = ((effective_end - start_n) / step).min(count - 1);
        if i_start > i_end {
            pow10_prev = pow10;
            continue;
        }

        let first_term = start_n + i_start * step;
        let last_term = start_n + i_end * step;
        let terms = i_end - i_start + 1;
        let sum = terms * (first_term + last_term);
        total += sum / 2;

        pow10_prev = pow10;
    }

    total
}

fn sum_repeated_range(start: u64, end: u64, pow10: &[u128; MAX_DIGITS + 1]) -> u128 {
    if start > end {
        return 0;
    }

    let mut divisors: Vec<Vec<usize>> = vec![Vec::new(); MAX_BASE + 1];
    for (k, list) in divisors.iter_mut().enumerate().skip(1) {
        for d in 1..k {
            if k % d == 0 {
                list.push(d);
            }
        }
    }

    let mut g_vals = vec![vec![0u128; MAX_DIGITS + 1]; MAX_BASE + 1];
    let mut total = 0u128;
    let range_end = end as u128;

    for (k, divisor_list) in divisors.iter().enumerate().skip(1) {
        let count = pow10[k] - pow10[k - 1];
        let denom = pow10[k] - 1;

        for r in 2.. {
            let digits = k * r;
            if digits > MAX_DIGITS {
                break;
            }

            let numerator = pow10[digits] - 1;
            let rep_factor = numerator / denom;
            let first_term = pow10[k - 1] * rep_factor;

            if first_term > range_end {
                break;
            }

            let sum_all = sum_progression_overlap(start, end, first_term, rep_factor, count);
            let mut g_val = sum_all;

            for &d in divisor_list {
                let multiplier = k / d;
                let r_prime = r * multiplier;
                let prev = g_vals[d][r_prime];
                if g_val >= prev {
                    g_val -= prev;
                } else {
                    g_val = 0;
                    break;
                }
            }

            g_vals[k][r] = g_val;
            total += g_val;
        }
    }

    total
}

fn sum_progression_overlap(
    range_start: u64,
    range_end: u64,
    first_term: u128,
    step: u128,
    count: u128,
) -> u128 {
    if range_start > range_end || count == 0 {
        return 0;
    }

    let range_start = range_start as u128;
    let range_end = range_end as u128;
    if first_term > range_end {
        return 0;
    }

    let last_term = first_term + (count - 1) * step;
    if last_term < range_start {
        return 0;
    }

    let effective_start = range_start.max(first_term);
    let effective_end = range_end.min(last_term);

    let i_start = if effective_start <= first_term {
        0
    } else {
        (effective_start - first_term).div_ceil(step)
    };

    if i_start >= count {
        return 0;
    }

    let mut i_end = (effective_end - first_term) / step;
    if i_end >= count {
        i_end = count - 1;
    }

    if i_start > i_end {
        return 0;
    }

    let terms = i_end - i_start + 1;
    let first = first_term + i_start * step;
    let last = first_term + i_end * step;
    terms * (first + last) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1_227_775_554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4_174_379_265));
    }
}
