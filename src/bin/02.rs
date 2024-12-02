advent_of_code::solution!(2);

pub fn get_reports(input: &str) -> Vec<Vec<i32>> {
    let reports: Vec<Vec<i32>> = input
        .lines()
        .filter_map(|line| {
            Some(
                line.split_whitespace()
                    .filter_map(|x| x.parse::<i32>().ok())
                    .collect::<Vec<i32>>(),
            )
        })
        .collect();

    reports
}

pub fn is_report_safe(r: &Vec<i32>) -> bool {
    let mut direction = 0;

    for (i, n) in r.iter().enumerate() {
        if i == 0 {
            // start comparing from 2nd element, vs prev element
            continue;
        }

        let diff = r[i - 1] - n;

        // Unsafe if:
        // - diff more than 3
        // - diff is 0 (no change vs prev)
        // - direction changed
        if diff.abs() > 3 || diff == 0 || direction * diff < 0 {
            return false;
        }

        direction = diff;
    }

    true
}

pub fn rm_idx(vec: &Vec<i32>, i: usize) -> Vec<i32> {
    vec![vec[..i].to_vec(), vec[i + 1..].to_vec()].concat()
}

pub fn part_one(input: &str) -> Option<u32> {
    let reports = get_reports(input);
    let mut count = 0;

    for r in reports {
        if !is_report_safe(&r) {
            continue;
        }
        count += 1;
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports = get_reports(input);
    let mut count = 0;

    for r in reports {
        if is_report_safe(&r) {
            // Report is safe without dampener
            count += 1;
        } else {
            // Use dampener - check if safe with each level removed
            let mut i = 0;
            while i < r.len() {
                if is_report_safe(&rm_idx(&r, i)) {
                    // Found safe report using dampener
                    count += 1;
                    break;
                }
                i += 1;
            }
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
