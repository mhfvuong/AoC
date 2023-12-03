use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
mod data_days;

fn main() {
    // let mut user_input = String::new();
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/data_days/data_03.txt");

    let mut data_file = File::open(&path).unwrap();
    let mut data_string = String::new();
    data_file.read_to_string(&mut data_string).expect(
        "something went wrong"
    );
    data_days::day_03::day_03ab(data_string);
}
