advent_of_code::solution!(22);

const PRUNE: u64 = 16777216;

pub fn next(secret: u64) -> u64 {
    let mut next = (secret ^ (secret * 64)) % PRUNE;
    next = (next ^ (next / 32)) % PRUNE;
    next = (next ^ (next * 2048)) % PRUNE;
    next
}

pub fn part_one(input: &str) -> Option<u64> {
    let secrets: Vec<u64> = input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect();
    let end = 2000;
    let mut sum = 0;

    for s in secrets {
        let mut s2 = s;
        for _ in 0..end {
            s2 = next(s2);
        }
        sum += s2;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}
