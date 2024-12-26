use std::collections::{HashMap, HashSet};

advent_of_code::solution!(23);

pub fn parse_input(input: &str) -> (HashMap<&str, Vec<&str>>, HashSet<(&str, &str)>) {
    let mut all = HashMap::new();
    let mut exists = HashSet::new();

    for line in input.lines() {
        let (left, right) = line.split_once("-").unwrap();

        // Add left
        let val = all.entry(left).or_insert(vec![]);
        val.push(right);
        // Add right
        let val = all.entry(right).or_insert(vec![]);
        val.push(left);

        exists.insert((left, right));
        exists.insert((right, left));
    }

    (all, exists)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (all, exists) = parse_input(input);
    let mut threes: HashSet<(&str, &str, &str)> = HashSet::new();
    let letter = 't';

    for &pc in all.keys() {
        let pc_list: &Vec<&str> = all.get(pc).unwrap();
        for (i, &second) in pc_list.iter().enumerate() {
            for j in i..pc_list.len() {
                let third = pc_list[j];
                // Check pc, second, third are all linked
                if exists.contains(&(pc, second))
                    && exists.contains(&(pc, third))
                    && exists.contains(&(second, third))
                {
                    // Check at least one starts with 't'
                    if pc.chars().nth(0).unwrap() == letter
                        || second.chars().nth(0).unwrap() == letter
                        || third.chars().nth(0).unwrap() == letter
                    {
                        let mut arr: [&str; 3] = [pc, second, third];
                        arr.sort();
                        threes.insert((arr[0], arr[1], arr[2]));
                    }
                }
            }
        }
    }

    Some(threes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}
