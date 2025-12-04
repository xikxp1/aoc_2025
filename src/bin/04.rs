advent_of_code::solution!(4);

const DIRS: [(i16, i16); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

const ADJACENT_MAX_COUNT: usize = 3;

pub fn part_one(input: &str) -> Option<u64> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut removed_grid: Vec<Vec<bool>> = Vec::new();
    for line in input.lines() {
        grid.push(line.chars().collect());
        removed_grid.push(line.chars().map(|_| false).collect::<Vec<bool>>());
    }
    let (rolls, _) = count_rolls(&grid, &mut removed_grid);
    Some(rolls as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut removed_grid: Vec<Vec<bool>> = Vec::new();
    for line in input.lines() {
        grid.push(line.chars().collect());
        removed_grid.push(line.chars().map(|_| false).collect::<Vec<bool>>());
    }
    let mut removed_count: usize = 0;
    loop {
        let (_, cnt) = count_rolls(&grid, &mut removed_grid);
        if cnt == 0 {
            break;
        }
        removed_count += cnt;
    }
    Some(removed_count as u64)
}

fn count_rolls(grid: &[Vec<char>], removed_grid: &mut [Vec<bool>]) -> (usize, usize) {
    let mut count: usize = 0;
    let mut removed: Vec<(usize, usize)> = Vec::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c != '@' {
                continue;
            }
            if removed_grid[i][j] {
                continue;
            }
            let mut adjacent_count: usize = 0;
            for (di, dj) in DIRS {
                let ni = i as i16 + di;
                let nj = j as i16 + dj;
                if ni < 0 || ni >= grid.len() as i16 || nj < 0 || nj >= grid[i].len() as i16 {
                    continue;
                }
                let nc = grid[ni as usize][nj as usize];
                if nc != '@' {
                    continue;
                }
                if removed_grid[ni as usize][nj as usize] {
                    continue;
                }
                adjacent_count += 1;
                if adjacent_count > ADJACENT_MAX_COUNT {
                    break;
                }
            }
            if adjacent_count <= ADJACENT_MAX_COUNT {
                removed.push((i, j));
                count += 1;
            }
        }
    }
    let removed_count = removed.len();
    for (i, j) in removed {
        removed_grid[i][j] = true;
    }
    (count, removed_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
