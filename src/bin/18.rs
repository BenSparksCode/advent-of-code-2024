use std::collections::HashMap;

advent_of_code::solution!(18);

const DIRS: [(i32, i32); 4] = [
    (0, -1), // UP
    (1, 0),  // RIGHT
    (0, 1),  // DOWN
    (-1, 0), // LEFT
];

pub fn parse_input(input: &str) -> Option<Vec<(i32, i32)>> {
    let bytes = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            return (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap());
        })
        .collect();

    Some(bytes)
}

pub fn build_grid(bytes: &Vec<(i32, i32)>, height: usize, limit: usize) -> Vec<Vec<char>> {
    let mut grid = vec![vec!['.'; height + 1]; height + 1];
    for (i, b) in bytes.iter().enumerate() {
        if i > limit - 1 {
            break;
        }
        grid[b.1 as usize][b.0 as usize] = '#';
    }
    grid
}

pub fn dfs(
    grid: &Vec<Vec<char>>,
    memo: &mut HashMap<(i32, i32), i32>,
    max_x: i32,
    p: (i32, i32),
    steps: i32,
) {
    // Stop if we found a faster route to this tile already
    let dist = memo.entry(p).or_insert(i32::MAX);
    if steps >= *dist {
        return;
    } else {
        *dist = steps;
    }
    // Stop if we reach the bottom right tile (end)
    if p == (max_x, max_x) {
        return;
    }

    for d in DIRS {
        let next = (p.0 + d.0, p.1 + d.1);

        // Stop if next is out of bounds
        if next.0 < 0 || next.0 > max_x || next.1 < 0 || next.1 > max_x {
            continue;
        }
        // If next is obstacle, cannot move there, try next direction
        if grid[next.1 as usize][next.0 as usize] == '#' {
            continue;
        }

        dfs(grid, memo, max_x, next, steps + 1);
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let bytes = parse_input(input).unwrap();
    let height = 71;
    let limit = 1024;
    let start = (0, 0);

    // store lowest steps to reach tile
    let mut memo: HashMap<(i32, i32), i32> = HashMap::new();
    let grid = build_grid(&bytes, height, limit);

    dfs(&grid, &mut memo, height as i32 - 1, start, 0);

    Some(*memo.get(&(height as i32 - 1, height as i32 - 1)).unwrap() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}
