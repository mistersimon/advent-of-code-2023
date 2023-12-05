mod day_1;
mod day_2;
mod day_3;
mod day_4;

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

    let result = match format!("{}{}", day, part).as_str() {
        "1a" => day_1::day_1a(input),
        "1b" => day_1::day_1b(input),
        "2a" => day_2::day_2a(input),
        "2b" => day_2::day_2b(input),
        "3a" => day_3::day_3a(input),
        "3b" => day_3::day_3b(input),
        "4a" => day_4::day_4a(input),
        "4b" => day_4::day_4b(input),
        _ => {
            println!("Day {} {} not implemented", day, part);
            0
        }
    };

    println!("{}", result)
}
