use std::collections::{HashMap, HashSet};

advent_of_code::solution!(16);

const DIRS: [(i32, i32); 4] = [
    (0, -1), // UP
    (1, 0),  // RIGHT
    (0, 1),  // DOWN
    (-1, 0), // LEFT
];

pub fn parse_input(input: &str) -> Option<(Vec<Vec<char>>, (i32, i32))> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    for y in (0..grid.len() - 1).rev() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 'S' {
                return Some((grid, (x as i32, y as i32)));
            }
        }
    }

    None
}

pub fn dfs(
    grid: &Vec<Vec<char>>,
    memo: &mut HashMap<((i32, i32), (i32, i32)), u32>,
    lowest: &mut u32,
    current: u32,
    p: (i32, i32),
    dir: (i32, i32),
) {
    // If current score higher than lowest, stop
    if current >= *lowest {
        return;
    }

    // Check we have the lowest score so far for this point and direction
    let dir_score = memo.entry((p, dir)).or_insert(u32::MAX);
    if current >= *dir_score {
        return;
    } else {
        *dir_score = current;
    }

    // If end (E) reached, and score lower than lowest, update lowest
    if grid[p.1 as usize][p.0 as usize] == 'E' {
        *lowest = current;
        return;
    }

    // Otherwise continue searching in all directions available
    for d in DIRS {
        let next = (p.0 + d.0, p.1 + d.1);
        if grid[next.1 as usize][next.0 as usize] == '#' {
            // Direction is blocked. Continue to other directions.
            continue;
        }
        if (d.0 + dir.0 == 0) && (d.1 + dir.1 == 0) {
            // Cannot turn 180 degrees. Continue to other directions.
            continue;
        } else if d == dir {
            // Moving in the same direction, add 1 to score
            dfs(grid, memo, lowest, current + 1, next, d);
        } else {
            // Turning 90 degrees (+1000) and move (+1), add 1001 to score
            dfs(grid, memo, lowest, current + 1001, next, d);
        }
    }
}

pub fn dfs2(
    grid: &Vec<Vec<char>>,
    best: &mut HashSet<(i32, i32)>,
    memo: &mut HashMap<((i32, i32), (i32, i32)), u32>,
    lowest: &u32, // Not mutable, lowest discovered in Part 1
    current: u32,
    p: (i32, i32),
    dir: (i32, i32),
    tiles: Vec<(i32, i32)>,
) {
    // If current score higher than lowest, stop
    if current > *lowest {
        return;
    }

    // If end (E) reached, add all tiles to best
    if grid[p.1 as usize][p.0 as usize] == 'E' {
        let mut final_tiles = tiles.clone();
        final_tiles.push(p);

        for t in final_tiles {
            best.insert(t);
        }

        return;
    }

    // Check we have the lowest score so far for this point and direction
    let dir_score = memo.entry((p, dir)).or_insert(*lowest);
    // NOTE: We explore if current == dir_score to get ALL optimal paths in Part 2
    if current > *dir_score {
        return;
    } else {
        *dir_score = current;
    }

    // Otherwise continue searching in all directions available
    for d in DIRS {
        let next = (p.0 + d.0, p.1 + d.1);
        if grid[next.1 as usize][next.0 as usize] == '#' {
            // Direction is blocked. Continue to other directions.
            continue;
        }
        if (d.0 + dir.0 == 0) && (d.1 + dir.1 == 0) {
            // Cannot turn 180 degrees. Continue to other directions.
            continue;
        } else if d == dir {
            // Moving in the same direction, add 1 to score
            let mut new_tiles = tiles.clone();
            new_tiles.push(p);
            dfs2(grid, best, memo, lowest, current + 1, next, d, new_tiles);
        } else {
            // Turning 90 degrees (+1000) and move (+1), add 1001 to score
            let mut new_tiles = tiles.clone();
            new_tiles.push(p);
            dfs2(grid, best, memo, lowest, current + 1001, next, d, new_tiles);
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, start) = parse_input(input).unwrap();
    // Key = ((x, y), (dx, dy))
    // Value = lowest score so far at that point and direction
    let mut memo: HashMap<((i32, i32), (i32, i32)), u32> = HashMap::new();
    let mut lowest = u32::MAX;

    dfs(&grid, &mut memo, &mut lowest, 0, start, (1, 0));

    Some(lowest)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (grid, start) = parse_input(input).unwrap();
    // Key = ((x, y), (dx, dy))
    // Value = lowest score so far at that point and direction
    let mut memo: HashMap<((i32, i32), (i32, i32)), u32> = HashMap::new();
    let mut best: HashSet<(i32, i32)> = HashSet::new();
    let tiles: Vec<(i32, i32)> = vec![];
    let lowest = 82460; // Answer to Part 1

    dfs2(
        &grid,
        &mut best,
        &mut memo,
        &lowest,
        0,
        start,
        (1, 0),
        tiles,
    );

    Some(best.len() as u32)
}
