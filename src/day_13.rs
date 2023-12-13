fn find_row_reflect(grid: &Vec<Vec<char>>, smudges: usize) -> Option<usize> {
    'outer: for idx in 0..grid.len() - 1 {
        let mut a = idx as i64;
        let mut b = idx + 1;

        let mut diff = 0;
        while a >= 0 && b < grid.len() {
            for j in 0..grid[0].len() {
                if grid[a as usize][j] != grid[b][j] {
                    diff += 1
                }

                if diff > smudges {
                    continue 'outer;
                }
            }
            a -= 1;
            b += 1;
        }
        if diff == smudges {
            return Some(idx + 1);
        }
    }

    None
}

fn find_col_reflect(grid: &Vec<Vec<char>>, max: usize) -> Option<usize> {
    'outer: for idx in 0..grid[0].len() - 1 {
        let mut a = idx as i64;
        let mut b = idx + 1;

        let mut diff = 0;
        while a >= 0 && b < grid[0].len() {
            for g in grid {
                if g[a as usize] != g[b] {
                    diff += 1
                }

                if diff > max {
                    continue 'outer;
                }
            }
            a -= 1;
            b += 1;
        }
        if diff == max {
            return Some(idx + 1);
        }
    }
    None
}

pub fn main(lines: impl Iterator<Item = String>, smudges: usize) -> u64 {
    let mut result = 0;
    let input = lines.collect::<Vec<String>>().join("\n");
    let maps = input
        .split("\n\n")
        .map(|m| m.to_string())
        .collect::<Vec<String>>();

    for map in maps {
        let grid = map
            .lines()
            .map(|m| m.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        if let Some(n) = find_row_reflect(&grid, smudges) {
            result += n * 100;
        } else if let Some(c) = find_col_reflect(&grid, smudges) {
            result += c;
        } else {
            println!("Didn't find reflection");
        }
    }

    result.try_into().unwrap()
}
pub fn part_one(lines: impl Iterator<Item = String>) -> u64 {
    main(lines, 0)
}

pub fn part_two(lines: impl Iterator<Item = String>) -> u64 {
    main(lines, 1)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    // use rstest::rstest;

    const CASE_1: &str = indoc! {"
        #.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.

        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#
    "};

    #[test]
    fn test_part_one_case_1() {
        let lines = CASE_1.lines().map(|line| line.to_string());
        let result = super::part_one(lines);
        assert_eq!(result, 405);
    }

    #[test]
    fn test_part_two_case_1() {
        let lines = CASE_1.lines().map(|line| line.to_string());
        let result = super::part_two(lines);
        assert_eq!(result, 400);
    }
}
