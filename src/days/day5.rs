use std::collections::HashMap;

pub fn crate_crane(data_string: String) {
    let lines = data_string.lines();
    let mut instruction:bool = false;
    let crates: HashMap<usize, Vec<&str>> = HashMap::new();
    for line in lines {
        if line.len() > 1 && !instruction{
            todo!() // read data of the crates
        }
        else if line.len() > 1 && instruction{
            todo!() // read instructions
        }
        if line == "" {instruction = true;}
    }
}