pub fn cathode_ray(data_string: String) {
    let mut cycle = 0;
    let mut register = 1;
    let mut signal_strength = 0;
    let lines = data_string.lines();
    for line in lines {
        // println!("{:?}", line);
        if line.trim() == "noop" {
            cycle += 1;
            if (cycle - 20) % 40 == 0 { signal_strength += cycle * register; }
        }
        else {
            cycle += 1;
            if (cycle - 20) % 40 == 0 { signal_strength += cycle * register; }
            cycle += 1;

            if (cycle - 20) % 40 == 0 { signal_strength += cycle * register; }
            let add_x = line.split(' ').collect::<Vec<&str>>();
            register += add_x[1].parse::<i32>().unwrap();
        }
    }
    println!("signal strength: {}", signal_strength);
}