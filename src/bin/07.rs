advent_of_code::solution!(7);

// Oh shit, he got the u64 digits lookup table 😳
fn digit_length(n: u64) -> u32 {
    const POWERS_OF_10: [u64; 19] = [
        1,
        10,
        100,
        1000,
        10_000,
        100_000,
        1_000_000,
        10_000_000,
        100_000_000,
        1_000_000_000,
        10_000_000_000,
        100_000_000_000,
        1_000_000_000_000,
        10_000_000_000_000,
        100_000_000_000_000,
        1_000_000_000_000_000,
        10_000_000_000_000_000,
        100_000_000_000_000_000,
        1_000_000_000_000_000_000,
    ];
    POWERS_OF_10.iter().position(|&p| n < p).unwrap_or(19) as u32
}

// Recursively search every combination of operators and nums
pub fn search_combos(nums: &Vec<u64>, target: u64, result: u64, i: usize) -> bool {
    // Return false early if result grew larger than target - cannot get smaller
    if result > target {
        return false;
    }

    // If we've used all nums, check if result in this branch match the target. Return outcome.
    if i == nums.len() {
        return result == target;
    }

    let next = nums[i];

    // True if target found in either of the X or + children
    return search_combos(nums, target, result * next, i + 1)
        || search_combos(nums, target, result + next, i + 1);
}

// Recursively search every combination of operators and nums. Incl. the || operator 😳
pub fn search_combos_3ops(nums: &Vec<u64>, target: u64, result: u64, i: usize) -> bool {
    // Return false early if result grew larger than target - cannot get smaller
    if result > target {
        return false;
    }

    // If we've used all nums, check if result in this branch match the target. Return outcome.
    if i == nums.len() {
        return result == target;
    }

    let next = nums[i];

    return search_combos_3ops(nums, target, result * next, i + 1)
        || search_combos_3ops(nums, target, result + next, i + 1)
        || search_combos_3ops(
            nums,
            target,
            result * 10u64.pow(digit_length(next)) + next,
            i + 1,
        );
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines: Vec<(u64, Vec<u64>)> = input
        .lines()
        .filter_map(|line| {
            line.split_once(": ").and_then(|(total, rest)| {
                let total = total.parse::<u64>().ok()?;
                let rest = rest
                    .split_whitespace()
                    .map(|n| n.parse::<u64>().ok())
                    .collect::<Option<Vec<_>>>()?;
                Some((total, rest))
            })
        })
        .collect();
    let mut total = 0;

    for line in lines {
        if search_combos(&line.1, line.0, line.1[0], 1) {
            total += line.0;
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<(u64, Vec<u64>)> = input
        .lines()
        .filter_map(|line| {
            line.split_once(": ").and_then(|(total, rest)| {
                let total = total.parse::<u64>().ok()?;
                let rest = rest
                    .split_whitespace()
                    .map(|n| n.parse::<u64>().ok())
                    .collect::<Option<Vec<_>>>()?;
                Some((total, rest))
            })
        })
        .collect();
    let mut total = 0;

    for line in lines {
        if search_combos_3ops(&line.1, line.0, line.1[0], 1) {
            total += line.0;
        }
    }

    Some(total)
}
