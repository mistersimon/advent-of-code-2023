const NORTH: (i64, i64) = (-1, 0);
const SOUTH: (i64, i64) = (1, 0);
const EAST: (i64, i64) = (0, 1);
const WEST: (i64, i64) = (0, -1);

pub fn part_one(lines: impl Iterator<Item = String>) -> u64 {
    // Get map as matrix
    let map: Vec<Vec<char>> = lines
        .map(|m| m.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let ylen = map.len();
    let xlen = map[0].len();

    let get_next = |pos: (usize, usize), from: (i64, i64)| -> ((usize, usize), (i64, i64)) {
        if (pos.0 == 0 && from == NORTH)
            || (pos.0 >= ylen && from == SOUTH)
            || (pos.1 == 0 && from == WEST)
            || (pos.1 == xlen && from == EAST)
        {
            println!("Cannot move {:?}", from);
            return (pos, (0i64, 0i64));
        }

        let next: (usize, usize) = (
            (i64::try_from(pos.0).unwrap() + from.0).try_into().unwrap(),
            (i64::try_from(pos.1).unwrap() + from.1).try_into().unwrap(),
        );

        let c = map[next.0][next.1];

        let to = match (c, from) {
            ('7', NORTH) => WEST,
            ('7', EAST) => SOUTH,
            ('F', NORTH) => EAST,
            ('F', WEST) => SOUTH,
            ('J', EAST) => NORTH,
            ('J', SOUTH) => WEST,
            ('L', WEST) => NORTH,
            ('L', SOUTH) => EAST,
            ('|', NORTH) => NORTH,
            ('|', SOUTH) => SOUTH,
            ('-', WEST) => WEST,
            ('-', EAST) => EAST,
            _ => (0, 0),
        };

        (next, to)
    };

    // Find the start
    let mut start: (usize, usize) = (0, 0);
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 'S' {
                start = (y, x);
            }
        }
    }

    println!("At {} {:?}", map[start.0][start.1], start);

    let mut steps = 0u64;
    let mut pos = (0usize, 0usize);
    let mut mov = (0i64, 0i64);

    // Find start move by brute force
    steps += 1;
    for m in [NORTH, SOUTH, EAST, WEST] {
        (pos, mov) = get_next(start, m);
        if mov != (0, 0) {
            break;
        }
    }
    // println!("At {} {:?} after moving {:?}", map[pos.0][pos.1], pos, mov);

    // Loop through back to the start
    while pos != start {
        (pos, mov) = get_next(pos, mov);
        // println!("At {} {:?} after moving {:?}", map[pos.0][pos.1], pos, mov);
        steps += 1;
    }

    println!("Completed loop in {steps}");
    // println!("{:?}", map);
    steps / 2
}

pub fn part_two(_lines: impl Iterator<Item = String>) -> u64 {
    todo!();
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use rstest::rstest;

    const CASE_ONE: &str = indoc! {"
        ..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ...
    "};

    #[test]
    fn test_part_one_case_1() {
        let lines = CASE_ONE.lines().map(|line| line.to_string());
        let result = super::part_one(lines);
        assert_eq!(result, 8);
    }
}
