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
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;

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
        "13a" => day_13::part_one(input),
        "13b" => day_13::part_two(input),
        "14a" => day_14::part_one(input),
        "14b" => day_14::part_two(input),
        "15a" => day_15::part_one(input),
        "15b" => day_15::part_two(input),
        "16a" => day_16::part_one(input),
        "16b" => day_16::part_two(input),
        "17a" => day_17::part_one(input),
        "17b" => day_17::part_two(input),
        // "18a" => day_18::part_one(input),
        // "18b" => day_18::part_two(input),
        // "19a" => day_19::part_one(input),
        // "19b" => day_19::part_two(input),
        // "20a" => day_20::part_one(input),
        // "20b" => day_20::part_two(input),
        // "21a" => day_21::part_one(input),
        // "21b" => day_21::part_two(input),
        // "22a" => day_22::part_one(input),
        // "22b" => day_22::part_two(input),
        // "23a" => day_23::part_one(input),
        // "23b" => day_23::part_two(input),
        // "24a" => day_24::part_one(input),
        // "24b" => day_24::part_two(input),
        // "25a" => day_25::part_one(input),
        // "25b" => day_25::part_two(input),
        _ => {
            println!("Day {} {} not implemented", day, part);
            0
        }
    };

    println!("{}", result)
}
