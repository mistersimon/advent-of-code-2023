fn permutations(processed: &str, remaining: &str, groups: Vec<usize>, within: bool) -> u64 {
    if remaining.is_empty() {
        return if groups.is_empty() || (groups.len() == 1 && groups[0] == 0) {
            // println!("{}{} => {:?} ->  Valid", processed, remaining, groups);
            1
        } else {
            0
        };
    }

    let first = remaining.chars().next().unwrap();

    // Try ? as both '.' or '#' without moving forward
    if first == '?' {
        return permutations(
            processed,
            &format!(".{}", &remaining[1..]),
            groups.clone(),
            within,
        ) + permutations(
            processed,
            &format!("#{}", &remaining[1..]),
            groups.clone(),
            within,
        );
    }

    let r = remaining[1..].to_string();
    if first == '.' && within {
        if groups[0] == 0 {
            let mut g = groups.clone();
            g.remove(0);
            return permutations(&format!("{}.", processed), &r, g, false);
        }
        return 0;
    }

    // Skip
    if first == '.' {
        if within && groups[0] == 0 {
            let mut g = groups.clone();
            g.remove(0);
            return permutations(&format!("{}.", processed), &r, g, within);
        } else if !within {
            return permutations(&format!("{}.", processed), &r, groups, within);
        }
    }

    // Process #
    if first == '#' && !groups.is_empty() && groups[0] > 0 {
        let mut g = groups.clone();
        g[0] -= 1;
        return permutations(&format!("{}#", processed), &r, g, true);
    }

    0
}

fn arrange_row(s: String) -> u64 {
    let m = s.split(" ").collect::<Vec<&str>>();

    let [record, group] = m[..] else { panic!() };

    let groups: Vec<usize> = group
        .split(',')
        .map(|m| m.to_string().parse::<usize>().unwrap())
        .collect();

    permutations("", record, groups, false)
}

pub fn part_one(lines: impl Iterator<Item = String>) -> u64 {
    let mut result = 0;
    for line in lines {
        result += arrange_row(line.clone());
    }
    result
}

pub fn part_two(_lines: impl Iterator<Item = String>) -> u64 {
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
