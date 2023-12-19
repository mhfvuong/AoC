use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::io;
mod data_days;

fn main() {
    // let mut user_input = String::new();
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // path.push("src/data_days/data_13.txt");

    let stdin = io::stdin();
    let mut user_input = String::new();
    println!("type 'tst' or a number 1-25");
    stdin.read_line(&mut user_input).expect("");
    if user_input.trim() == "tst" {
        path.push("src/data_days/tst.txt");
    }
    else if user_input.trim().len() == 1 {
        path.push("src/data_days/data_0".to_owned() + user_input.trim() + ".txt");
    } 
    else {
        path.push("src/data_days/data_".to_owned() + user_input.trim() + ".txt");
    }
    let mut data_file = File::open(&path).unwrap();
    let mut data_string = String::new();
    data_file.read_to_string(&mut data_string).expect(
        "something went wrong"
    );
    // data_days::day_13::day_13(data_string);
    match user_input.trim() {
        "1" => {data_days::day_01::day_01a(&data_string);
            data_days::day_01::day_01b(data_string);
        },
        "2" => data_days::day_02::day_02ab(data_string),
        "3" => data_days::day_03::day_03ab(data_string),
        "4" => data_days::day_04::day_04a(data_string),
        "5" => data_days::day_05::day_05(data_string),
        "6" => data_days::day_06::day_06(data_string),
        "7" => data_days::day_07::day_07(data_string),
        "8" => data_days::day_08::day_08(data_string),
        "9" => data_days::day_09::day_09(data_string),
        "10" => data_days::day_10::day_10(data_string),
        "11" => data_days::day_11::day_11(data_string),
        "12" => data_days::day_12::day_12(data_string),
        "13" => data_days::day_13::day_13(data_string),
        "tst" => data_days::day_13::day_13(data_string),
        _ => print!("no day or test selected"),
    }
}
