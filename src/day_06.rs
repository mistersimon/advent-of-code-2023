use regex::Regex;

fn get_ways(t: u64, r: u64) -> u64 {
    let mut result = 0;
    for n in 1..(t - 1) {
        let d = n * (t - n);
        if d > r {
            result += 1;
        }
    }
    result
}

pub fn day_6a(mut lines: impl Iterator<Item = String>) -> u64 {
    let mut result = 1;
    let re = Regex::new(r"\d+").unwrap();
    let times: Vec<u64> = re
        .find_iter(&lines.by_ref().take(1).next().unwrap())
        .map(|m| m.as_str().parse::<u64>().unwrap())
        .collect();
    let record: Vec<u64> = re
        .find_iter(&lines.by_ref().take(1).next().unwrap())
        .map(|m| m.as_str().parse::<u64>().unwrap())
        .collect();

    for (t, r) in times.iter().zip(record.iter()) {
        result *= get_ways(*t, *r);
    }
    result
}

pub fn day_6b(mut lines: impl Iterator<Item = String>) -> u64 {
    let re = Regex::new(r"\d+").unwrap();
    let time: u64 = re
        .find_iter(&lines.by_ref().take(1).next().unwrap())
        .map(|m| m.as_str())
        .collect::<Vec<&str>>()
        .join("")
        .parse::<u64>()
        .unwrap();
    let record: u64 = re
        .find_iter(&lines.by_ref().take(1).next().unwrap())
        .map(|m| m.as_str())
        .collect::<Vec<&str>>()
        .join("")
        .parse::<u64>()
        .unwrap();

    get_ways(time, record)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use rstest::rstest;
    const EXAMPLE: &str = indoc! {"
        Time:      7  15   30
        Distance:  9  40  200
    "};

    #[rstest]
    #[case(7, 9, 4)]
    #[case(15, 40, 8)]
    #[case(30, 200, 9)]
    fn apply_map_test(#[case] t: u64, #[case] d: u64, #[case] expected: u64) {
        let output = super::get_ways(t, d);
        assert_eq!(output, expected);
    }

    #[test]
    fn test_day_6a() {
        let lines = EXAMPLE.lines().map(|line| line.to_string());
        let result = super::day_6a(lines);
        assert_eq!(result, 288);
    }

    #[test]
    fn test_day_6b() {
        let lines = EXAMPLE.lines().map(|line| line.to_string());
        let result = super::day_6b(lines);
        assert_eq!(result, 71503);
    }
}
