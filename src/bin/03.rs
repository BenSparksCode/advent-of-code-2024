advent_of_code::solution!(3);

use regex::Regex;

pub fn get_lines(input: &str) -> Vec<&str> {
    let lines: Vec<&str> = input.lines().collect();

    lines
}

pub fn part_one(input: &str) -> Option<u32> {
    let line = get_lines(input).concat();
    let pattern = r"mul\((\d{1,3}),(\d{1,3})\)"; // capture groups for X and Y in mul(X,Y)
    let re = Regex::new(pattern).unwrap();
    let mut total = 0;

    // For each pattern match, we extract 3 captures:
    // 0) the full matched string
    // 1) the first number in the mul
    // 2) the second number in the mul
    for captures in re.captures_iter(&line) {
        total += captures[1].parse::<u32>().unwrap() * captures[2].parse::<u32>().unwrap();
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