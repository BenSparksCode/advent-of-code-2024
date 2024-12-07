advent_of_code::solution!(7);

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

    // True if target found in either of the X or + children
    return search_combos(nums, target, result * nums[i], i + 1)
        || search_combos(nums, target, result + nums[i], i + 1);
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
