use std::collections::HashMap;
pub fn day_08(data_string: String){
    let lines = data_string.lines();
    let mut directions = String::new();
    let mut source_lr = HashMap::new();
    let mut positions = Vec::new();
    for line in lines {
        if directions.is_empty() {directions += line; continue;}
        let dest_lr = line.split(" = ").collect::<Vec<&str>>();
        let mut l = String::new();
        let mut r = String::new();
        if !dest_lr[0].is_empty() {
            let lr = dest_lr[1].split(", ").collect::<Vec<&str>>();
            for dir in lr {
                for letter in dir.chars() {
                    if letter.is_alphabetic() || letter.is_numeric() {
                        if l.len() != 3 { l.push(letter); }
                        else { r.push(letter); }
                    }
                }
            }
            source_lr.insert(dest_lr[0], vec![l, r]);
            if dest_lr[0].chars().last().unwrap() == 'A' {
                positions.push(dest_lr[0]);
            }
        }
    }
    let mut steps = 0;
    let mut location = "AAA";
    while location != "ZZZ" {
        // println!("current location: {}", &location);
        let step = directions.chars().nth(steps % directions.len()).unwrap();
        match source_lr.get(location) {
            None => println!("not possible"),
            Some(lr) => {
                if step == 'L' { location = &lr[0]; } else { location = &lr[1]; }
            }
        }
        steps += 1;
    }
    println!("part 1 - total steps: {}", steps);

    let mut steps = 0;
    let mut step_per_p = vec![0,0,0,0,0,0];
    let mut step_since_z = vec![0,0,0,0,0,0];
    let mut steps_taken = vec![0,0,0,0,0,0];
    let mut changed = 0;
    while changed < 6{
        steps += 1;
        for p in 0..positions.len() {
            let step = directions.chars().nth(steps % directions.len()).unwrap();
            match source_lr.get(positions[p]) {
                None => println!("not possible"),
                Some(lr) => {
                    if step == 'L' { positions[p] = &lr[0]; } else { positions[p] = &lr[1]; }
                }
            }
            if positions[p].chars().last().unwrap() == 'Z' {
                let d = steps - step_per_p[p];
                step_since_z[p] = d;
                step_per_p[p] = steps;
                if step_since_z[p] == steps_taken[p] {
                    changed += 1;
                }
                else {
                    steps_taken[p] = step_since_z[p];
                    changed = 0;
                }
            }
        }
    }
    let mut lcm = find_lcm(steps_taken[0] as u128, steps_taken[1] as u128);
    for i in 2..steps_taken.len(){
        lcm = find_lcm(lcm, steps_taken[i] as u128);
    }
    println!("part 2 - total steps: {}", lcm);
}

fn find_lcm (a: u128, b: u128) -> u128 {
    let mut numerator = a;
    let mut denominator = b;
    if a < b {numerator = b; denominator = a;}
    let mut remainder = numerator % denominator;
    while remainder != 0 {
        numerator = denominator;
        denominator = remainder;
        remainder = numerator % denominator;
    }
    let gcd = denominator;
    let lcm = (a * b)/gcd;
    return lcm;
}