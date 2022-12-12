use std::fs::File;
use std::path::PathBuf;
use std::io;
use std::io::Read;
use crate::days::day10::cathode_ray;
use crate::days::day11::monkey_trouble;
use crate::days::day1::elf_calories;
use crate::days::day2::rock_paper_scissor;
use crate::days::day3::rucksack;
use crate::days::day4::cleanup_pairs;
use crate::days::day5::crate_crane;
use crate::days::day6::tune_device;
use crate::days::day7::file_size;
use crate::days::day8::visible_trees;
use crate::days::day9::rope_bridge;

mod days;


fn main() {
    let mut user_input = String::new();
    let stdin = io::stdin();
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src");
    path.push("puzzle_data");
    println!("enter 'tst' for tst data, or only press enter for real data");
    stdin.read_line(&mut user_input).expect(
        "Reading user input went wrong"
    );
    if user_input.trim() == "tst".to_string() {
        path.push("test");
    }
    else {
        path.push("real");
    }

    println!("choose a day (1-current)");
    user_input = "".to_string();
    stdin.read_line(&mut user_input).expect(
        "Reading user input went wrong"
    );

    path.push("day".to_owned() + user_input.trim() + ".txt");
    let mut data_file = File::open(&path).unwrap();
    let mut data_string = String::new();
    data_file.read_to_string(&mut data_string).expect(
        "Something went wrong with reading the data"
    );

    println!("Result:");
    match user_input.trim() {
        "1" => elf_calories(data_string), // Day 1
        "2" => rock_paper_scissor(data_string), // Day 2
        "3" => rucksack(data_string), // Day 3
        "4" => cleanup_pairs(data_string), // Day 4
        "5" => crate_crane(data_string), // Day 5
        "6" => tune_device(data_string), // Day 6
        "7" => file_size(data_string), // Day 7
        "8" => visible_trees(data_string), // Day 8
        "9" => rope_bridge(data_string), // Day 9
        "10" => cathode_ray(data_string), // Day 10
        "11" => monkey_trouble(data_string), // Day 11
        _ => println!("No valid day has been selected, program quits")
    }
}
