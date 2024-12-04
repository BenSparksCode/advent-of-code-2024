advent_of_code::solution!(4);

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

// (x,y) is the location that needs to be checked in this call
// (dx,dy) is the direction of word search
pub fn search_for_word(
    lines: &Vec<Vec<char>>,
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
        if x < 0 || x >= lines[0].len() as i32 || y < 0 || y >= lines.len() as i32 {
            return false;
        }

        // Check if next char in search direction is next char in "XMAS"
        if lines[y as usize][x as usize] != XMAS[letter_matches] {
            return false;
        }

        // Move one step in search direction, then in the next call, that block will be checked
        return search_for_word(lines, letter_matches + 1, x + dx, y + dy, dx, dy);
    }
}

pub fn count_matches_from_loc(lines: &Vec<Vec<char>>, x: i32, y: i32) -> u32 {
    // Only search from Xs - start of XMAS
    if lines[y as usize][x as usize] != 'X' {
        return 0;
    }

    let mut count = 0;

    // North
    if search_for_word(lines, 1, x, y - 1, 0, -1) {
        count += 1;
    }

    // North East
    if search_for_word(lines, 1, x + 1, y - 1, 1, -1) {
        count += 1;
    }

    // East
    if search_for_word(lines, 1, x + 1, y, 1, 0) {
        count += 1;
    }

    // South East
    if search_for_word(lines, 1, x + 1, y + 1, 1, 1) {
        count += 1;
    }

    // South
    if search_for_word(lines, 1, x, y + 1, 0, 1) {
        count += 1;
    }

    // South West
    if search_for_word(lines, 1, x - 1, y + 1, -1, 1) {
        count += 1;
    }

    // West
    if search_for_word(lines, 1, x - 1, y, -1, 0) {
        count += 1;
    }

    // North West
    if search_for_word(lines, 1, x - 1, y - 1, -1, -1) {
        count += 1;
    }

    count
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut y: i32 = 0;
    let mut total = 0;

    while y < lines.len() as i32 {
        let mut x = 0;
        while x < lines[0].len() as i32 {
            total += count_matches_from_loc(&lines, x, y);
            x += 1;
        }
        y += 1;
    }

    Some(total)
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
