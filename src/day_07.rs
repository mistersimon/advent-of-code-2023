use std::collections::HashMap;

// #[derive(Debug)]
struct Hand {
    bid: u64,
    cards: [u64; 5],
    type_: u64,
}

fn convert_card(c: char) -> u64 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => c.to_digit(10).unwrap(),
    }
    .try_into()
    .unwrap()
}

fn get_type(hand: &str) -> u64 {
    let mut s: HashMap<char, usize> = HashMap::new();
    for c in hand.chars() {
        if let Some(count) = s.get(&c) {
            s.insert(c, count + 1);
        } else {
            s.insert(c, 0);
        }
    }

    let mut counts = s.into_values().collect::<Vec<usize>>();

    counts.sort_by(|a, b| b.cmp(a));

    match counts[..] {
        [5] => 6,
        [4, 1] => 5,
        [3, 2] => 4,
        [3, 1, 1] => 3,
        [2, 2, 1] => 2,
        [2, 1, 1, 1] => 1,
        _ => 0,
    }
}

fn get_type_with_joker(hand: &str) -> u64 {
    let mut s: HashMap<char, u64> = HashMap::new();
    let mut jokers: u64 = 0;
    for c in hand.chars() {
        if c == 'J' {
            jokers += 1;
            continue;
        }

        if let Some(count) = s.get(&c) {
            s.insert(c, count + 1);
        } else {
            s.insert(c, 0);
        }
    }

    let mut counts = s.into_values().collect::<Vec<u64>>();

    counts.sort_by(|a, b| b.cmp(a));

    if jokers == 5 {
        counts = vec![5]
    } else {
        counts[0] += jokers;
    }

    match counts[..] {
        [5] => 6,
        [4, 1] => 5,
        [3, 2] => 4,
        [3, 1, 1] => 3,
        [2, 2, 1] => 2,
        [2, 1, 1, 1] => 1,
        _ => 0,
    }
}

pub fn day_7a(lines: impl Iterator<Item = String>) -> u64 {
    let mut hands: Vec<Hand> = lines
        .map(|f| {
            let hand = f.split(' ').collect::<Vec<&str>>();
            if hand.len() != 2 {
                panic!()
            }

            let display = hand[0].to_string();
            Hand {
                bid: hand[1].to_string().parse::<u64>().unwrap(),
                cards: display
                    .chars()
                    .map(convert_card)
                    .collect::<Vec<u64>>()
                    .try_into()
                    .unwrap(),
                type_: get_type(&display),
            }
        })
        .collect();

    hands.sort_by(|a, b| {
        if a.type_ != b.type_ {
            return a.type_.cmp(&b.type_);
        };

        for i in 0..5 {
            if a.cards[i] == b.cards[i] {
                continue;
            }

            return a.cards[i].cmp(&b.cards[i]);
        }

        panic!();
    });

    let mut result: u64 = 0;

    for (i, hand) in hands.iter().enumerate() {
        result += (u64::try_from(i).unwrap() + 1) * hand.bid;
    }

    result
}

pub fn day_7b(lines: impl Iterator<Item = String>) -> u64 {
    let mut hands: Vec<Hand> = lines
        .map(|f| {
            let hand = f.split(' ').collect::<Vec<&str>>();
            if hand.len() != 2 {
                panic!()
            }

            let display = hand[0].to_string();
            let cards: [u64; 5] = display
                .chars()
                .map(convert_card)
                .map(|a| match a {
                    11 => 1,
                    _ => a,
                })
                .collect::<Vec<u64>>()
                .try_into()
                .unwrap();
            Hand {
                bid: hand[1].to_string().parse::<u64>().unwrap(),
                cards,
                type_: get_type_with_joker(&display),
            }
        })
        .collect();

    hands.sort_by(|a, b| {
        if a.type_ != b.type_ {
            return a.type_.cmp(&b.type_);
        };

        for i in 0..5 {
            if a.cards[i] == b.cards[i] {
                continue;
            }

            return a.cards[i].cmp(&b.cards[i]);
        }

        panic!();
    });

    let mut result: u64 = 0;

    for (i, hand) in hands.iter().enumerate() {
        result += (u64::try_from(i).unwrap() + 1) * hand.bid;
    }

    result
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use rstest::rstest;

    const EXAMPLE: &str = indoc! {"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
    "};

    #[rstest]
    #[case('A', 14)]
    #[case('K', 13)]
    #[case('Q', 12)]
    #[case('J', 11)]
    #[case('T', 10)]
    #[case('9', 9)]
    fn convert_card_test(#[case] card: char, #[case] expected: u64) {
        let output = super::convert_card(card);
        assert_eq!(output, expected);
    }

    #[rstest]
    #[case("AAAAA", 6)]
    #[case("AA8AA", 5)]
    #[case("23332", 4)]
    #[case("TTT98", 3)]
    #[case("23432", 2)]
    #[case("A23A4", 1)]
    #[case("23456", 0)]
    fn get_type_test(#[case] hand: String, #[case] expected: u64) {
        let output = super::get_type(&hand);
        assert_eq!(output, expected);
    }

    #[rstest]
    #[case("T55J5", 5)]
    #[case("KTJJT", 5)]
    #[case("QQQJA", 5)]
    #[case("23456", 0)]
    fn get_type_with_joker_test(#[case] hand: String, #[case] expected: u64) {
        let output = super::get_type_with_joker(&hand);
        assert_eq!(output, expected);
    }

    #[test]
    fn test_day_7a() {
        let lines = EXAMPLE.lines().map(|line| line.to_string());
        let result = super::day_7a(lines);
        assert_eq!(result, 6440);
    }

    // #[test]
    // fn test_day_7b() {
    //     let lines = EXAMPLE.lines().map(|line| line.to_string());
    //     let result = super::day_7b(lines);
    //     assert_eq!(result, 71503);
    // }
}
