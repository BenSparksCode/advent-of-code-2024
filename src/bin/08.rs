use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let max_x = (grid[0].len() - 1) as i32;
    let max_y = (grid.len() - 1) as i32;

    // Map of c: [(x, y), (x, y)] antenna locations per char
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    // Find all antennas
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '.' {
                continue;
            }
            let entry = antennas.entry(*c).or_insert(vec![]);
            entry.push((x as i32, y as i32));
        }
    }

    // Iterate through char map, then through each vec to calculate if antinodes are on the map
    for (_, list) in &antennas {
        // For each array, take each antenna and check if the antinodes it creates with others are on map
        for (i, curr) in list.iter().enumerate() {
            // Find the 2 antinodes created by each prev antenna + current
            let mut j = 0;
            while j < i {
                let prev = list[j];

                // a1: (x, y) = (x1 + (x1 - x0), y1 + (y1 - y0))
                let a1 = (curr.0 + (curr.0 - prev.0), curr.1 + (curr.1 - prev.1));

                if a1.0 >= 0 && a1.0 <= max_x && a1.1 >= 0 && a1.1 <= max_y {
                    antinodes.insert(a1);
                }

                // Same as above, but swap x1 <> x0 and y1 <> y0
                let a2 = (prev.0 + (prev.0 - curr.0), prev.1 + (prev.1 - curr.1));

                if a2.0 >= 0 && a2.0 <= max_x && a2.1 >= 0 && a2.1 <= max_y {
                    antinodes.insert(a2);
                }

                j += 1;
            }
        }
    }

    Some(antinodes.len() as u32)
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
