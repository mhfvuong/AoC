use std::collections::HashMap;

fn check_adjacent(n: usize, m: usize, schematic: &Vec<Vec<char>>) -> bool {
    // check corners
    if n > 0 && m > 0 {
        if !schematic[n - 1][m - 1].is_numeric() && schematic[n - 1][m - 1] != '.' {
            // println!("found adjacent: {}", schematic[n-1][m-1]);
            return true;
        }
    }
    if n > 0 && m < (schematic[0].len()-1) {
        if !schematic[n - 1][m + 1].is_numeric() && schematic[n - 1][m + 1] != '.' {
            // println!("found adjacent: {}", schematic[n-1][m+1]);
            return true;
        }
    }
    if m > 0 && n < (schematic[0].len()-1) {
        if !schematic[n + 1][m - 1].is_numeric() && schematic[n + 1][m - 1] != '.' {
            // println!("found adjacent: {}", schematic[n+1][m-1]);
            return true;
        }
    }
    if n < (schematic[0].len()-1) && m < (schematic[0].len()-1) {
        if !schematic[n + 1][m + 1].is_numeric() && schematic[n + 1][m + 1] != '.' {
            // println!("found adjacent: {}", schematic[n+1][m+1]);
            return true;
        }
    }
    //check edges
    if m > 0 {
        if !schematic[n][m-1].is_numeric() && schematic[n][m-1] != '.'{
            // println!("found adjacent: {}", schematic[n][m-1]);
            return true;
        }
    }
    if m < (schematic[0].len()-1) {
        if !schematic[n][m+1].is_numeric() && schematic[n][m+1] != '.'{
            // println!("found adjacent: {}", schematic[n][m+1]);
            return true;
        }
    }
    if n < (schematic[0].len()-1) {
        if !schematic[n+1][m].is_numeric() && schematic[n+1][m] != '.'{
            // println!("found adjacent: {}", schematic[n+1][m]);
            return true;
        }
    }
    if n > 0 {
        if !schematic[n-1][m].is_numeric() && schematic[n-1][m] != '.'{
            // println!("found adjacent: {}", schematic[n-1][m]);
            return true;
        }
    }
    return false;
}

fn find_gear(n: usize, m: usize, schematic: &Vec<Vec<char>>) -> (bool,usize,usize) {
    // check corners
    if n > 0 && m > 0 {
        if !schematic[n - 1][m - 1].is_numeric() && schematic[n - 1][m - 1] == '*' {
            // println!("found adjacent: {}", schematic[n-1][m-1]);
            return (true, n - 1, m - 1);
        }
    }
    if n > 0 && m < (schematic[0].len()-1) {
        if !schematic[n - 1][m + 1].is_numeric() && schematic[n - 1][m + 1] == '*' {
            // println!("found adjacent: {}", schematic[n-1][m+1]);
            return (true, n - 1, m + 1);
        }
    }
    if m > 0 && n < (schematic[0].len()-1) {
        if !schematic[n + 1][m - 1].is_numeric() && schematic[n + 1][m - 1] == '*' {
            // println!("found adjacent: {}", schematic[n+1][m-1]);
            return (true, n + 1, m - 1);
        }
    }
    if n < (schematic[0].len()-1) && m < (schematic[0].len()-1) {
        if !schematic[n + 1][m + 1].is_numeric() && schematic[n + 1][m + 1] == '*' {
            // println!("found adjacent: {}", schematic[n+1][m+1]);
            return (true, n + 1, m + 1);
        }
    }
    //check edges
    if m > 0 {
        if !schematic[n][m-1].is_numeric() && schematic[n][m-1] == '*'{
            // println!("found adjacent: {}", schematic[n][m-1]);
            return (true, n, m - 1);
        }
    }
    if m < (schematic[0].len()-1) {
        if !schematic[n][m+1].is_numeric() && schematic[n][m+1] == '*'{
            // println!("found adjacent: {}", schematic[n][m+1]);
            return (true, n, m + 1);
        }
    }
    if n < (schematic[0].len()-1) {
        if !schematic[n+1][m].is_numeric() && schematic[n+1][m] == '*'{
            // println!("found adjacent: {}", schematic[n+1][m]);
            return (true, n + 1, m);
        }
    }
    if n > 0 {
        if !schematic[n-1][m].is_numeric() && schematic[n-1][m] == '*'{
            // println!("found adjacent: {}", schematic[n-1][m]);
            return (true, n -1, m);
        }
    }
    return (false, 0, 0);
}

pub fn day_03ab(data_string: String){
    let lines = data_string.lines();

    let mut gear_ratios = HashMap::new();
    let mut gear_m = 0;
    let mut gear_n = 0;
    let mut gear_sum = 0;
    let mut schematic = Vec::new();
    let mut i = 0;
    for line in lines{
        schematic.push(Vec::new());
        for letter in line.chars() {
            schematic[i].push(letter);
        }
        i += 1;
    }
    let mut tot_number = 0;
    for n in 0..schematic.len() {
        let mut number = String::new();
        let mut adjacent = false;
        let mut gear_found = false;
        for m in 0..schematic[0].len() {
            if schematic[n][m].is_numeric() {
                number.push(schematic[n][m]);
                if !adjacent{
                    adjacent = check_adjacent(n, m, &schematic);
                }
                if !gear_found{
                    (gear_found, gear_n, gear_m) = find_gear(n, m, &schematic);
                }
            } else {
                if number.len() > 0 {
                    if adjacent {
                        tot_number += number.parse::<i32>().unwrap();
                        println!("{} is adjacent to a symbol, sum is {}", &number, &tot_number);
                        adjacent = false;
                    } else { println!("{} is not adjacent to a symbol", &number); }
                    if gear_found {
                        let key = gear_n.to_string() + &gear_m.to_string();
                        let gear_product = number.parse::<i32>().unwrap();
                        match gear_ratios.get(&key){
                            None => { gear_ratios.insert(key.clone(), vec![gear_product]); println!("found gear at {}", key)},
                            Some(gear) => {
                                let mut new_v = vec![gear_product];
                                for g in gear {new_v.push(*g);}
                                println!("found connection! {:?}", &new_v);
                                gear_ratios.insert(key, new_v);
                            },
                        };
                        gear_found = false;
                    }
                    number = String::new();
                }
            }
        }
        if number.len() > 0 {
            if adjacent {
                tot_number += number.parse::<i32>().unwrap();
                println!("{} is adjacent to a symbol, sum is {}", &number, &tot_number);
                // adjacent = false;
            } else { println!("{} is not adjacent to a symbol", &number); }
        }
        if gear_found {
            let key = gear_n.to_string() + &gear_m.to_string();
            let gear_product = number.parse::<i32>().unwrap();
            match gear_ratios.get(&key){
                None => { gear_ratios.insert(key.clone(), vec![gear_product]); println!("found gear at {}", key)},
                Some(gear) => {
                    let mut new_v = vec![gear_product];
                    for g in gear {new_v.push(*g);}
                    println!("found connection! {:?}", &new_v);
                    gear_ratios.insert(key, new_v);
                },
            };
            // gear_found = false;
        }
    }
    for gears in gear_ratios.keys(){
        match gear_ratios.get(gears){
            None => (),
            Some(gear_vector) => {
                println!("gears connected are {:?}", &gear_vector);
                if gear_vector.len() == 2 {
                    gear_sum += gear_vector[0] * gear_vector[1];
                }
            }
        }
    }
    println!("schematic sum {}", tot_number);
    println!("gear product {}", gear_sum);
}