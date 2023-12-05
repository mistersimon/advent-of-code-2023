use regex::Regex;

pub fn day_3a(input: impl Iterator<Item = String>) -> u32 {
    let mut result = 0;

    let lines = input.collect::<Vec<String>>();

    let ylen = lines.len() - 1;
    for (y, line) in lines.clone().into_iter().enumerate() {
        let xlen = line.len();
        // println!("{}/{} {}", y, ylen, line);
        let re = Regex::new(r"\d+").unwrap();

        'number: for m in re.find_iter(&line) {
            let n = m.as_str().parse::<u32>().unwrap();

            let mut adj: Vec<char> = Vec::new();

            // right
            if m.start() > 0 {
                adj.push(line.chars().nth(m.start() - 1).unwrap());
            }

            // Left
            if m.end() < line.len() {
                adj.push(line.chars().nth(m.end()).unwrap());
            }

            let s = if m.start() == 0 { 0 } else { m.start() - 1 };
            let e = if m.end() == xlen { xlen } else { m.end() + 1 };
            // above
            if y > 0 {
                for c in lines[y - 1][s..e].chars() {
                    adj.push(c);
                }
            }
            // Below
            if y < ylen {
                for c in lines[y + 1][s..e].chars() {
                    adj.push(c);
                }
            }

            for a in adj {
                if !a.is_ascii_digit() && a != '.' {
                    // println!("> Adding {} because of {}", n, a);
                    result += n;
                    continue 'number;
                }
            }
        }
    }

    result
}

fn find_number_start(s: &str) -> Option<u32> {
    let re = Regex::new(r"^\d+").unwrap();
    match re.find(s) {
        Some(n) => n.as_str().parse::<u32>().ok(),
        None => None,
    }
}

fn find_number_end(s: &str) -> Option<u32> {
    let re = Regex::new(r"\d+$").unwrap();
    match re.find(s) {
        Some(n) => n.as_str().parse::<u32>().ok(),
        None => None,
    }
}

fn find_number_middle(s: &str, i: usize) -> Option<u32> {
    let re = Regex::new(r"\d+").unwrap();
    for m in re.find_iter(s) {
        if (m.start() <= i) && (i <= m.end()) {
            return m.as_str().parse::<u32>().ok();
        }
    }

    None
}

pub fn day_3b(input: impl Iterator<Item = String>) -> u32 {
    let mut result = 0;

    let lines = input.collect::<Vec<String>>();

    let ylen = lines.len() - 1;

    for (y, line) in lines.clone().into_iter().enumerate() {
        // println!("{}/{} {}", y, ylen, line);
        let re = Regex::new(r"\*").unwrap();

        // println!("{}, {}", y, line);
        for m in re.find_iter(&line) {
            let mut nums: Vec<u32> = Vec::new();
            // right
            if m.start() > 0 {
                if let Some(n) = find_number_end(&line[..m.start()]) {
                    nums.push(n);
                }
            }

            // Left
            if m.end() < line.len() {
                if let Some(n) = find_number_start(&line[m.end()..]) {
                    nums.push(n);
                }
            }

            // up
            if y > 0 {
                if lines[y - 1].chars().nth(m.start()).unwrap() == '.' {
                    if m.start() > 0 {
                        if let Some(n) = find_number_end(&lines[y - 1][..m.start()]) {
                            nums.push(n);
                        }
                    }

                    if m.end() < line.len() {
                        if let Some(n) = find_number_start(&lines[y - 1][m.end()..]) {
                            nums.push(n);
                        }
                    }
                } else if let Some(n) = find_number_middle(&lines[y - 1], m.start()) {
                    nums.push(n);
                }
            }
            // down
            if y < ylen {
                if lines[y + 1].chars().nth(m.start()).unwrap() == '.' {
                    if m.start() > 0 {
                        if let Some(n) = find_number_end(&lines[y + 1][..m.start()]) {
                            nums.push(n);
                        }
                    }

                    if m.end() < line.len() {
                        if let Some(n) = find_number_start(&lines[y + 1][m.end()..]) {
                            nums.push(n);
                        }
                    }
                } else if let Some(n) = find_number_middle(&lines[y + 1], m.start()) {
                    nums.push(n);
                }
            }

            if nums.len() == 2 {
                result += nums[0] * nums[1];
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    #[test]
    fn test_day_3a() {
        let input = indoc! {"
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "};
        let lines = input.lines().map(|line| line.to_string());
        let result = super::day_3a(lines);
        assert_eq!(result, 4361);
    }

    #[test]
    fn test_day_3a1() {
        let input = indoc! {"
            ..35..6333
            ......#...
        "};
        let lines = input.lines().map(|line| line.to_string());
        let result = super::day_3a(lines);
        assert_eq!(result, 6333);
    }

    #[test]
    fn test_day_3b() {
        let input = indoc! {"
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "};
        let lines = input.lines().map(|line| line.to_string());
        let result = super::day_3b(lines);
        assert_eq!(result, 467835);
    }
}
