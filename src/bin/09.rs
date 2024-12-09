advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let nums: Vec<i32> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();
    let mut nested: Vec<Vec<i32>> = vec![vec![]; nums.len()];
    let mut gap_offset = 0;

    // First expand to Vec<Vec<i32>> form with -1 as gaps
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

pub fn part_two(input: &str) -> Option<u64> {
    let nums: Vec<i32> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();
    let mut nested: Vec<Vec<i32>> = vec![vec![]; nums.len()];
    let mut gap_offset = 0;

    // First expand to Vec<Vec<i32>> form with -1 as gaps
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
    let last = all.len() - 1;

    // Front and back sliding window pointers
    //          -->              <--
    // [.....(f0...f1)........(b0...b1)..]
    let (mut b0, mut b1) = (last, last);
    let (mut f0, mut f1) = (0, 0);
    let mut last_file_num = i32::MAX;
    let mut first_gap = 0;

    loop {
        let mut file_too_big = false;

        // 1. Move b1 <- until it finds the num of next file to move
        let mut file_num = all[b1];
        while file_num == -1 || file_num >= last_file_num {
            b1 -= 1;
            if b1 == 0 {
                // Kill entire compacting process if we get here
                file_num = 0;
                break;
            }
            file_num = all[b1];
        }

        last_file_num = file_num;

        // Stop compacting if back pointers have reached file 0 (first file)
        if file_num == 0 {
            break;
        }

        // 2. Move b0 <- until its on the start of file_num file
        b0 = b1;
        while all[b0 - 1] == file_num {
            b0 -= 1;
        }

        // // Kill entire compacting process if we get here
        if b0 < first_gap {
            break;
        }

        // Cache gap size for efficiency
        let gap_size = b1 - b0;

        // 3. Update first gap if needed
        while all[first_gap] != -1 {
            first_gap += 1;
        }

        // 4. Move (f0, f1) until gap found. Start searching from first gap.
        (f0, f1) = (first_gap, first_gap);
        while f1 - f0 < gap_size {
            // 4.1. Move f0 -> until its on a gap start (-1)
            while all[f0] != -1 && f0 < b0 {
                f0 += 1;
            }

            // 4.2. Move f1 -> until correct gap size or non gap
            f1 = f0;
            while f1 - f0 < gap_size && f1 < b0 && all[f1 + 1] == -1 {
                f1 += 1;
            }

            // 4.3. If sliding windows overlapped, all gaps are too small
            if f1 >= b0 {
                file_too_big = true;
                break;
            }

            // Need a check to jump f0 into the non-gap char, to avoid infinite loop
            if f1 - f0 < gap_size {
                f0 += 1;
                f1 += 1;
            }
        }

        if file_too_big {
            continue;
        }

        // 5. If reached, swap (f0, f1) with (b0, b1)
        let mut j = 0;
        while j < gap_size + 1 {
            all.swap(f0 + j, b0 + j);
            j += 1;
        }
    }

    let mut checksum: u64 = 0;
    let mut i = 0;
    while i < all.len() {
        let n = all[i];
        // If gap, skip until non-gap found
        if n == -1 {
            i += 1;
            continue;
        }

        checksum += i as u64 * n as u64;
        i += 1;
    }

    Some(checksum)
}
