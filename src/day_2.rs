use regex::Regex;

pub fn day_2a(input: impl Iterator<Item = String>) -> u32 {
    let mut result: u32 = 0;
    let re = Regex::new(r"[\;\:]").unwrap();

    // only 12 red cubes, 13 green cubes, and 14 blue cubes?

    'outer: for line in input {
        let game: u32 = Regex::new(r"\d+")
            .unwrap()
            .find(&line)
            .map(|m| m.as_str())
            .unwrap_or("0")
            .to_string()
            .parse()
            .unwrap();

        let parts: Vec<String> = re.split(&line).map(|x| x.to_string()).collect();
        for part in &parts[1..] {
            // ==== Red
            let rer = Regex::new(r"(\d+)( red)").unwrap();
            let red = match rer.captures(part) {
                Some(n) => match n.get(1) {
                    Some(x) => x.as_str().parse::<i32>().unwrap(),
                    None => 0,
                },
                None => 0,
            };

            // ==== Green
            let reg = Regex::new(r"(\d+)( green)").unwrap();
            let green = match reg.captures(part) {
                Some(n) => match n.get(1) {
                    Some(x) => x.as_str().parse::<i32>().unwrap(),
                    None => 0,
                },
                None => 0,
            };

            // ==== Blue
            let reb = Regex::new(r"(\d+)( blue)").unwrap();
            let blue = match reb.captures(part) {
                Some(n) => match n.get(1) {
                    Some(x) => x.as_str().parse::<i32>().unwrap(),
                    None => 0,
                },
                None => 0,
            };

            if (blue > 14) || (red > 12) || (green > 13) {
                continue 'outer;
            }
        }
        result += game;
    }

    result
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    #[test]
    fn test_day_2a() {
        let input = indoc! {"
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "};
        let lines = input.lines().map(|line| line.to_string());
        let result = super::day_2a(lines);
        assert_eq!(result, 8);
    }
}
