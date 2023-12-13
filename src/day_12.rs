use std::collections::HashMap;

#[allow(clippy::type_complexity)]

fn solve<'a>(
    s: &'a [char],
    in_group: Option<usize>,
    groups: &'a [usize],
    cache: &mut HashMap<(&'a [char], Option<usize>, &'a [usize]), u64>,
) -> u64 {
    if s.is_empty() {
        return match in_group {
            Some(n) if groups == [n] => 1,
            None if groups.is_empty() => 1,
            _ => 0,
        };
    }

    if let Some(result) = cache.get(&(s, in_group, groups)) {
        return *result;
    }

    let result = match (s[0], in_group, groups) {
        ('.', None, _) => solve(&s[1..], None, groups, cache),
        ('.' | '?', Some(n), [e, ..]) if n == *e => solve(&s[1..], None, &groups[1..], cache),
        ('#' | '?', Some(n), [e, ..]) if n < *e => solve(&s[1..], Some(n + 1), groups, cache),
        ('#', None, [_, ..]) => solve(&s[1..], Some(1), groups, cache),
        ('?', None, _) => {
            solve(&s[1..], None, groups, cache) + solve(&s[1..], Some(1), groups, cache)
        }
        _ => 0,
    };

    cache.insert((s, in_group, groups), result);
    // println!("{}{} => {:?} ->  Invalid", processed, remaining, groups);
    result
}

pub fn part_one(lines: impl Iterator<Item = String>) -> u64 {
    let mut result = 0;
    for line in lines {
        let m = line.split(' ').collect::<Vec<&str>>();

        let [record, group] = m[..] else { panic!() };

        let groups: Vec<usize> = group
            .split(',')
            .map(|m| m.to_string().parse::<usize>().unwrap())
            .collect();

        let records: Vec<char> = record.chars().collect();
        result += solve(&records, None, &groups, &mut HashMap::new());
    }

    result
}

pub fn part_two(lines: impl Iterator<Item = String>) -> u64 {
    let copies = 5;
    let mut result = 0;
    for line in lines {
        let m = line.split(' ').collect::<Vec<&str>>();

        let [record, group] = m[..] else { panic!() };

        let record = format!("{}?", record).repeat(copies);
        let record = record[..record.len() - 1].to_string();
        let group = format!("{},", group).repeat(copies);

        let groups: Vec<usize> = group
            .split(',')
            .filter_map(|m| m.to_string().parse::<usize>().ok())
            .collect();

        let records: Vec<char> = record.chars().collect();

        result += solve(&records, None, &groups, &mut HashMap::new());
    }

    result
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
    fn test_part_one_row(#[case] input: String, #[case] expected: u64) {
        let output = super::part_one(input.lines().map(|m| m.to_string()));
        assert_eq!(output, expected);
    }
    #[test]
    fn test_part_one_case_1() {
        let lines = CASE_1.lines().map(|line| line.to_string());
        let result = super::part_one(lines);
        assert_eq!(result, 21);
    }

    #[rstest]
    #[case("???.### 1,1,3", 1)]
    #[case(".??..??...?##. 1,1,3", 16384)]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case("????.#...#... 4,1,1", 16)]
    #[case("????.######..#####. 1,6,5", 2500)]
    #[case("?###???????? 3,2,1", 506250)]
    fn test_part_two_row(#[case] input: String, #[case] expected: u64) {
        let output = super::part_two(input.lines().map(|m| m.to_string()));
        assert_eq!(output, expected);
    }

    #[test]
    fn test_part_two_case_1() {
        let lines = CASE_1.lines().map(|line| line.to_string());
        let result = super::part_two(lines);
        assert_eq!(result, 525152);
    }
}
