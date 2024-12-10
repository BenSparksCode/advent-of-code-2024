use std::collections::{HashMap, HashSet};

advent_of_code::solution!(10);

const DIRS: [(i32, i32); 4] = [
    (0, -1), // UP
    (1, 0),  // RIGHT
    (0, 1),  // DOWN
    (-1, 0), // LEFT
];

pub fn dfs_1(
    grid: &Vec<Vec<i32>>,
    memo: &mut HashMap<(i32, i32), HashSet<(i32, i32)>>,
    p: (i32, i32),
    max_x: i32,
    max_y: i32,
) -> HashSet<(i32, i32)> {
    // If p in memo, already explored -> return HashSet for p
    if let Some(res) = memo.get(&p) {
        return res.clone();
    }

    // if p is a peak (9), stop exploring, return HashSet with p added
    let height = grid[p.1 as usize][p.0 as usize];
    if height == 9 {
        return HashSet::from([p]);
    }

    // Otherwise explore in all 4 directions if possible
    let mut set: HashSet<(i32, i32)> = HashSet::new();
    for d in DIRS {
        let next = (p.0 + d.0, p.1 + d.1);

        // Only move to next if next is in bounds
        if next.0 >= 0 && next.0 <= max_x && next.1 >= 0 && next.1 <= max_y {
            // Only move to next if height increases by 1
            if grid[next.1 as usize][next.0 as usize] == height + 1 {
                set.extend(dfs_1(grid, memo, next, max_x, max_y));
            }
        }
    }

    // Once all explore directions returned, update memo for this point
    memo.insert(p, set.clone());

    set
}

pub fn dfs_2(
    grid: &Vec<Vec<i32>>,
    memo: &mut HashMap<(i32, i32), i32>,
    p: (i32, i32),
    max_x: i32,
    max_y: i32,
) -> i32 {
    // If p in memo, already explored -> return value
    if let Some(res) = memo.get(&p) {
        return *res;
    }

    // if p is a peak (9), stop exploring, return 1
    let height = grid[p.1 as usize][p.0 as usize];
    if height == 9 {
        return 1;
    }

    // Otherwise explore in all 4 directions if possible
    let mut score = 0;

    for d in DIRS {
        let next = (p.0 + d.0, p.1 + d.1);

        // Only move to next if next is in bounds
        if next.0 >= 0 && next.0 <= max_x && next.1 >= 0 && next.1 <= max_y {
            // Only move to next if height increases by 1
            if grid[next.1 as usize][next.0 as usize] == height + 1 {
                score += dfs_2(grid, memo, next, max_x, max_y);
            }
        }
    }

    // Once all explore directions returned, update memo for this point
    memo.insert(p, score);

    score
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();
    let mut memo: HashMap<(i32, i32), HashSet<(i32, i32)>> = HashMap::new();
    let max_x = (grid[0].len() - 1) as i32;
    let max_y = (grid.len() - 1) as i32;

    let mut sum = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, n) in row.iter().enumerate() {
            // Only explore from trailheads
            if *n == 0 {
                sum += dfs_1(&grid, &mut memo, (x as i32, y as i32), max_x, max_y).len();
            }
        }
    }

    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();
    let mut memo: HashMap<(i32, i32), i32> = HashMap::new();
    let max_x = (grid[0].len() - 1) as i32;
    let max_y = (grid.len() - 1) as i32;

    let mut sum = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, n) in row.iter().enumerate() {
            // Only explore from trailheads
            if *n == 0 {
                sum += dfs_2(&grid, &mut memo, (x as i32, y as i32), max_x, max_y);
            }
        }
    }

    Some(sum as u32)
}
