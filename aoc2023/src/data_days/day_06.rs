pub fn day_06(data_string: String){
    let lines = data_string.lines();
    let mut times = Vec::new();
    let mut distances = Vec::new();
    let mut big_time = 0;
    let mut big_distance = 0;
    let mut tt = false;
    for line in lines {
        if !tt {
            times = string_to_vec(&line);
            big_time = string_to_vec2(line);
        }
        else {
            distances = string_to_vec(&line);
            big_distance = string_to_vec2(line);
        }
        tt = true;
    }

    // println!("{:?}", &times);
    // println!("{:?}", &distances);
    //
    // println!("{}", &big_time);
    // println!("{}", &big_distance);

    let mut winning_possibilities = Vec::new();
    for n in 0..times.len() {
        let t = times[n];
        // let mut speed = 0;
        let mut wins = 0;
        for i in 0..t+1 {
            // println!("holding the button down for {} milliseconds", &i);
            let time_left = t - i;
            // println!("the boat will go for {} milliseconds", &time_left);
            let distance = i * time_left;
            // println!("it will travel {} millimeters", &distance);

            if distance > distances[n]{
                // println!("we beat {} millimeter", &distances[n]);
                wins += 1;
            }
            // speed += 1;
        }
        // println!("we got {} wins!", &wins);
        winning_possibilities.push(wins);
    }
    let mut possibilities = 1;
    for n in winning_possibilities {
        possibilities *= n;
    }
    println!("{} possible wins", possibilities);
    let mut big_win = 0;
    for i in 0..big_time+1 {
        let time_left = big_time - i;
        let distance = i * time_left;
        if distance > big_distance {
            // println!("{}", &i);
            big_win += 1;
        }
    }
    println!("{} possible wins with big number", big_win);
}
fn string_to_vec(line: &str) -> Vec::<i32> {
    let mut number_vec = Vec::new();
    let mut number = String::new();
    let mut n = 0;
    for c in line.chars() {
        if c.is_numeric() { number.push(c); }
        if !c.is_numeric() || n == line.len() - 1 {
            if !number.is_empty() {
                number_vec.push(number.parse::<i32>().unwrap());
                number = String::new();
            }
        }
        n += 1;
    }
    return number_vec;
}
fn string_to_vec2(line: &str) -> i64 {
    let mut real_number = 0;
    let mut number = String::new();
    let mut n = 0;
    for c in line.chars() {
        if c.is_numeric() { number.push(c); }
        if n == line.len() - 1 {
            real_number = number.parse::<i64>().unwrap();
        }
        n += 1;
    }
    return real_number;
}
