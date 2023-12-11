use std::vec;

fn main(lines: impl Iterator<Item = String>, expansion_factor: usize) -> u64 {
    let map: Vec<Vec<char>> = lines
        .map(|m| m.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    let mut empty_col: Vec<bool> = vec![true; map.len()];
    let mut empty_row: Vec<bool> = vec![true; map[0].len()];

    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '#' {
                galaxies.push((y, x));
                empty_col[y] = false;
                empty_row[x] = false;
            }
        }
    }

    let collapse = |v: Vec<bool>| {
        v.iter()
            .enumerate()
            .filter_map(|(i, is_empty)| if *is_empty { Some(i) } else { None })
            .collect::<Vec<usize>>()
    };

    let empty_rows = collapse(empty_row);
    let empty_cols = collapse(empty_col);

    // println!("{:?}", galaxies);
    // println!("{:?}", empty_rows);
    // println!("{:?}", empty_cols);

    // Space out the galaxies
    galaxies = galaxies
        .iter()
        .map(|(y, x)| {
            let dx = empty_rows
                .iter()
                .position(|n| x < n)
                .unwrap_or(empty_rows.len());

            let dy = empty_cols
                .iter()
                .position(|n| y < n)
                .unwrap_or(empty_cols.len());

            (
                *y + dy * (expansion_factor - 1usize),
                *x + dx * (expansion_factor - 1usize),
            )
        })
        .collect();

    // Get distance between each pair
    let mut distance = 0usize;
    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            distance +=
                galaxies[j].0.abs_diff(galaxies[i].0) + galaxies[j].1.abs_diff(galaxies[i].1)
        }
    }

    distance.try_into().unwrap()
}

pub fn part_one(lines: impl Iterator<Item = String>) -> u64 {
    main(lines, 2)
}

pub fn part_two(lines: impl Iterator<Item = String>) -> u64 {
    main(lines, 1_000_000)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const CASE_1: &str = indoc! {"
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
    "};

    #[test]
    fn test_part_one_case_1() {
        let lines = CASE_1.lines().map(|line| line.to_string());
        let result = super::part_one(lines);
        assert_eq!(result, 374);
    }

    #[test]
    fn test_part_two_case_1a() {
        let lines = CASE_1.lines().map(|line| line.to_string());
        let result = super::main(lines, 10);
        assert_eq!(result, 1030);
    }

    #[test]
    fn test_part_two_case_1b() {
        let lines = CASE_1.lines().map(|line| line.to_string());
        let result = super::main(lines, 100);
        assert_eq!(result, 8410);
    }
}
