pub fn day_1a(input: impl Iterator<Item = String>) -> u32 {
    let mut sum: u32 = 0;
    for line in input {
        let mut chars: [char; 2] = ['0', '0'];
        for c in line.chars() {
            if c.is_ascii_digit() {
                chars[1] = c;
                if chars[0] == '0' {
                    chars[0] = c;
                }
            }
        }
        sum += format!("{}{}", chars[0], chars[1]).parse::<u32>().unwrap();
    }

    sum
}

pub fn day_1b(input: impl Iterator<Item = String>) -> u32 {
    let nums = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut sum: u32 = 0;
    for line in input {
        let mut chars: [u32; 2] = [0, 0];
        for (i, c) in line.chars().enumerate() {
            let mut d = None;
            if c.is_ascii_digit() {
                d = c.to_digit(10);
            } else {
                for (v, n) in nums.iter().enumerate() {
                    if line[i..].starts_with(n) {
                        d = Some(u32::try_from(v + 1).unwrap());
                        break;
                    }
                }
            }
            if d.is_none() {
                continue;
            }

            chars[1] = d.unwrap();
            if chars[0] == 0 {
                chars[0] = d.unwrap();
            }
        }
        sum += format!("{}{}", chars[0], chars[1]).parse::<u32>().unwrap();
    }

    sum
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    #[test]
    fn test_day_1a() {
        let input = indoc! {"
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        "};
        let lines = input.lines().map(|line| line.to_string());
        let result = super::day_1a(lines);
        assert_eq!(result, 12 + 38 + 15 + 77);
    }

    #[test]
    fn test_day_1b() {
        let input = indoc! {"
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        "};
        let lines = input.lines().map(|line| line.to_string());
        let result = super::day_1b(lines);
        assert_eq!(result, 281);
    }
}
