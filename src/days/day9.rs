pub fn rope_bridge(data_string: String) {
    let mut positions_visited: Vec<[i32; 2]> = Vec::from([[0; 2]]);
    let mut pos_t: [[i32; 2]; 9] = [[0; 2]; 9];
    let mut pos_h: [i32; 2] = [0; 2];
    let mut current_knot: [i32; 2];
    let options: [i32; 3] = [-1, 0, 1];
    let lines = data_string.lines();
    for line in lines {
        let movement = line.split(' ').collect::<Vec<&str>>();
        let direction = movement[0];
        let steps = movement[1].parse::<usize>().unwrap();
        // println!("new instruction");
        for _step in 0..steps {
            match direction {
                "U" => pos_h[1] += 1,
                "D" => pos_h[1] -= 1,
                "L" => pos_h[0] -= 1,
                "R" => pos_h[0] += 1,
                _ => println!("No valid step"),
            }
            for i in 0..pos_t.len() {
                if i > 0 {
                    current_knot = pos_t[i - 1];
                }
                else {current_knot = pos_h;}
                if distance(&current_knot, &pos_t[i]) > 2_f64.sqrt() {
                    let tx = pos_t[i][0];
                    let ty = pos_t[i][1];
                    let mut next_connection_found = false;
                    for x in options {
                        for y in options {
                            if !next_connection_found && distance(&current_knot, &[tx + x, ty + y]) == 1.0 {
                                pos_t[i] = [tx + x, ty + y];
                                next_connection_found = true;
                            }
                            if !next_connection_found && distance(&current_knot, &[tx + x, ty + y]) == 2_f64.sqrt() {
                                   pos_t[i] = [tx + x, ty + y];
                            }
                        }
                    }
                }
            }
            if !positions_visited.contains(&pos_t[8]) {
                positions_visited.push(pos_t[8]);
            }
        }
    }
    println!("spots visited at least once: {}", positions_visited.len());
}

fn distance(pos_h: &[i32; 2], pos_t: &[i32; 2]) -> f64 {
    let dx = pos_h[0] - pos_t[0];
    let dy = pos_h[1] - pos_t[1];
    let sqr_dis = dx.pow(2) + dy.pow(2);
    (sqr_dis as f64).sqrt()
}