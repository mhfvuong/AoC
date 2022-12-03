use std::fs::File;
use std::path::PathBuf;
use std::io;
use std::io::Read;
use crate::days::day1::elf_calories;
use crate::days::day2::rock_paper_scissor;

mod days;


fn main() {
    let mut user_input = String::new();
    let stdin = io::stdin();
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src");
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
    rock_paper_scissor(data_string);
}