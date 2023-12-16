use regex::Regex;

fn hash(s: &str) -> usize {
    let mut hash = 0usize;
    for c in s.chars() {
        hash += usize::try_from(c as u32).unwrap();
        hash *= 17;
        hash %= 256;
    }

    hash
}

pub fn part_one(mut lines: impl Iterator<Item = String>) -> u64 {
    lines
        .next()
        .unwrap()
        .split(',')
        .map(hash)
        .sum::<usize>()
        .try_into()
        .unwrap()
}

pub fn part_two(mut lines: impl Iterator<Item = String>) -> u64 {
    let mut power = 0usize;
    let mut boxes: [Vec<(String, usize)>; 256] = [(); 256].map(|_| Vec::new());

    let steps = lines.next().unwrap();

    let re = Regex::new(r"(\w+)([-=])(\d*)").unwrap();

    for step in steps.split(',') {
        let caps = re.captures(step).unwrap();
        let label = caps.get(1).unwrap().as_str().to_string();
        let operation = caps.get(2).unwrap().as_str();
        let focal: usize = caps.get(3).unwrap().as_str().parse::<usize>().unwrap_or(0);

        let b = hash(&label);
        let lens = (label.clone(), focal);
        // println!("{b} {label} {operation} {focal}");

        if let Some(n) = boxes[b].iter().position(|x| x.0 == label) {
            if operation == "=" {
                boxes[b][n] = lens;
            } else {
                boxes[b].remove(n);
            }
        } else if operation == "=" {
            boxes[b].push(lens);
        }
    }

    for (b, boxx) in boxes.iter().enumerate() {
        for (i, (_, f)) in boxx.iter().enumerate() {
            power += (b + 1) * (i + 1) * f;
        }
    }

    power.try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    // use rstest::rstest;

    const CASE_1: &str = indoc! {"
        rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
    "};

    #[test]
    fn test_part_one_case_1() {
        let lines = CASE_1.lines().map(|line| line.to_string());
        let result = super::part_one(lines);
        assert_eq!(result, 1320);
    }

    #[test]
    fn test_part_two_case_1() {
        let lines = CASE_1.lines().map(|line| line.to_string());
        let result = super::part_two(lines);
        assert_eq!(result, 145);
    }
}
