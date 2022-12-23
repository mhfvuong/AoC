pub fn cathode_ray(data_string: String) {
    let mut cycle = 0;
    let mut register = 1;
    let mut signal_strength = 0;
    let mut crt_draw: [[char; 40]; 6] = [[' '; 40]; 6];
    let lines = data_string.lines();
    for line in lines {
        // println!("{:?}", line);
        if line.trim() == "noop" {
            cycle += 1;
            cycle_check(&mut signal_strength, cycle, register, &mut crt_draw);
        }
        else {
            cycle += 1;
            cycle_check(&mut signal_strength, cycle, register, &mut crt_draw);
            cycle += 1;
            cycle_check(&mut signal_strength, cycle, register, &mut crt_draw);
            let add_x = line.split(' ').collect::<Vec<&str>>();
            register += add_x[1].parse::<i32>().unwrap();
        }
    }
    println!("signal strength: {}", signal_strength);
    println!("characters:");
    let mut answer: Vec<String> = Vec::new();
    for i in 0..crt_draw.len() {
        answer.push(String::new());
        for crt_char in crt_draw[i] { answer[i].push(crt_char); }
    }
    for i in &answer { println!("{:?}", i); }
}

fn cycle_check(signal_strength: &mut i32, cycle: i32, register: i32, crt_draw: &mut [[char; 40]; 6]) {
    if (cycle - 20) % 40 == 0 {
        *signal_strength += cycle * register;
    }
    let row = (cycle - 1) / 40;
    let column = (cycle - 1) % 40;
    if (register - column).abs() <= 1 {
        crt_draw[row as usize][column as usize] = '#';
    }
}