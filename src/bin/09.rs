advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let nums: Vec<i32> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();
    let mut nested: Vec<Vec<i32>> = vec![vec![]; nums.len()];
    let mut gap_offset = 0;

    // First expand to Vec<Vec<>> form with -1 as gaps
    for (i, num) in nums.iter().enumerate() {
        if i % 2 == 1 {
            // Fill with -1 to represent gaps
            nested[i] = vec![-1; *num as usize];
            gap_offset += 1;
            continue;
        }
        nested[i] = vec![i as i32 - gap_offset; *num as usize];
    }

    // Then flatten to Vec<i32>
    let mut all: Vec<i32> = nested.into_iter().flatten().collect();
    let mut checksum: u64 = 0;

    // Then iterate through, pop off back as used, and calculate checksum
    let mut i = 0;
    while i < all.len() {
        let n = all[i];
        // If gap, pop number from back.
        if n == -1 {
            // Keep popping nums off the back until we get a non-gap
            let mut back = all.pop().unwrap();
            while back == -1 {
                back = all.pop().unwrap();
            }

            checksum += i as u64 * back as u64;
            i += 1;
            continue;
        }

        checksum += i as u64 * n as u64;
        i += 1;
    }

    Some(checksum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}
