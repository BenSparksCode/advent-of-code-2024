use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

advent_of_code::solution!(20);

const DIRS: [(i32, i32); 4] = [
    (0, -1), // UP
    (1, 0),  // RIGHT
    (0, 1),  // DOWN
    (-1, 0), // LEFT
];

pub fn parse_input(input: &str) -> (Vec<Vec<char>>, (i32, i32), (i32, i32)) {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut start: (i32, i32) = (-1, -1);
    let mut end: (i32, i32) = (-1, -1);

    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 'S' {
                start = (x as i32, y as i32);
                continue;
            }
            if *c == 'E' {
                end = (x as i32, y as i32);
                continue;
            }
        }
        if start.0 != -1 && end.0 != -1 {
            break;
        }
    }

    (grid, start, end)
}

// Get all tiles in the dists HashMap, and put into vec in order from start to end.
// NOTE: This only works because there's only 1 valid route in the map. Thus we use all traversed tiles.
pub fn get_path_vec_from_dists(dists: &HashMap<(i32, i32), i32>) -> Vec<(i32, i32)> {
    let mut entries: Vec<((i32, i32), i32)> = dists.iter().map(|(&k, &v)| (k, v)).collect();
    entries.sort_by(|a, b| b.1.cmp(&a.1));
    // Map sorted (x, y) coords only into vec form, and return
    entries.into_iter().map(|(xy, _)| xy).collect()
}

// Traverse using Djikstra. Build Vec of path tiles and HashMap of dists to end.
pub fn traverse(
    grid: &Vec<Vec<char>>,
    end: (i32, i32),
) -> (Vec<(i32, i32)>, HashMap<(i32, i32), i32>) {
    let mut min_heap = BinaryHeap::new();
    let mut dists: HashMap<(i32, i32), i32> = HashMap::new();
    let max_x = grid[0].len() as i32;
    let max_y = grid.len() as i32;

    // Explore from end to start
    dists.insert(end, 0);
    min_heap.push(Reverse((0, end)));

    while let Some(Reverse((curr_dist, p))) = min_heap.pop() {
        // Explore p's neighbors
        for d in DIRS {
            let next = (p.0 + d.0, p.1 + d.1);

            // Skip neighbor if out of bounds
            if next.0 < 0 || next.0 > max_x || next.1 < 0 || next.1 > max_y {
                continue;
            }
            // Skip neighbor if wall
            if grid[next.1 as usize][next.0 as usize] == '#' {
                continue;
            }

            // Update next's dist to end, to new shortest. And add to min_heap for exploring.
            let next_dist = curr_dist + 1;
            if next_dist < *dists.get(&next).unwrap_or(&i32::MAX) {
                dists.insert(next, next_dist);
                min_heap.push(Reverse((next_dist, next)));
            }
        }
    }

    // Return sorted vec of path tiles, and the dists HashMap
    (get_path_vec_from_dists(&dists), dists)
}

// Part 1:
// - First, get path from end to start with Djikstra.
// - While traversing, build distances from end HashMap, and vec of path tiles from start to end.
// - Then, iterate through tiles from start to end:
// -- for each tile, check for valid shortcuts of 1 or 2 gap in all directions.
// -- Increment cheats count for each shortcut that saves at least 100 steps.
pub fn part_one(input: &str) -> Option<u32> {
    let (grid, _, end) = parse_input(input);
    let (path, dists) = traverse(&grid, end);
    let min_saved = 100; // Min picoseconds saved by shortcut
    let mut num_cheats = 0;

    // Iterate through path from start to end, looking for shortcuts
    for t in path {
        for d in DIRS {
            // Skip to next dir if next tile is on the path
            if dists.contains_key(&(t.0 + d.0, t.1 + d.1)) {
                continue;
            }

            // First check if gap=1 is valid shortcut
            let mut target = (t.0 + 2 * d.0, t.1 + 2 * d.1);
            if dists.contains_key(&target) {
                if dists.get(&t).unwrap() - dists.get(&target).unwrap() >= min_saved + 1 {
                    num_cheats += 1;
                    continue; // if gap=1 skip is valid, don't count gap=2 skip as well
                }
            }

            // Else check if gap=2 is valid shortcut
            target = (t.0 + 3 * d.0, t.1 + 3 * d.1);
            if dists.contains_key(&target) {
                if dists.get(&t).unwrap() - dists.get(&target).unwrap() >= min_saved + 2 {
                    num_cheats += 1;
                }
            }
        }
    }

    Some(num_cheats)
}

// Part 2:
// - Same as Part 1, except:
// - When we iterate through the path tiles, find the Manhattan distance between tile t,
// and all tiles ahead of it in the path.
// - Then check that the Manhattan distance is <= cheat length, and the time saved is enough.
pub fn part_two(input: &str) -> Option<u32> {
    let (grid, _, end) = parse_input(input);
    let (path, dists) = traverse(&grid, end);
    let min_saved = 100; // Min picoseconds saved by shortcut
    let cheat_len = 20;
    let mut num_cheats = 0;

    // Iterate through path from start to end, looking for shortcuts
    for (i, t) in path.iter().enumerate() {
        for j in (i + min_saved)..path.len() {
            let target = path[j];

            // Manhattan distance between tile t and target tile
            let man_dist = (t.0 - target.0).abs() + (t.1 - target.1).abs();
            if man_dist > cheat_len {
                continue;
            }

            let time_saved = dists.get(t).unwrap() - dists.get(&target).unwrap() - man_dist;
            if time_saved < min_saved as i32 {
                continue;
            }

            num_cheats += 1;
        }
    }

    Some(num_cheats)
}
