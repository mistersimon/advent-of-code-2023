pub fn part_one(lines: impl Iterator<Item = String>) -> u64 {
    todo!();
}

pub fn part_two(lines: impl Iterator<Item = String>) -> u64 {
    todo!();
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    // use rstest::rstest;

    const CASE_1: &str = indoc! {"
        ..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ...
    "};

    #[test]
    fn test_part_one_case_1() {
        let lines = CASE_1.lines().map(|line| line.to_string());
        let result = super::part_one(lines);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part_two_case_1() {
        let lines = CASE_1.lines().map(|line| line.to_string());
        let result = super::part_two(lines);
        assert_eq!(result, 0);
    }
}
