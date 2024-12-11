advent_of_code::solution!(11);

// Returns 2 nums (bool indicates if 2nd num applicable)
pub fn blink(stone: u64) -> (u64, u64, bool) {
    if stone == 0 {
        // is 0 -> becomes 1
        return (1, 0, false);
    } else if stone.to_string().len() % 2 == 0 {
        // TODO optimize this: no str -> int conversions
        // Even length -> split into 2

        let s = stone.to_string();

        // YIKES...
        return (
            (&s[..s.len() / 2]).parse().unwrap(),
            (&s[s.len() / 2..]).parse().unwrap(),
            true,
        );
    } else {
        // Otherwise X 2024
        return (stone * 2024, 0, false);
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut stones: Vec<u64> = input
        .split_whitespace()
        .map(|num| num.parse::<u64>().unwrap())
        .collect();
    let blinks = 25;

    let mut b = 0; // b for blink index
    while b < blinks {
        let mut s = 0; // s for stone index
        while s < stones.len() {
            let new_stones = blink(stones[s]);
            stones[s] = new_stones.0;

            // If blink created a split, need to merge another stone in
            if new_stones.2 {
                stones.insert(s + 1, new_stones.1);
                s += 1;
            }
            s += 1;
        }
        b += 1;
    }

    Some(stones.len() as u64)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}
