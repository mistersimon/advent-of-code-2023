fn delta(input: &Vec<i64>) -> Vec<i64> {
    let mut result: Vec<i64> = Vec::with_capacity(input.len() - 1);

    for n in 1..input.len() {
        result.push(input[n] - input[n - 1]);
    }

    result
}

fn extrapolate_forward(x: &Vec<i64>) -> i64 {
    if x.iter().all(|n| *n == 0) {
        return 0;
    }

    x.last().unwrap() + extrapolate_forward(&delta(x))
}

fn extrapolate_backward(x: &Vec<i64>) -> i64 {
    if x.iter().all(|n| *n == 0) {
        return 0;
    }

    x.first().unwrap() - crate::day_09::extrapolate_backward(&delta(x))
}

pub fn part_one(lines: impl Iterator<Item = String>) -> u64 {
    let mut result: i64 = 0;
    for line in lines {
        let nums = line
            .split(' ')
            .map(|m| m.to_string().parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        result += extrapolate_forward(&nums);
    }

    result.try_into().unwrap()
}

pub fn part_two(lines: impl Iterator<Item = String>) -> u64 {
    let mut result: i64 = 0;
    for line in lines {
        let nums = line
            .split(' ')
            .map(|m| m.to_string().parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        result += extrapolate_backward(&nums);
    }

    result.try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use rstest::rstest;

    const CASE_ONE: &str = indoc! {"
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
    "};

    #[rstest]
    #[case(vec ! [1, 3, 6, 10, 15, 21], vec ! [2, 3, 4, 5, 6])]
    #[case(vec ! [2, 3, 4, 5, 6], vec ! [1, 1, 1, 1])]
    #[case(vec ! [1, 1, 1, 1], vec ! [0, 0, 0 ])]
    fn delta_test(#[case] input: Vec<i64>, #[case] expected: Vec<i64>) {
        let output = super::delta(&input);
        assert_eq!(input.len() - 1, output.len());
        assert_eq!(output, expected);
    }

    #[rstest]
    #[case(vec ! [1, 1, 1], 1)]
    #[case(vec ! [1, 2, 3], 4)]
    #[case(vec ! [1, 3, 6, 10, 15, 21], 28)]
    fn extrapolate_forward_test(#[case] input: Vec<i64>, #[case] expected: i64) {
        let output = super::extrapolate_forward(&input);
        assert_eq!(output, expected);
    }

    #[rstest]
    #[case(vec ! [0, 3, 6, 9, 12, 15], - 3)]
    #[case(vec ! [1, 3, 6, 10, 15, 21], 0)]
    #[case(vec ! [10, 13, 16, 21, 30, 45], 5)]
    fn extrapolate_backward_test(#[case] input: Vec<i64>, #[case] expected: i64) {
        let output = super::extrapolate_backward(&input);
        assert_eq!(output, expected);
    }

    #[test]
    fn test_part_one_case_1() {
        let lines = CASE_ONE.lines().map(|line| line.to_string());
        let result = super::part_one(lines);
        assert_eq!(result, 114);
    }
}
