use regex::Regex;
use std::cmp;

fn apply_map(i: u64, map: &Vec<[u64; 3]>) -> u64 {
    for m in map {
        if m[1] <= i && i < (m[1] + m[2]) {
            return m[0] + i - m[1];
        }
    }

    i
}
pub fn get_and_apply_mapping(mut seeds: Vec<u64>, lines: impl Iterator<Item = String>) -> u64 {
    let mut maps: Vec<Vec<[u64; 3]>> = Vec::new();

    // Map for a specific category
    let mut map: Vec<[u64; 3]> = Vec::new();

    for line in lines {
        if line.is_empty() {
            continue;
        }

        let m: Vec<u64> = Regex::new(r"\d+")
            .unwrap()
            .find_iter(&line)
            .map(|m| m.as_str().parse::<u64>().unwrap())
            .collect();

        if !m.is_empty() {
            assert!(m.len() == 3, "Error");
            map.push(m.try_into().unwrap());
            continue;
        }

        if !map.is_empty() {
            maps.push(map);
            map = Vec::new();
        }
    }
    maps.push(map);

    for m in maps {
        seeds = seeds.iter().map(|i| apply_map(*i, &m)).collect();
        // println!("{:?}", seeds);
    }

    let mut result = u64::MAX;
    for n in seeds.clone() {
        result = cmp::min(result, n);
    }

    result
}

pub fn day_5a(mut lines: impl Iterator<Item = String>) -> u64 {
    let seeds: Vec<u64> = Regex::new(r"\d+")
        .unwrap()
        .find_iter(&lines.by_ref().take(1).next().unwrap())
        .map(|m| m.as_str().parse::<u64>().unwrap())
        .collect();

    get_and_apply_mapping(seeds, lines)
}

pub fn day_5b(mut lines: impl Iterator<Item = String>) -> u64 {
    let mut seeds: Vec<u64> = Regex::new(r"\d+")
        .unwrap()
        .find_iter(&lines.by_ref().take(1).next().unwrap())
        .map(|m| m.as_str().parse::<u64>().unwrap())
        .collect();
    let pairs: Vec<[u64; 2]> = seeds.chunks(2).map(|x| x.try_into().unwrap()).collect();

    seeds = pairs.iter().flat_map(|p| p[0]..(p[0] + p[1])).collect();

    get_and_apply_mapping(seeds, lines)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use rstest::rstest;
    const EXAMPLE: &str = indoc! {"
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4
        "};

    #[rstest]
    #[case(79, 81)]
    #[case(14, 14)]
    #[case(55, 57)]
    #[case(13, 13)]
    fn apply_map_test(#[case] input: u64, #[case] expected: u64) {
        let map = vec![[50, 98, 2], [52, 50, 48]];
        let output = super::apply_map(input, &map);
        assert_eq!(output, expected);
    }

    #[test]
    fn test_day_5a() {
        let lines = EXAMPLE.lines().map(|line| line.to_string());
        let result = super::day_5a(lines);
        assert_eq!(result, 35);
    }

    #[test]
    fn test_day_5b() {
        let lines = EXAMPLE.lines().map(|line| line.to_string());
        let result = super::day_5b(lines);
        assert_eq!(result, 46);
    }
}
