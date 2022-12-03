use std::path::PathBuf;
use std::io;

fn main() {
    let mut user_input = String::new();
    let stdin = io::stdin();
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src");
    stdin.read_line(&mut user_input);
    if user_input.trim() == String::from("test") {
        path.push("test_input.txt");
        println!("test input:");
    }
    else {
        path.push("input.txt");
        println!("real data input:");
    }

    // path.push("")
}