advent_of_code::solution!(4);

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

// FOR PART 1
// (x,y) is the location that needs to be checked in this call
// (dx,dy) is the direction of word search
pub fn search_for_word(
    grid: &Vec<Vec<char>>,
    letter_matches: usize,
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
) -> bool {
    // If we found the end 'S' in the prev search iteration, matches will be 4 here.
    if letter_matches == 4 {
        return true;
    } else {
        // Check if (x,y) is within grid bounds to safely check its value
        if x < 0 || x >= grid[0].len() as i32 || y < 0 || y >= grid.len() as i32 {
            return false;
        }

        // Check if next char in search direction is next char in "XMAS"
        if grid[y as usize][x as usize] != XMAS[letter_matches] {
            return false;
        }

        // Move one step in search direction, then in the next call, that block will be checked
        return search_for_word(grid, letter_matches + 1, x + dx, y + dy, dx, dy);
    }
}

// FOR PART 1
pub fn count_matches_from_loc(grid: &Vec<Vec<char>>, x: i32, y: i32) -> u32 {
    // Only search from Xs - start of XMAS
    if grid[y as usize][x as usize] != 'X' {
        return 0;
    }

    let mut count = 0;

    // North
    if search_for_word(grid, 1, x, y - 1, 0, -1) {
        count += 1;
    }

    // North East
    if search_for_word(grid, 1, x + 1, y - 1, 1, -1) {
        count += 1;
    }

    // East
    if search_for_word(grid, 1, x + 1, y, 1, 0) {
        count += 1;
    }

    // South East
    if search_for_word(grid, 1, x + 1, y + 1, 1, 1) {
        count += 1;
    }

    // South
    if search_for_word(grid, 1, x, y + 1, 0, 1) {
        count += 1;
    }

    // South West
    if search_for_word(grid, 1, x - 1, y + 1, -1, 1) {
        count += 1;
    }

    // West
    if search_for_word(grid, 1, x - 1, y, -1, 0) {
        count += 1;
    }

    // North West
    if search_for_word(grid, 1, x - 1, y - 1, -1, -1) {
        count += 1;
    }

    count
}

// FOR PART 2
pub fn is_valid_x_mas(grid: &Vec<Vec<char>>, x: i32, y: i32) -> bool {
    // Only valid if A is in center
    if grid[y as usize][x as usize] != 'A' {
        return false;
    }

    // Check NW -> SE diagonal
    let c1 = grid[(y - 1) as usize][(x - 1) as usize];
    let c2 = grid[(y + 1) as usize][(x + 1) as usize];
    if !(c1 != c2 && (c1 == 'M' || c1 == 'S') && (c2 == 'M' || c2 == 'S')) {
        return false;
    }

    // Check NE -> SW diagonal
    let c1 = grid[(y - 1) as usize][(x + 1) as usize];
    let c2 = grid[(y + 1) as usize][(x - 1) as usize];
    if !(c1 != c2 && (c1 == 'M' || c1 == 'S') && (c2 == 'M' || c2 == 'S')) {
        return false;
    }

    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut y: i32 = 0;
    let mut total = 0;

    while y < grid.len() as i32 {
        let mut x = 0;
        while x < grid[0].len() as i32 {
            total += count_matches_from_loc(&grid, x, y);
            x += 1;
        }
        y += 1;
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut y: i32 = 1;
    let mut total = 0;

    // Now, we only iterate pointer to the center of the X with a 1-width border throughout the grid.
    // And this center char must be 'A'. So each diagonal needs 1x M and 1x S exactly to be valid.

    while y < (grid.len() - 1) as i32 {
        let mut x = 1;
        while x < (grid[0].len() - 1) as i32 {
            if is_valid_x_mas(&grid, x, y) {
                total += 1;
            }
            x += 1;
        }
        y += 1;
    }

    Some(total)
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
