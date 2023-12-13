mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;

use std::env::args;
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn get_input(day: u8) -> impl Iterator<Item = String> {
    let root = env::var("CARGO_MANIFEST_DIR").unwrap();
    let dir = format!("{}/inputs/{}.txt", root, day);
    let file = File::open(dir).expect("File not found");

    BufReader::new(file).lines().flatten()
}

fn main() {
    let day: u8 = args().nth(1).expect("no day given").parse().unwrap();
    let part: char = args().nth(2).unwrap_or("a".to_string()).parse().unwrap();

    println!("Running {} {}", day, part);

    let input = get_input(day);

    let result: u64 = match format!("{}{}", day, part).as_str() {
        "1a" => day_01::day_1a(input).try_into().unwrap(),
        "1b" => day_01::day_1b(input).try_into().unwrap(),
        "2a" => day_02::day_2a(input).try_into().unwrap(),
        "2b" => day_02::day_2b(input).try_into().unwrap(),
        "3a" => day_03::day_3a(input).try_into().unwrap(),
        "3b" => day_03::day_3b(input).try_into().unwrap(),
        "4a" => day_04::day_4a(input).try_into().unwrap(),
        "4b" => day_04::day_4b(input).try_into().unwrap(),
        "5a" => day_05::day_5a(input),
        "5b" => day_05::day_5b(input),
        "6a" => day_06::day_6a(input),
        "6b" => day_06::day_6b(input),
        "7a" => day_07::day_7a(input),
        "7b" => day_07::day_7b(input),
        "8a" => day_08::part_one(input),
        "8b" => day_08::part_two(input),
        "9a" => day_09::part_one(input),
        "9b" => day_09::part_two(input),
        "10a" => day_10::part_one(input),
        "10b" => day_10::part_two(input),
        "11a" => day_11::part_one(input),
        "11b" => day_11::part_two(input),
        "12a" => day_12::part_one(input),
        "12b" => day_12::part_two(input),
        _ => {
            println!("Day {} {} not implemented", day, part);
            0
        }
    };

    println!("{}", result)
}
