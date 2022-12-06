use std::fs::File;
use std::path::PathBuf;
use std::io;
use std::io::Read;
use crate::days::day1::elf_calories;
use crate::days::day2::rock_paper_scissor;
use crate::days::day3::rucksack;
use crate::days::day4::cleanup_pairs;
use crate::days::day5::crate_crane;
use crate::days::day6::tune_device;

mod days;


fn main() {
    let mut user_input = String::new();
    let stdin = io::stdin();
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src");
    path.push("puzzle_data");
    stdin.read_line(&mut user_input);
    if user_input.trim() == String::from("tst") {
        path.push("test_input.txt");
        println!("test input:");
    }
    else {
        path.push("input.txt");
        println!("real data input:");
    }
    let mut data_file = File::open(&path).unwrap();
    let mut data_string = String::new();
    data_file.read_to_string(&mut data_string);
    // elf_calories(data_string); // Day 1
    // rock_paper_scissor(data_string); // Day 2
    // rucksack(data_string); // Day 3
    // cleanup_pairs(data_string); // Day 4
    // crate_crane(data_string); // Day 5
    tune_device(data_string); // Day 6
}