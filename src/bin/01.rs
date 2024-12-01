advent_of_code::solution!(1);

pub fn get_vecs_from_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let nums1: Vec<i32> = input
        .lines()
        .filter_map(|line| line.split_whitespace().nth(0)?.parse::<i32>().ok())
        .collect();

    let nums2: Vec<i32> = input
        .lines()
        .filter_map(|line| line.split_whitespace().nth(1)?.parse::<i32>().ok())
        .collect();

    (nums1, nums2)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut nums1, mut nums2) = get_vecs_from_input(input);

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
    let (nums1, nums2) = get_vecs_from_input(input);

    let mut total: i32 = 0;

    for n in nums1.iter() {
        total += n * nums2.iter().filter(|x| n == *x).count() as i32;
    }

    Some(total as u32)
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
