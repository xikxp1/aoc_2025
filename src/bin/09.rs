use std::collections::BinaryHeap;

advent_of_code::solution!(9);

struct Tile {
    x: u64,
    y: u64,
}

fn distance(tile1: &Tile, tile2: &Tile) -> u64 {
    (((tile1.x as i64 - tile2.x as i64).abs() + 1) * ((tile1.y as i64 - tile2.y as i64).abs() + 1))
        as u64
}

fn is_on_segment(px: i64, py: i64, x1: i64, y1: i64, x2: i64, y2: i64) -> bool {
    if x1 == x2 {
        px == x1 && py >= y1.min(y2) && py <= y1.max(y2)
    } else if y1 == y2 {
        py == y1 && px >= x1.min(x2) && px <= x1.max(x2)
    } else {
        false
    }
}

fn is_inside_polygon(px: i64, py: i64, vertices: &[(i64, i64)]) -> bool {
    // Ray casting specialized for axis-aligned polygon
    let mut inside = false;
    let n = vertices.len();
    for i in 0..n {
        let j = (i + 1) % n;
        let (x1, y1) = vertices[i];
        let (x2, y2) = vertices[j];
        if y1 == y2 {
            // horizontal edge, skip for crossing test
            continue;
        }
        // Ensure y1 < y2
        let (x_low, y_low, y_high) = if y1 < y2 { (x1, y1, y2) } else { (x2, y2, y1) };
        // Half-open on top to avoid double counting vertices
        if py >= y_low && py < y_high && px < x_low {
            inside = !inside;
        }
    }
    inside
}

pub fn part_one(input: &str) -> Option<u64> {
    let n = input.lines().count();
    let mut tiles: Vec<Tile> = Vec::with_capacity(n);
    let mut distances: BinaryHeap<u64> = BinaryHeap::with_capacity(n * (n - 1) / 2);
    for (i, line) in input.lines().enumerate() {
        let (x, y) = line.split_once(',').unwrap();
        tiles.push(Tile {
            x: x.parse::<u64>().unwrap(),
            y: y.parse::<u64>().unwrap(),
        });
        for j in 0..i {
            let dist = distance(&tiles[i], &tiles[j]);
            distances.push(dist);
        }
    }
    Some(*distances.peek().unwrap())
}

pub fn part_two(input: &str) -> Option<u64> {
    let tiles: Vec<(i64, i64)> = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap())
        })
        .collect();

    // Coordinate compression: add vertex coords and neighbors to capture tile boundaries
    let mut xs: Vec<i64> = Vec::with_capacity(tiles.len() * 3 + 4);
    let mut ys: Vec<i64> = Vec::with_capacity(tiles.len() * 3 + 4);
    for &(x, y) in &tiles {
        xs.push(x - 1);
        xs.push(x);
        xs.push(x + 1);
        ys.push(y - 1);
        ys.push(y);
        ys.push(y + 1);
    }
    let (min_x, max_x) = (
        xs.iter().copied().min().unwrap_or(0) - 1,
        xs.iter().copied().max().unwrap_or(0) + 1,
    );
    let (min_y, max_y) = (
        ys.iter().copied().min().unwrap_or(0) - 1,
        ys.iter().copied().max().unwrap_or(0) + 1,
    );
    xs.push(min_x);
    xs.push(max_x + 1);
    ys.push(min_y);
    ys.push(max_y + 1);
    xs.sort_unstable();
    xs.dedup();
    ys.sort_unstable();
    ys.dedup();

    let w = xs.len() - 1;
    let h = ys.len() - 1;

    let mut filled = vec![vec![false; h]; w];
    for xi in 0..w {
        let cx = xs[xi];
        for yi in 0..h {
            let cy = ys[yi];

            let on_edge = tiles.iter().enumerate().any(|(idx, &(x1, y1))| {
                let (x2, y2) = tiles[(idx + 1) % tiles.len()];
                is_on_segment(cx, cy, x1, y1, x2, y2)
            });

            if on_edge || is_inside_polygon(cx, cy, &tiles) {
                filled[xi][yi] = true;
            }
        }
    }

    let mut prefix = vec![vec![0u64; h + 1]; w + 1];
    for xi in 0..w {
        let width = (xs[xi + 1] - xs[xi]) as u64;
        for yi in 0..h {
            let height = (ys[yi + 1] - ys[yi]) as u64;
            let cell_area = width * height;
            let add = if filled[xi][yi] { cell_area } else { 0 };
            prefix[xi + 1][yi + 1] =
                prefix[xi][yi + 1] + prefix[xi + 1][yi] - prefix[xi][yi] + add;
        }
    }

    let find_idx = |vals: &Vec<i64>, v: i64| -> usize {
        vals.binary_search(&v).expect("coord present")
    };

    let mut max_area = 0u64;

    for i in 0..tiles.len() {
        for j in i + 1..tiles.len() {
            let (x1, y1) = tiles[i];
            let (x2, y2) = tiles[j];

            let minx = x1.min(x2);
            let maxx = x1.max(x2);
            let miny = y1.min(y2);
            let maxy = y1.max(y2);

            let rect_area = ((maxx - minx + 1) as u64) * ((maxy - miny + 1) as u64);
            if rect_area <= max_area {
                continue;
            }

            let lx = find_idx(&xs, minx);
            let rx = find_idx(&xs, maxx + 1);
            let ly = find_idx(&ys, miny);
            let ry = find_idx(&ys, maxy + 1);

            let filled_area = prefix[rx][ry] + prefix[lx][ly] - prefix[lx][ry] - prefix[rx][ly];
            if filled_area == rect_area {
                max_area = rect_area;
            }
        }
    }

    Some(max_area)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
