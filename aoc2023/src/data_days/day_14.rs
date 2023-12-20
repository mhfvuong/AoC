pub fn day_14(data_string: String) {
    let lines = data_string.lines();
    let mut squared_rocks = Vec::new();
    let mut rounded_rocks = Vec::new();
    let mut y = 0;
    // let mut xx = 0;
    for line in lines {
        let mut x = 0;
        for rock in line.chars(){
            if rock == '#' { 
                squared_rocks.push([y, x]);
             }
            else if rock == 'O' {rounded_rocks.push([y, x]);}
            x += 1;
        }
        // xx = x;
        y += 1;
    }
    let mut rounded_rocks_new = Vec::new();
    for round in rounded_rocks {
        // let rock_x = rock[1];
        let mut new_y = 0;
        for y_rock in 0..round[0] {
            if squared_rocks.contains(&[y_rock, round[1]]){
                new_y = y_rock + 1;
            }
        }
        while rounded_rocks_new.contains(&[new_y, round[1]]){
            new_y += 1;
        }
        rounded_rocks_new.push([new_y, round[1]]);
    }

    // uncomment below to print
    // for y1 in 0..y {
    //     let mut prntline = String::new();
    //     for x1 in 0..xx {
    //         if rounded_rocks_new.contains(&[y1, x1]) {
    //             prntline.push('O');
    //         }
    //         else if squared_rocks.contains(&[y1, x1]) {
    //             prntline.push('#');
    //         }
    //         else {
    //             prntline.push('.');
    //         }
    //         if x1 == xx - 1 {
    //             println!("{}", &prntline);
    //         }
    //     }
    // }


    let mut load = 0;
    for rock in rounded_rocks_new {
        load += y - rock[0];
    }
    println!("{}", load);
}