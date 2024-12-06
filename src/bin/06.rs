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

    // If gaurd has moved off the grid, stop moving
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
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
