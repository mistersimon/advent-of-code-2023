const NORTH: (i64, i64) = (-1, 0);
const SOUTH: (i64, i64) = (1, 0);
const EAST: (i64, i64) = (0, 1);
const WEST: (i64, i64) = (0, -1);

type Coordinate = (usize, usize);
type Move = (i64, i64);

fn walk(
    map: &Vec<Vec<char>>,
    pos: (usize, usize),
    mov: (i64, i64),
) -> ((usize, usize), (i64, i64)) {
    let ylen = map.len();
    let xlen = map[0].len();
    if (pos.0 == 0 && mov == NORTH)
        || (pos.0 >= ylen && mov == SOUTH)
        || (pos.1 == 0 && mov == WEST)
        || (pos.1 == xlen && mov == EAST)
    {
        return (pos, (0, 0));
    }

    let next: (usize, usize) = (
        (i64::try_from(pos.0).unwrap() + mov.0).try_into().unwrap(),
        (i64::try_from(pos.1).unwrap() + mov.1).try_into().unwrap(),
    );

    let c = map[next.0][next.1];

    let next_move = match (c, mov) {
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

    (next, next_move)
}

fn get_start(map: &Vec<Vec<char>>) -> (Coordinate, Move, char) {
    let mut start: (usize, usize) = (0, 0);

    // Find start by scanning
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 'S' {
                start = (y, x);
            }
        }
    }

    // Find valid first moves by brute force
    let tos = [NORTH, SOUTH, EAST, WEST]
        .iter()
        .map(|m| {
            let (_, mov) = walk(map, start, *m);
            match mov {
                (0, 0) => (0i64, 0i64),
                _ => *m,
            }
        })
        .filter(|m| *m != (0, 0))
        .collect::<Vec<Move>>();

    let piece = match tos[..] {
        [NORTH, SOUTH] => '|',
        [NORTH, EAST] => 'L',
        [NORTH, WEST] => 'J',
        [SOUTH, EAST] => 'F',
        [SOUTH, WEST] => '7',
        [EAST, WEST] => '-',
        _ => 'S',
    };

    (start, tos[0], piece)
}

pub fn part_one(lines: impl Iterator<Item = String>) -> u64 {
    // Get map as matrix
    let map: Vec<Vec<char>> = lines
        .map(|m| m.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut steps = 0u64;

    let (start, mut mov, _) = get_start(&map);
    let mut pos = start;

    // Keep looping till we made it round
    while !(pos == start && steps > 0) {
        println!(
            "Step {:3}: At {} {:?}, moving {:?}",
            steps, map[pos.0][pos.1], pos, mov
        );
        (pos, mov) = walk(&map, pos, mov);
        steps += 1;
    }

    println!("Completed loop in {steps}");
    steps / 2
}

pub fn part_two(lines: impl Iterator<Item = String>) -> u64 {
    // Get map as matrix
    let mut map: Vec<Vec<char>> = lines
        .map(|m| m.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut pipe_map: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];

    let mut steps = 0u64;

    let (start, mut mov, piece) = get_start(&map);
    map[start.0][start.1] = piece;
    let mut pos = start;

    // Keep looping till we made it round
    while !(pos == start && steps > 0) {
        pipe_map[pos.0][pos.1] = true;
        (pos, mov) = walk(&map, pos, mov);
        steps += 1;
    }

    println!("Completed loop in {steps}");

    let mut area = 0u64;
    for (y, row) in map.iter().enumerate() {
        let mut is_inside = false;
        for (x, c) in row.iter().enumerate() {
            let is_pipe = pipe_map[y][x];

            if is_pipe && "|JL".contains(*c) {
                is_inside = !is_inside;
            }

            if !is_pipe && is_inside {
                area += 1;
            }

            if is_pipe {
                print!(
                    "{}",
                    match c {
                        '-' => '─',
                        '|' => '│',
                        'F' => '╭',
                        '7' => '╮',
                        'L' => '╰',
                        'J' => '╯',
                        _ => *c,
                    }
                )
            } else if is_inside {
                print!("I");
            } else {
                print!("0");
            }
        }
        println!();
    }

    area
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const CASE_ONE: &str = indoc! {"
        ..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ...
    "};

    const CASE_TWO: &str = indoc! {"
        ...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ...........
    "};

    const CASE_THREE: &str = indoc! {"
        FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L
    "};

    #[test]
    fn test_part_one_case_1() {
        let lines = CASE_ONE.lines().map(|line| line.to_string());
        let result = super::part_one(lines);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_part_two_case_2() {
        let lines = CASE_TWO.lines().map(|line| line.to_string());
        let result = super::part_two(lines);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_part_two_case_3() {
        let lines = CASE_THREE.lines().map(|line| line.to_string());
        let result = super::part_two(lines);
        assert_eq!(result, 10);
    }
}
