use regex::Regex;
use std::fmt::format;

fn permutations(processed: String, mut remaining: String, groups: Vec<usize>, within: bool) -> u64 {
    // println!("{}|{} -> {:?}", processed, remaining, groups);
    // println!("{}^", " ".repeat(processed.len()));
    if remaining.is_empty() {
        return if groups.is_empty() || (groups.len() == 1 && groups[0] == 0) {
            // println!("{}{} => {:?} ->  Valid", processed, remaining, groups);
            1
        } else {
            0
        };
    }

    let first = remaining.chars().nth(0).unwrap();

    if first == '?' {
        // Try as both '.' or '#' without moving forward
        return permutations(
            processed.clone(),
            format!(".{}", remaining[1..].to_string()),
            groups.clone(),
            within,
        ) + permutations(
            processed.clone(),
            format!("#{}", remaining[1..].to_string()),
            groups.clone(),
            within,
        );
    }

    if first == '.' && within {
        if groups[0] == 0 {
            let mut g = groups.clone();
            g.remove(0);
            return permutations(
                format!("{}.", processed),
                remaining[1..].to_string(),
                g,
                false,
            );
        }
        return 0;
    }
    if first == '.' {
        // Skip
        return permutations(
            format!("{}.", processed),
            remaining[1..].to_string(),
            groups,
            within,
        );
    }

    // Process #
    if first == '#' && groups.len() > 0 && groups[0] > 0 {
        let mut g = groups.clone();
        g[0] -= 1;
        return permutations(
            format!("{}#", processed),
            remaining[1..].to_string(),
            g,
            true,
        );
    }

    // println!("{}{} = {:?} ->  Invalid", processed, remaining, groups);
    0
}

fn arrange_row(s: String) -> u64 {
    let m = s.split(" ").collect::<Vec<&str>>();

    let [record, group] = m[..] else { panic!() };

    let groups: Vec<usize> = group
        .split(',')
        .map(|m| m.to_string().parse::<usize>().unwrap())
        .collect();

    // println!("\n\n {} {}", record, group);
    println!("{} => {:?}", record, groups);
    permutations("".to_string(), record.to_string(), groups, false)
}

pub fn part_one(lines: impl Iterator<Item = String>) -> u64 {
    let mut result = 0;
    for line in lines {
        println!("{}", line.clone());
        let p = arrange_row(line.clone());
        result += p
    }
    result
}

pub fn part_two(lines: impl Iterator<Item = String>) -> u64 {
    todo!();
}
#[cfg(test)]
mod tests {
    use indoc::indoc;
    use rstest::rstest;

    const CASE_1: &str = indoc! {"
        ???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1
    "};

    #[rstest]
    #[case("# 1", 1)]
    #[case("??? 3", 1)]
    #[case("???.### 1,1,3", 1)]
    #[case("??..??...?##. 1,1,3", 4)]
    #[case("#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case("????.#...#... 4,1,1", 1)]
    #[case("????.######..#####. 1,6,5", 4)]
    #[case("?###???????? 3,2,1", 10)]
    #[case("???.?????.?# 1,1,2,1", 13)]
    fn test_arrange_row(#[case] input: String, #[case] expected: u64) {
        let output = super::arrange_row(input);
        assert_eq!(output, expected);
    }
    #[test]
    fn test_part_one_case_1() {
        let lines = CASE_1.lines().map(|line| line.to_string());
        let result = super::part_one(lines);
        assert_eq!(result, 21);
    }
    //
    // #[test]
    // fn test_part_two_case_1() {
    //     let lines = CASE_1.lines().map(|line| line.to_string());
    //     let result = super::part_two(lines);
    //     assert_eq!(result, 0);
    // }
}
