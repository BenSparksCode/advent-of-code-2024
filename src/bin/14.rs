advent_of_code::solution!(14);

pub fn parse_input(line: &str) -> Option<((i32, i32), (i32, i32))> {
    let mut parts = line.split_whitespace();
    let pos_part = parts.next()?; // "p=0,4"
    let vel_part = parts.next()?; // "v=3,-3"

    // Extract position
    let pos_values = pos_part.strip_prefix("p=")?.split_once(',')?;
    let pos = (pos_values.0.parse().ok()?, pos_values.1.parse().ok()?);

    // Extract velocity
    let vel_values = vel_part.strip_prefix("v=")?.split_once(',')?;
    let vel = (vel_values.0.parse().ok()?, vel_values.1.parse().ok()?);

    Some((pos, vel))
}

pub fn part_one(input: &str) -> Option<u32> {
    // Vec of [(x,y) , (dx, dy)]
    let bots: Vec<((i32, i32), (i32, i32))> = input
        .lines()
        .map(|line| parse_input(line).unwrap())
        .collect();

    let time = 100; // 100 seconds of movement
    let width = 101;
    let height = 103;
    let mut quads = [0; 4];

    // Positions after travelling and teleports
    let after: Vec<(i32, i32)> = bots
        .iter()
        .map(|b| {
            (
                (((b.0 .0 + time * b.1 .0) % width) + width) % width,
                (((b.0 .1 + time * b.1 .1) % height) + height) % height,
            )
        })
        .collect();

    // Q1 | Q2
    // --------
    // Q3 | Q4

    for p in after {
        // Skip non quadrant bots
        if p.0 == (width - 1) / 2 || p.1 == (height - 1) / 2 {
            continue;
        }
        if p.0 < width / 2 {
            // LEFT
            if p.1 < height / 2 {
                // Q1
                quads[0] += 1;
            } else {
                // Q3
                quads[2] += 1;
            }
        } else {
            // RIGHT
            if p.1 < height / 2 {
                // Q2
                quads[1] += 1;
            } else {
                // Q4
                quads[3] += 1;
            }
        }
    }

    Some(quads.iter().product())
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
