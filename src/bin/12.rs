use std::collections::HashSet;

advent_of_code::solution!(12);

const DIRS: [(i32, i32); 4] = [
    (0, -1), // UP
    (1, 0),  // RIGHT
    (0, 1),  // DOWN
    (-1, 0), // LEFT
];

pub fn dfs(
    grid: &Vec<Vec<char>>,
    explored: &mut HashSet<(i32, i32)>,
    max_x: i32,
    max_y: i32,
    p: (i32, i32),
    edges: &mut i32,
    area: &mut i32,
) {
    // If landed on already explored block -> stop
    if explored.contains(&p) {
        return;
    }

    // Increase area by this block and mark as explored
    *area += 1;
    explored.insert(p);
    let c = grid[p.1 as usize][p.0 as usize];

    for d in DIRS {
        // Check if next is in grid bounds
        let next = (p.0 + d.0, p.1 + d.1);
        if next.0 >= 0 && next.0 <= max_x && next.1 >= 0 && next.1 <= max_y {
            if grid[next.1 as usize][next.0 as usize] == c {
                // If same char in the next block keep exploring
                dfs(grid, explored, max_x, max_y, next, edges, area);
            } else {
                // Otherwise add to edges
                *edges += 1;
            }
        } else {
            // Must also increase edges if we are at the edge of the grid
            *edges += 1;
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let max_x = (grid[0].len() - 1) as i32;
    let max_y = (grid.len() - 1) as i32;

    let mut explored: HashSet<(i32, i32)> = HashSet::new();
    let mut total = 0;

    for (y, row) in grid.iter().enumerate() {
        for x in 0..row.len() {
            let p = (x as i32, y as i32);
            if !explored.contains(&p) {
                let mut edges = 0;
                let mut area = 0;

                dfs(&grid, &mut explored, max_x, max_y, p, &mut edges, &mut area);

                total += area * edges;
            }
        }
    }

    Some(total as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}
