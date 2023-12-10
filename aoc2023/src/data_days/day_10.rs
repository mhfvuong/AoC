pub fn day_10(data_string: String){
    let mut tile_maze = Vec::new();
    let mut big_maze = Vec::new();
    let lines = data_string.lines();
    let mut s = [0, 0];
    let mut y = 0;
    for line in lines {
        if line.contains('S') {
            let mut x = 0;
            for c in line.chars() {
                if c == 'S' {
                    s = [y, x];
                }
                x += 1;
            }
        }
        y += 1;
        tile_maze.push(line.chars().collect::<Vec<char>>());
        let mut empty_line = Vec::new();
        let mut larger_line = Vec::new();
        for c in line.chars() {
            empty_line.push(' ');
            larger_line.push(c);
            empty_line.push(' ');
            larger_line.push(' ');
        }
        big_maze.push(larger_line);
        big_maze.push(empty_line)
    }
    // print_pipes(&tile_maze);
    let mut path = vec![s];
    let mut big_path = vec![[s[0]*2, s[1] * 2]];
    let mut end_found = false;
    let mut current_tile = 'S';
    let mut left_right = Vec::new();
    while !end_found {
        next_step(&mut path, &tile_maze, current_tile, &mut big_path, &mut big_maze);
        if *path.last().unwrap() == s {end_found = true;}
        if current_tile == '|' && left_right.is_empty(){
            let recent_path = big_path.last().unwrap();
            left_right.push([recent_path[0], recent_path[1] - 1]);
            left_right.push([recent_path[0], recent_path[1] + 1]);
        }
        current_tile = tile_maze[path.last().unwrap()[0]][path.last().unwrap()[1]];
    }

    print_pipes(&big_maze);
    let inside = flood(big_maze, big_path, left_right);
    println!("the path loop is {} tiles long", path.len() - 1);
    println!("the longest distance away from S is {} tiles", (path.len() - 1) / 2);
    println!("the tiles inside the loop are: {}", inside);
}

fn next_step(path: &mut Vec<[usize; 2]>, tile_maze: &Vec<Vec<char>>, current_tile: char,  big_path: &mut Vec<[usize; 2]>, big_maze: &mut Vec<Vec<char>>) {
    let mut c = path.last().unwrap();
    let x = c[1];
    let y = c[0];
    let mut option_a = Vec::new();
    let mut option_b = Vec::new();
    let mut option_big_a = Vec::new();
    let mut option_big_b = Vec::new();
    if current_tile == 'S' || current_tile == '|' || current_tile == 'L' || current_tile == 'J' {
        if y > 0 {
            let tile = tile_maze[y - 1][x];
            if tile == '|' || tile == '7' || tile == 'F' || tile == 'S' {
                big_maze[y*2 - 1][x*2] = '|';
                if option_a.is_empty() {
                    option_a.push(y - 1);
                    option_a.push(x);
                    option_big_a.push(y * 2 - 1);
                    option_big_a.push(x*2);
                    option_big_a.push(y * 2 - 2);
                    option_big_a.push(x*2);
                }
                else {
                    option_b.push(y-1);
                    option_b.push(x);
                    option_big_b.push(y * 2 - 1);
                    option_big_b.push(x*2);
                    option_big_b.push(y * 2 - 2);
                    option_big_b.push(x*2);
                }
            }
        }
    }
    if current_tile == 'S' || current_tile == '|' || current_tile == 'F' || current_tile == '7' {
        if y < tile_maze.len() {
            let tile = tile_maze[y + 1][x];
            if tile == '|' || tile == 'L' || tile == 'J' || tile == 'S' {
                big_maze[y*2 + 1][x*2] = '|';
                if option_a.is_empty() {
                    option_a.push(y + 1);
                    option_a.push(x);
                    option_big_a.push(y * 2 + 1);
                    option_big_a.push(x*2);
                    option_big_a.push(y * 2 + 2);
                    option_big_a.push(x*2);
                } else {
                    option_b.push(y + 1);
                    option_b.push(x);
                    option_big_b.push(y * 2 + 1);
                    option_big_b.push(x*2);
                    option_big_b.push(y * 2 + 2);
                    option_big_b.push(x*2);
                }
            }
        }
    }
    if current_tile == 'S' || current_tile == '-' || current_tile == 'J' || current_tile == '7' {
        if x > 0 {
            let tile = tile_maze[y][x - 1];
            if tile == '-' || tile == 'L' || tile == 'F' || tile == 'S' {
                big_maze[y*2][x*2-1] = '-';
                if option_a.is_empty() {
                    option_a.push(y);
                    option_a.push(x-1);
                    option_big_a.push(y*2);
                    option_big_a.push(x*2-1);
                    option_big_a.push(y*2);
                    option_big_a.push(x*2-2);
                }
                else {
                    option_b.push(y);
                    option_b.push(x-1);
                    option_big_b.push(y*2);
                    option_big_b.push(x*2-1);
                    option_big_b.push(y*2);
                    option_big_b.push(x*2-2);
                }
            }
        }
    }
    if current_tile == 'S' || current_tile == '-' || current_tile == 'F' || current_tile == 'L' {
        if x < tile_maze[0].len() {
            let tile = tile_maze[y][x + 1];
            if tile == '-' || tile == '7' || tile == 'J' || tile == 'S' {
                big_maze[y*2][x*2+1] = '-';
                if option_a.is_empty() {
                    option_a.push(y);
                    option_a.push(x+1);
                    option_big_a.push(y*2);
                    option_big_a.push(x*2+1);
                    option_big_a.push(y*2);
                    option_big_a.push(x*2+2);
                }
                else {
                    option_b.push(y);
                    option_b.push(x+1);
                    option_big_b.push(y*2);
                    option_big_b.push(x*2+1);
                    option_big_b.push(y*2);
                    option_big_b.push(x*2+2);
                }
            }
        }
    }
    let a = [option_a[0], option_a[1]];
    let big_a1 = [option_big_a[0], option_big_a[1]];
    let big_a2 = [option_big_a[2], option_big_a[3]];
    let b = [option_b[0], option_b[1]];
    let big_b1 = [option_big_b[0], option_big_b[1]];
    let big_b2 = [option_big_b[2], option_big_b[3]];
    if !path.contains(&a) {path.push(a);
        big_path.push(big_a1); big_path.push(big_a2);
    }
    else if !path.contains(&b) {path.push(b);
        big_path.push(big_b1); big_path.push(big_b2);}
    else {
        if option_a == path[0] {path.push(a);
            big_path.push(big_a1); big_path.push(big_a2);}
        else { path.push(b);
            big_path.push(big_b1); big_path.push(big_b2);}
    }
}
fn print_pipes(tile_maze: &Vec<Vec<char>>){
    for line in tile_maze {
        let mut string = String::new();
        for tile in line {
            string.push(*tile);
        }
        println!("{string}");
    }
}

fn flood(big_maze: Vec<Vec<char>>, big_path: Vec<[usize; 2]>, left_right: Vec<[usize; 2]>) -> i32 {
    let maze_bounds = [big_maze.len(), big_maze[0].len()];
    let mut insides = 99999;
    for s in left_right  {
        let mut visited = Vec::new();
        let inside = find_inside(&maze_bounds, &big_path, s, &mut visited);
        if inside < insides && inside != 0 {insides = inside;}
    }
    return insides;
}

fn find_inside(mb: &[usize; 2], big_path: &Vec<[usize; 2]>, yx: [usize; 2], visited: &mut Vec<[usize; 2]>) -> i32 {
    if !big_path.contains(&yx) {
        if !visited.contains(&yx) { visited.push(yx);}
        else { return 0;}
        let mut inside = 0;
        if yx[0] % 2 == 0 && yx[1] % 2 == 0 { // it is a coordinate on the original map
            inside += 1;
        }
        if yx[0] > 0 { inside += find_inside(mb, &big_path, [yx[0] - 1, yx[1]], visited);}
        if yx[0] < mb[0] { inside += find_inside(mb, &big_path, [yx[0] + 1, yx[1]], visited);}
        if yx[1] > 0 { inside += find_inside(mb, &big_path, [yx[0], yx[1] - 1], visited);}
        if yx[1] < mb[1]{ inside += find_inside(mb, &big_path, [yx[0], yx[1] + 1], visited);}
        return inside;
    }
    return 0;
}
