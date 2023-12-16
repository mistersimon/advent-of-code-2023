use std::cmp;
use std::collections::HashSet;

const NORTH: (i32, i32) = (-1, 0);
const SOUTH: (i32, i32) = (1, 0);
const EAST: (i32, i32) = (0, 1);
const WEST: (i32, i32) = (0, -1);

fn energized(map: &Vec<Vec<char>>, p: (usize, usize), d: (i32, i32)) -> u64 {
    let mut energized = vec![vec![0; map[0].len()]; map.len()];

    let mut beams = vec![(p, d)];

    let mut cache: HashSet<((usize, usize), (i32, i32))> = HashSet::new();

    while let Some((pos, dir)) = beams.pop() {
        let c = map[pos.0][pos.1];

        energized[pos.0][pos.1] = 1;

        // Avoid loops
        if cache.contains(&(pos, dir)) {
            continue;
        } else {
            cache.insert((pos, dir));
        }

        let dirs = match (c, dir) {
            ('/', NORTH) => vec![EAST],
            ('/', SOUTH) => vec![WEST],
            ('/', EAST) => vec![NORTH],
            ('/', WEST) => vec![SOUTH],
            ('\\', NORTH) => vec![WEST],
            ('\\', SOUTH) => vec![EAST],
            ('\\', EAST) => vec![SOUTH],
            ('\\', WEST) => vec![NORTH],
            ('|', EAST | WEST) => vec![NORTH, SOUTH],
            ('-', NORTH | SOUTH) => vec![EAST, WEST],
            _ => vec![dir],
        };

        for d in dirs {
            if (pos.0 == 0 && d == NORTH)
                || (pos.0 == map.len() - 1 && d == SOUTH)
                || (pos.1 == 0 && d == WEST)
                || (pos.1 == map[0].len() - 1 && d == EAST)
            {
                continue;
            }

            // println!("{:?} {:?} {} next: {:?}", pos, dir, c, d);
            beams.push((
                (
                    usize::try_from(i32::try_from(pos.0).unwrap() + d.0).unwrap(),
                    usize::try_from(i32::try_from(pos.1).unwrap() + d.1).unwrap(),
                ),
                d,
            ))
        }
        // println!("Beams: {:?}", beams);
    }

    // for l in energized.iter() {
    //     println!("{:?}", l);
    // }

    energized
        .iter()
        .map(|m| m.iter().sum::<usize>())
        .sum::<usize>()
        .try_into()
        .unwrap()
}
pub fn part_one(lines: impl Iterator<Item = String>) -> u64 {
    let map = lines
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    energized(&map, (0, 0), (0, 1))
}

pub fn part_two(lines: impl Iterator<Item = String>) -> u64 {
    let map = lines
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut max = 0;
    for i in 0..map.len() {
        // Left edge, going East
        max = cmp::max(max, energized(&map, (i, 0), EAST));
        // Right edge, going East
        max = cmp::max(max, energized(&map, (i, map[0].len() - 1), WEST));
    }
    for i in 0..map[0].len() {
        // TOP edge, going south
        max = cmp::max(max, energized(&map, (0, i), SOUTH));
        // Bottom edge, going North
        max = cmp::max(max, energized(&map, (map.len() - 1, i), NORTH));
    }
    max
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    // use rstest::rstest;

    const CASE_1: &str = indoc! {"
        .|...\\....
        |.-.\\.....
        .....|-...
        ........|.
        ..........
        .........\\
        ..../.\\\\..
        .-.-/..|..
        .|....-|.\\
        ..//.|....
    "};

    #[test]
    fn test_part_one_case_1() {
        let lines = CASE_1.lines().map(|line| line.to_string());
        let result = super::part_one(lines);
        assert_eq!(result, 46);
    }

    #[test]
    fn test_part_two_case_1() {
        let lines = CASE_1.lines().map(|line| line.to_string());
        let result = super::part_two(lines);
        assert_eq!(result, 51);
    }
}
