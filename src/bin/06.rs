use std::collections::HashSet;

advent_of_code::solution!(6);

pub fn get_start(grid: &Vec<Vec<char>>) -> (i32, i32) {
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '^' {
                return (x as i32, y as i32);
            }
        }
    }
    (0, 0)
}

pub fn do_move(
    grid: &Vec<Vec<char>>,
    p: &mut (i32, i32),
    dir: &mut (i32, i32),
    moving: &mut bool,
    max_x: i32,
    max_y: i32,
) {
    let mut next = (p.0 + dir.0, p.1 + dir.1);

    // If guard has moved off the grid, stop moving
    if next.0 < 0 || next.1 < 0 || next.0 > max_x || next.1 > max_y {
        *moving = false;
        return;
    }

    // Check if turn needed
    if grid[next.1 as usize][next.0 as usize] == '#' {
        if *dir == (0, -1) {
            // UP -> RIGHT
            *dir = (1, 0);
        } else if *dir == (1, 0) {
            // RIGHT -> DOWN
            *dir = (0, 1);
        } else if *dir == (0, 1) {
            // DOWN -> LEFT
            *dir = (-1, 0);
        } else if *dir == (-1, 0) {
            // LEFT -> UP
            *dir = (0, -1);
        }

        // Update next to reflect turn + move
        next = (p.0 + dir.0, p.1 + dir.1);
    }

    // Update p to reflect move
    *p = next;
}

// Part 2 variant of the do_move() function from Part 1. Returns true if loop found.
pub fn do_move_track_turns(
    grid: &Vec<Vec<char>>,
    turns: &mut HashSet<(i32, i32)>,
    prev: &mut (i32, i32),
    p: &mut (i32, i32),
    dir: &mut (i32, i32),
    moving: &mut bool,
    max_x: i32,
    max_y: i32,
) -> bool {
    let next = (p.0 + dir.0, p.1 + dir.1);

    // If guard has moved off the grid, stop moving
    if next.0 < 0 || next.1 < 0 || next.0 > max_x || next.1 > max_y {
        *moving = false;
        return false;
    }

    // Check if turn needed
    if grid[next.1 as usize][next.0 as usize] == '#' {
        // Already turned right on this spot = LOOP FOUND
        // Only check if we've turned here before, if that turn wasn't on the move immediately before this one
        if prev != p && turns.contains(p) {
            return true;
        }

        // Add turn to set of turns found
        turns.insert(*p);

        if *dir == (0, -1) {
            // UP -> RIGHT
            *dir = (1, 0);
        } else if *dir == (1, 0) {
            // RIGHT -> DOWN
            *dir = (0, 1);
        } else if *dir == (0, 1) {
            // DOWN -> LEFT
            *dir = (-1, 0);
        } else if *dir == (-1, 0) {
            // LEFT -> UP
            *dir = (0, -1);
        }

        // If we turned in this round, do not move forward. Just update prev.
        *prev = *p;
        return false;
    }

    // Move forward - update both prev and p
    *prev = *p;
    *p = next;

    return false;
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let max_x = (grid[0].len() - 1) as i32;
    let max_y = (grid.len() - 1) as i32;

    let mut positions: HashSet<(i32, i32)> = HashSet::new();
    let mut p = get_start(&grid); // (x, y) pos on grid
    let mut dir = (0, -1); // direction using (x, y), always starts as UP
    let mut moving = true;

    while moving {
        positions.insert(p);
        do_move(&grid, &mut p, &mut dir, &mut moving, max_x, max_y);
    }

    Some(positions.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let max_x = (grid[0].len() - 1) as i32;
    let max_y = (grid.len() - 1) as i32;
    let start = get_start(&grid);
    let mut og_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut turns: HashSet<(i32, i32)>;
    let mut loops = 0;

    // First, build set of coords in original path from Part 1

    let mut prev;
    let mut p = start;
    let mut dir = (0, -1); // UP
    let mut moving = true;

    while moving {
        og_positions.insert(p);
        do_move(&grid, &mut p, &mut dir, &mut moving, max_x, max_y);
    }

    // Then, check each empty spot if adding obstacle results in loop

    let mut y = 0;
    while y <= max_y {
        let mut x = 0;
        while x <= max_x {
            // if already obstacle or the guard's start -> skip
            if grid[y as usize][x as usize] != '.' {
                x += 1;
                continue;
            }

            // if (x, y) not in og_positions, wouldn't affect path -> skip
            if !og_positions.contains(&(x, y)) {
                x += 1;
                continue;
            }

            // Add new obstacle
            grid[y as usize][x as usize] = '#';

            // Reset path traversal vars
            prev = (-1, -1);
            p = start;
            dir = (0, -1); // UP
            moving = true;
            turns = HashSet::new();

            while moving {
                if do_move_track_turns(
                    &grid,
                    &mut turns,
                    &mut prev,
                    &mut p,
                    &mut dir,
                    &mut moving,
                    max_x,
                    max_y,
                ) {
                    // do_move_track_turns returns true if loop found -> break
                    loops += 1;
                    break;
                }
            }

            // Remove new obstacle before next iteration
            grid[y as usize][x as usize] = '.';

            x += 1;
        }
        y += 1;
    }

    Some(loops as u32)
}
