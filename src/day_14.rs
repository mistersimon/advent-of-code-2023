use cached::proc_macro::cached;

type Map = Vec<Vec<char>>;
fn slide_north(mut map: Map) -> Map {
    let mut count: Vec<usize> = vec![0; map[0].len()];
    for j in 0..map.len() {
        for i in 0..map[j].len() {
            if map[j][i] == 'O' {
                map[j][i] = '.';
                map[count[i]][i] = 'O';
                count[i] += 1;
            }
            if map[j][i] == '#' {
                count[i] = j + 1
            }
        }
    }
    map
}

fn slide_south(mut map: Map) -> Map {
    let l = map.len() - 1;
    let mut count: Vec<usize> = vec![l; map[0].len()];
    for x in 0..map.len() {
        let j = l - x;
        for i in 0..map[j].len() {
            if map[j][i] == 'O' {
                map[j][i] = '.';
                map[count[i]][i] = 'O';
                count[i] = count[i].saturating_sub(1);
            }
            if map[j][i] == '#' && j > 0 {
                count[i] = j - 1;
            }
        }
    }
    map
}

fn slide_west(mut map: Map) -> Map {
    for line in &mut map {
        let mut count: usize = 0;
        for i in 0..line.len() {
            if line[i] == 'O' {
                line[i] = '.';
                line[count] = 'O';
                count += 1;
            }
            if line[i] == '#' {
                count = i + 1
            }
        }
    }
    map
}

fn slide_east(mut map: Map) -> Map {
    let w = map[0].len() - 1;
    for line in &mut map {
        let mut count: usize = w;
        for x in 0..line.len() {
            let i = w - x;
            if line[i] == 'O' {
                line[i] = '.';
                line[count] = 'O';
                count = count.saturating_sub(1);
            }
            if line[i] == '#' && i > 0 {
                count = i - 1
            }
        }
    }
    map
}

fn calc_loading(map: &Vec<Vec<char>>) -> u64 {
    let mut loading = 0u64;
    for (j, line) in map.iter().enumerate() {
        for c in line.iter() {
            if *c == 'O' {
                loading += u64::try_from(map.len() - j).unwrap();
            }
        }
    }

    loading
}

pub fn part_one(lines: impl Iterator<Item = String>) -> u64 {
    let map = lines
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let map = slide_north(map);

    calc_loading(&map)
}

#[cached]

fn cycle(mut map: Map) -> Map {
    for _ in 0..10_000 {
        map = slide_north(map);
        map = slide_west(map);
        map = slide_south(map);
        map = slide_east(map);
    }

    map
}
pub fn part_two(lines: impl Iterator<Item = String>) -> u64 {
    let mut map = lines
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    for _ in 0..100_000 {
        map = cycle(map);
    }

    calc_loading(&map)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    // use rstest::rstest;

    const CASE_1: &str = indoc! {"
        O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....
    "};

    #[test]
    fn test_part_one_case_1() {
        let lines = CASE_1.lines().map(|line| line.to_string());
        let result = super::part_one(lines);
        assert_eq!(result, 136);
    }

    #[test]
    fn test_part_two_case_1() {
        let lines = CASE_1.lines().map(|line| line.to_string());
        let result = super::part_two(lines);
        assert_eq!(result, 64);
    }
}
