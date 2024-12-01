advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut nums1: Vec<i32> = input
        .lines()
        .filter_map(|line| line.split_whitespace().nth(0))
        .filter_map(|word| word.parse::<i32>().ok())
        .collect();

    let mut nums2: Vec<i32> = input
        .lines()
        .filter_map(|line| line.split_whitespace().nth(1))
        .filter_map(|word| word.parse::<i32>().ok())
        .collect();

    nums1.sort();
    nums2.sort();

    let mut total: i32 = 0;
    let mut i = 0;

    while i < nums1.len() {
        total += (nums1[i] - nums2[i]).abs();

        i += 1;
    }

    Some(total as u32)
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
