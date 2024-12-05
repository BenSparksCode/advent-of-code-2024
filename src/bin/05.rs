use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

pub fn structure_data(
    input: &str,
    need_rule_set: bool,
) -> (HashSet<(i32, i32)>, Vec<Vec<i32>>, HashMap<i32, Vec<i32>>) {
    let divider_idx = 1176;
    let lines = input.lines().collect::<Vec<&str>>();
    let mut afters: HashMap<i32, Vec<i32>> = HashMap::new();

    let rules: Vec<(i32, i32)> = lines[..divider_idx]
        .to_vec()
        .iter()
        .map(|line| {
            let nums: Vec<&str> = line.split("|").collect();
            (
                nums[0].parse::<i32>().unwrap(),
                nums[1].parse::<i32>().unwrap(),
            )
        })
        .collect();

    let updates: Vec<Vec<i32>> = lines[divider_idx + 1..]
        .to_vec()
        .iter()
        .map(|line| {
            line.split(",")
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let mut rules_set: HashSet<(i32, i32)> = HashSet::new();

    // Build HashMap of afters
    for r in &rules {
        afters.entry(r.0).or_default().push(r.1);
        if need_rule_set {
            rules_set.insert(*r);
        }
    }

    // Sort HashMap vector values
    for v in afters.values_mut() {
        v.sort();
    }

    (rules_set, updates, afters)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, updates, afters) = structure_data(input, false);
    let mut sum_mids = 0;

    // Search for rule violations
    for update in updates {
        let mut valid = true;
        let mut i: i32 = 1;
        while i < update.len() as i32 {
            let mut j: i32 = 0;
            while j < i {
                // Check if j is in i's afters = invalid
                if let Some(after) = afters.get(&update[i as usize]) {
                    if after.binary_search(&update[j as usize]).is_ok() {
                        valid = false;
                        break;
                    }
                }
                j += 1;
            }
            if !valid {
                break;
            }
            i += 1;
        }

        // Add middle num to sum if update is valid
        if valid {
            sum_mids += update[(update.len() - 1) / 2];
        }
    }

    Some(sum_mids as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, mut updates, afters) = structure_data(input, true);
    let mut sum_mids = 0;

    // Search for rule violations
    for update in updates.iter_mut() {
        let mut valid = true;
        let mut i: i32 = 1;
        while i < update.len() as i32 {
            let mut j: i32 = 0;
            while j < i {
                // Check if j is in i's afters = invalid
                if let Some(after) = afters.get(&update[i as usize]) {
                    if after.binary_search(&update[j as usize]).is_ok() {
                        valid = false;
                        break;
                    }
                }
                j += 1;
            }
            if !valid {
                break;
            }
            i += 1;
        }

        // If update not valid, fix order and add middle num to sum
        if !valid {
            let mut x = 1;
            while x < update.len() {
                let mut j = 0;
                while j < x {
                    // If there is a rule that says (x|j), swap them
                    if rules.contains(&(update[x], update[j])) {
                        update.swap(j, x);
                    }
                    j += 1;
                }
                x += 1;
            }
            sum_mids += update[(update.len() - 1) / 2];
        }
    }

    Some(sum_mids as u32)
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
