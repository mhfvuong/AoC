use std::collections::HashMap;

pub fn crate_crane(data_string: String) {
    let lines = data_string.lines();
    let mut instruction:bool = false;
    let mut crates: HashMap<usize, Vec<char>> = HashMap::new();
    for line in lines {
        if line.len() > 1 && !instruction{ // placing crates on stacks
            let mut i = 1;
            let mut n = 0;
            for c in line.chars() {
                match c {
                    'A'..='Z' => {
                        if let Some(stack) = crates.get_mut(&i) {
                            stack.insert(0,c);
                        }
                        else {
                            crates.insert(i, vec![c]);
                        }
                    },
                    _ => (),
                }
                n += 1;
                if n % 4 == 0 { i += 1; }
            }
        }
        else if line.len() > 1 && instruction{ // replacing crates
            let s = line.split(" from ").collect::<Vec<&str>>();
            let amount = s[0].split(" ").collect::<Vec<&str>>();
            let origin_destination = s[1].split(" to ").collect::<Vec<&str>>();
            let mut buff = Vec::new();
            for i in 0..amount[1].parse::<usize>().unwrap() { // part 1
                if let Some(origin) = crates.get_mut(&origin_destination[0].parse::<usize>().unwrap()) {
                    match origin.pop() {
                        Some(t) => buff.push(t),
                        None => (),
                    }
                }
            }
            for i in 0..amount[1].parse::<usize>().unwrap() {
                if let Some(destination) = crates.get_mut(&origin_destination[1].parse::<usize>().unwrap()) {
                    match buff.pop() {
                        Some(t) => destination.push(t),
                        None => (),
                    }
                }
            }
        }
        if line == "" {instruction = true;}
    }
    let mut answer = String::new();
    for i in 1..=crates.len() {
        if let Some(stack) = crates.get(&i) {
            answer.push(stack[stack.len() - 1]);
        }
    }
    println!("top of all the stacks: {answer}");
}