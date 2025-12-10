use std::collections::HashSet;

advent_of_code::solution!(10);

#[derive(Debug)]
struct Machine {
    target: Vec<u8>,
    joltage: Vec<u64>,
    buttons: Vec<Vec<usize>>,
    num_lights: usize,
}

fn build_matrix(buttons: &[Vec<usize>], num_lights: usize) -> Vec<Vec<u8>> {
    let num_buttons = buttons.len();
    let mut matrix = vec![vec![0u8; num_buttons]; num_lights];

    for (j, button) in buttons.iter().enumerate() {
        for &light_idx in button {
            if light_idx < num_lights {
                matrix[light_idx][j] = 1;
            }
        }
    }

    matrix
}

fn parse_machine(line: &str) -> Option<Machine> {
    let pattern_start = line.find('[')?;
    let pattern_end = line.find(']')?;
    let pattern = &line[pattern_start + 1..pattern_end];

    let target: Vec<u8> = pattern
        .chars()
        .map(|c| if c == '#' { 1 } else { 0 })
        .collect();

    let num_lights = target.len();

    let mut buttons = Vec::new();
    let mut i = pattern_end + 1;

    while i < line.len() {
        if line[i..].starts_with('(') {
            if let Some(end) = line[i..].find(')') {
                let button_str = &line[i + 1..i + end];
                let lights: Vec<usize> = button_str
                    .split(',')
                    .filter_map(|s| s.trim().parse().ok())
                    .collect();
                if !lights.is_empty() {
                    buttons.push(lights);
                }
                i += end + 1;
            } else {
                break;
            }
        } else {
            i += 1;
        }
    }

    let pattern_start = line.find('{')?;
    let pattern_end = line.find('}')?;
    let pattern = &line[pattern_start + 1..pattern_end];

    let joltage: Vec<u64> = pattern
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    Some(Machine {
        target,
        joltage,
        buttons,
        num_lights,
    })
}

fn solve_machine(input: &str) -> Option<u64> {
    let machine = parse_machine(input)?;

    if machine.buttons.is_empty() {
        return if machine.target.iter().all(|&t| t == 0) {
            Some(0)
        } else {
            None
        };
    }

    let matrix = build_matrix(&machine.buttons, machine.num_lights);
    let (x0, nullspace) = gauss_eliminate_gf2(&matrix, &machine.target)?;

    Some(
        find_min_weight_solution(&x0, &nullspace)
            .try_into()
            .unwrap(),
    )
}

fn solve_machine_with_joltage(input: &str) -> Option<u64> {
    let machine = parse_machine(input)?;

    if machine.buttons.is_empty() {
        return if machine.target.iter().all(|&t| t == 0) {
            Some(0)
        } else {
            None
        };
    }

    let matrix = build_matrix(&machine.buttons, machine.num_lights);

    match gauss_eliminate(&matrix, &machine.joltage) {
        Some((x0, _)) => {
            Some(u64::try_from(l1_norm(&x0)).ok()?)
        }
        None => {
            println!("No solution found for {:?}", machine);
            None
        },
    }
}

fn find_min_weight_solution(x0: &[u8], nullspace: &[Vec<u8>]) -> usize {
    if nullspace.is_empty() {
        return hamming_weight(x0);
    }

    let k = nullspace.len();
    let mut min_weight = hamming_weight(x0);

    // Enumerate all 2^k combinations
    for combo in 0..(1 << k) {
        let mut x = x0.to_vec();

        for (i, basis_vec) in nullspace.iter().enumerate() {
            if (combo & (1 << i)) != 0 {
                x = xor_vectors(&x, basis_vec);
            }
        }

        let weight = hamming_weight(&x);
        if weight < min_weight {
            min_weight = weight;
        }
    }

    min_weight
}

fn gauss_eliminate_gf2(a: &[Vec<u8>], b: &[u8]) -> Option<(Vec<u8>, Vec<Vec<u8>>)> {
    let m = a.len();
    let n = if m > 0 { a[0].len() } else { 0 };

    // Create augmented matrix [A|b]
    let mut aug: Vec<Vec<u8>> = a
        .iter()
        .zip(b.iter())
        .map(|(row, &bi)| {
            let mut new_row = row.clone();
            new_row.push(bi);
            new_row
        })
        .collect();

    let mut pivot_cols = Vec::new();
    let mut row = 0;

    // Forward elimination
    for col in 0..n {
        // Find pivot
        let pivot_row = (row..m).find(|&i| aug[i][col] == 1);

        if pivot_row.is_none() {
            continue;
        }

        let pivot_row = pivot_row.unwrap();

        aug.swap(row, pivot_row);

        let pivot = aug[row].clone();
        for (i, target_row) in aug.iter_mut().enumerate() {
            if i != row && target_row[col] == 1 {
                for (val, &pivot_val) in target_row.iter_mut().take(n + 1).zip(pivot.iter()) {
                    *val ^= pivot_val;
                }
            }
        }

        pivot_cols.push(col);
        row += 1;
    }

    if aug.iter().take(m).skip(row).any(|row| row[n] == 1) {
        return None;
    }

    let mut x0 = vec![0u8; n];
    for (i, &col) in pivot_cols.iter().enumerate() {
        x0[col] = aug[i][n];
    }

    let pivot_set: HashSet<usize> = pivot_cols.iter().copied().collect();
    let free_cols: Vec<usize> = (0..n).filter(|j| !pivot_set.contains(j)).collect();

    let mut nullspace = Vec::new();

    for &free_col in &free_cols {
        let mut v = vec![0u8; n];
        v[free_col] = 1;

        for (i, &pivot_col) in pivot_cols.iter().enumerate() {
            if aug[i][free_col] == 1 {
                v[pivot_col] = 1;
            }
        }

        nullspace.push(v);
    }

    Some((x0, nullspace))
}

fn gauss_eliminate(a: &[Vec<u8>], b: &[u64]) -> Option<(Vec<i64>, Vec<Vec<i64>>)> {
    let m = a.len();
    let n = if m > 0 { a[0].len() } else { 0 };

    let mut aug: Vec<Vec<i64>> = a
        .iter()
        .zip(b.iter())
        .map(|(row, &bi)| {
            let mut new_row: Vec<i64> = row.clone().iter().map(|&x| x as i64).collect();
            new_row.push(bi as i64);
            new_row
        })
        .collect();

    let mut pivot_cols = Vec::new();
    let mut row = 0;

    for col in 0..n {
        let pivot_row = (row..m).find(|&i| aug[i][col] != 0);

        if pivot_row.is_none() {
            continue;
        }

        let pivot_row = pivot_row.unwrap();
        aug.swap(row, pivot_row);

        pivot_cols.push(col);
        row += 1;
    }

    let pivot_set: HashSet<usize> = pivot_cols.iter().copied().collect();
    let free_cols: Vec<usize> = (0..n).filter(|j| !pivot_set.contains(j)).collect();

    let mut x0 = vec![0i64; n];

    for i in (0..pivot_cols.len()).rev() {
        let col = pivot_cols[i];
        let mut sum = aug[i][n];

        for j in (col + 1)..n {
            sum -= aug[i][j] * x0[j] as i64;
        }

        if aug[i][col] == 0 || sum % aug[i][col] != 0 {
            println!("#1 No solution found for {:?}", aug);
            return None;
        }

        x0[col] = sum / aug[i][col];
    }

    for aug in aug.iter().take(m) {
        let mut sum = 0;
        for j in 0..n {
            sum += aug[j] * x0[j];
        }
        if sum != aug[n] {
            println!("#2 No solution found for {:?}", aug);
            return None;
        }
    }

    let mut nullspace = Vec::new();

    for &free_col in &free_cols {
        let mut v = vec![0i64; n];
        v[free_col] = 1;

        // Solve for pivot variables when free_col = 1
        for i in (0..pivot_cols.len()).rev() {
            let col = pivot_cols[i];
            let mut sum = -aug[i][free_col];

            for j in (col + 1)..n {
                if j != free_col {
                    sum -= aug[i][j] * v[j] as i64;
                }
            }

            if aug[i][col] != 0 {
                v[col] = sum / aug[i][col];
            }
        }

        nullspace.push(v);
    }

    Some((x0, nullspace))
}

fn hamming_weight(vec: &[u8]) -> usize {
    vec.iter().filter(|&&x| x == 1).count()
}

fn xor_vectors(v1: &[u8], v2: &[u8]) -> Vec<u8> {
    v1.iter().zip(v2.iter()).map(|(a, b)| a ^ b).collect()
}

fn l1_norm(vec: &[i64]) -> i64 {
    vec.iter().map(|&x| x.abs()).sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    input.lines().map(solve_machine).sum()
}

pub fn part_two(input: &str) -> Option<u64> {
    input.lines().map(solve_machine_with_joltage).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // Not solved yet Some(33)
        assert_eq!(result, None);
    }
}
