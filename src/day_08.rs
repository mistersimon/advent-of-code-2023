use regex::Regex;
use std::collections::HashMap;

fn gcd(a: u64, b: u64) -> u64 {
    match ((a, b), (a & 1, b & 1)) {
        ((x, y), _) if x == y => y,
        ((0, x), _) | ((x, 0), _) => x,
        ((x, y), (0, 1)) | ((y, x), (1, 0)) => gcd(x >> 1, y),
        ((x, y), (0, 0)) => gcd(x >> 1, y >> 1) << 1,
        ((x, y), (1, 1)) => {
            let (x, y) = (x.min(y), x.max(y));
            gcd((y - x) >> 1, x)
        }
        _ => unreachable!(),
    }
}

fn lcm(xs: Vec<u64>) -> u64 {
    xs.iter().fold(1, |ans, x| (x * ans) / gcd(*x, ans))
}

pub fn part_one(mut lines: impl Iterator<Item = String>) -> u64 {
    let re = Regex::new(r"[A-Z]{3}").unwrap();

    // let binding = lines.next().unwrap();
    let instructions: Vec<usize> = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => panic!(),
        })
        .collect();

    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for line in lines {
        if line.is_empty() {
            continue;
        }

        let mut iter = re.find_iter(&line).map(|m| m.as_str());
        let key = iter.next().unwrap().to_string();
        let value = iter.map(|m| m.to_string()).collect::<Vec<String>>();

        map.insert(key, value);
    }

    let mut pos: String = "AAA".to_string();
    let mut steps: usize = 0;

    // println!("{:?}", map.get(pos).unwrap());
    while pos != "ZZZ" {
        let direction = instructions[steps % instructions.len()];
        let nodes = map.get(&pos).unwrap();

        // println!("{steps} {pos} {} {:?}", direction, nodes);
        pos = nodes[direction].to_string();
        steps += 1;
    }

    steps.try_into().unwrap()
}

pub fn part_two(mut lines: impl Iterator<Item = String>) -> u64 {
    let re = Regex::new(r"[A-Z0-9]{3}").unwrap();

    // let binding = lines.next().unwrap();
    let instructions: Vec<usize> = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => panic!(),
        })
        .collect();

    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let mut positions: Vec<String> = Vec::new();

    for line in lines {
        if line.is_empty() {
            continue;
        }

        let mut iter = re.find_iter(&line).map(|m| m.as_str());
        let key = iter.next().unwrap().to_string();
        let value = iter.map(|m| m.to_string()).collect::<Vec<String>>();

        map.insert(key.clone(), value);
        if key.chars().nth(2).unwrap() == 'A' {
            positions.push(key.to_string())
        }
    }

    let mut steps: Vec<u64> = Vec::new();

    for mut pos in positions {
        let mut step: usize = 0;

        // println!("{:?}", map.get(pos).unwrap());
        while pos.chars().nth(2).unwrap() != 'Z' {
            let direction = instructions[step % instructions.len()];
            let nodes = map.get(&pos).unwrap();

            // println!("{steps} {pos} {} {:?}", direction, nodes);
            pos = nodes[direction].to_string();
            step += 1;
        }

        steps.push(step.try_into().unwrap())
    }

    lcm(steps)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const CASE_ONE: &str = indoc! {"
        RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)
    "};

    const CASE_TWO: &str = indoc! {"
        LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)

    "};

    #[test]
    fn test_part_one_case_1() {
        let lines = CASE_ONE.lines().map(|line| line.to_string());
        let result = super::part_one(lines);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part_one_case_2() {
        let lines = CASE_TWO.lines().map(|line| line.to_string());
        let result = super::part_one(lines);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_part_two() {
        const EXAMPLE: &str = indoc! {"
        LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)
    "};
        let lines = EXAMPLE.lines().map(|line| line.to_string());
        let result = super::part_two(lines);
        assert_eq!(result, 6);
    }
}
