advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let line = input.lines().collect::<Vec<&str>>().concat();
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
    let line = input.lines().collect::<Vec<&str>>().concat();
    // Now includes matches for "do()" and "don't()" to toggle mul functionality
    let pattern = r"(?P<mul>mul\((\d{1,3}),(\d{1,3})\))|(?P<enable>do\(\))|(?P<disable>don't\(\))";
    let re = Regex::new(pattern).unwrap();
    let mut total = 0;
    let mut enabled = true;

    for captures in re.captures_iter(&line) {
        if captures.name("mul").is_some() {
            if !enabled {
                continue;
            }
            // If the match was a "mul", we now have 4 capture groups:
            // 0) the named "mul" full pattern match
            // 1) the original unnamed full pattern match
            // 2) the x group in "mul(x,y)"     <-- we want this
            // 3) the y group in "mul(x,y)"     <-- we also want this
            total += captures[2].parse::<u32>().unwrap() * captures[3].parse::<u32>().unwrap();
        } else if captures.name("enable").is_some() {
            enabled = true;
        } else if captures.name("disable").is_some() {
            enabled = false;
        }
    }

    Some(total)
}
