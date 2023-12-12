use regex::Regex;
use std::{cmp, collections::HashSet};

fn get_nums(s: &str) -> Vec<u32> {
    let re = Regex::new(r"\d+").unwrap();

    re.find_iter(s)
        .map(|m| m.as_str().parse::<u32>().unwrap())
        .collect()
}

fn get_set(s: &str) -> HashSet<u32> {
    let mut set: HashSet<u32> = HashSet::new();
    let re = Regex::new(r"\d+").unwrap();

    for m in re.find_iter(s) {
        set.insert(m.as_str().parse::<u32>().unwrap());
    }

    set
}

pub fn day_4a(input: impl Iterator<Item = String>) -> u32 {
    let mut result = 0;

    for line in input {
        let re = Regex::new(r"[|\:]").unwrap();
        let parts: Vec<String> = re.split(&line).map(|x| x.to_string()).collect();
        assert!(parts.len() == 3, "something went wrong");

        let set = get_set(&parts[1]);
        let mut points = 0;
        for n in get_nums(&parts[2]) {
            if set.contains(&n) {
                points += 1
            }
        }
        if points > 0 {
            result += u32::pow(2, points - 1)
        }
    }

    result
}

pub fn day_4b(input: impl Iterator<Item = String>) -> u32 {
    let lines = input.collect::<Vec<String>>();

    let mut result: u32 = 0;
    let mut copies = vec![1; lines.len()];

    let ylen = lines.len() - 1;
    for (y, line) in lines.into_iter().enumerate() {
        let re = Regex::new(r"[|\:]").unwrap();
        let parts: Vec<String> = re.split(&line).map(|x| x.to_string()).collect();
        assert!(parts.len() == 3, "something went wrong");

        let set = get_set(&parts[1]);
        let mut points = 0;
        for n in get_nums(&parts[2]) {
            if set.contains(&n) {
                points += 1
            }
        }
        if points > 0 && y < ylen {
            let yend = cmp::min(ylen, y + points);
            for i in (y + 1)..(yend + 1) {
                // println!(
                //     "Card {} with {} points made {} copies of Card {}",
                //     y + 1,
                //     points,
                //     copies[y],
                //     i + 1
                // );
                copies[i] += copies[y];
            }
        }

        result += copies[y];
    }

    // 1,2,4,8,14,1
    // println!("{:?}", copies);
    result
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    #[test]
    fn test_day_4a() {
        let input = indoc! {"
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "};
        let lines = input.lines().map(|line| line.to_string());
        let result = super::day_4a(lines);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_day_4b() {
        let input = indoc! {"
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "};
        let lines = input.lines().map(|line| line.to_string());
        let result = super::day_4b(lines);
        assert_eq!(result, 30);
    }
}
