use std::collections::HashMap;

pub fn hill_climbing(data_string: String) {
    let lines = data_string.lines();
    let mut start: [usize; 2] = [0; 2];
    let mut end: [usize; 2] = [0; 2];
    let mut map = vec![Vec::new()];
    let mut path = vec![Vec::new()]; // debug path
    let mut explored = Vec::new();
    let mut queue = Vec::new();
    let mut connections: HashMap<[usize; 2], Vec<[usize; 2]>> = HashMap::new();
    for (i, line) in lines.enumerate(){
        if i == map.len() {map.push(Vec::new())}
        if i == path.len() {path.push(Vec::new())} // debug path
        for h in line.chars() {
            connections.insert([i, map[i].len()], Vec::new());
            if h == 'S' {start = [i, map[i].len()];}
            else if h == 'E' { end = [i, map[i].len()];}
            map[i].push(h);
            path[i].push('.'); // debug path
        }
    }
    // char.to_string().as_bytes()[0]
    // check_map(&map,&path);
    // println!("{:?}", &end);
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let mut current = map[i][j];
            match map[i][j] {
                'S' => current = 'a',
                'E' => current = 'z',
                _ => current = map[i][j]
            }
            compare_height(&mut connections, &map, current, i, j);
        }
    }
    // for c in connections.keys(){
    //     println!("{:?} is connected to {:?}", &c, connections.get(c));
    // }
    queue.push(start);
    while !queue.is_empty() {
        // println!("queue: {:?}", &queue);
        let current = queue.remove(0);
        // add neighbors in queue
            if current == end {
                // explored.push(end);
                queue.drain(..);
            } else {
                if let Some(neighbors) = connections.get(&current) {
                    for n in neighbors {
                        if !queue.contains(n) && !explored.contains(n) {
                            // n[2] = current[2] + 1;
                            queue.push(*n);
                        }
                    }
                }

                explored.push(current);
            }
        // println!("Explored: {:?}", &explored);
    }
    println!("explored tiles until E has been found:");
    println!("{:?}", explored);
    println!("{}", explored.len());
}

fn check_map(map: &Vec<Vec<char>>, path_map: &Vec<Vec<char>>) {
    println!("\n    map: ");
    for i in map {
        let mut map_line = String::new();
        for j in i {
            map_line.push(*j);
        }
        println!("{:?}", map_line);
    }
    println!("\n    Path map: ");
    for i in path_map {
        let mut pathmap_line = String::new();
        for j in i {
            pathmap_line.push(*j);
        }
        println!("{:?}", pathmap_line);
    }

}

fn compare_height(connections: &mut HashMap<[usize; 2], Vec<[usize; 2]>>, map: &Vec<Vec<char>>, current: char, i: usize, j: usize) {
    if i > 0 {
        if current.to_string().as_bytes()[0] + 1 >= map[i - 1][j].to_string().as_bytes()[0] {
            if let Some(new_connection) = connections.get_mut(&[i, j]) {
                new_connection.push([i - 1, j]);
            }
        }
    }
    if i < map.len() - 1 {
        if current.to_string().as_bytes()[0] + 1 >= map[i + 1][j].to_string().as_bytes()[0] {
            if let Some(new_connection) = connections.get_mut(&[i, j]) {
                new_connection.push([i + 1, j]);
            }
        }
    }
    if j > 0 {
        if current.to_string().as_bytes()[0] + 1 >= map[i][j - 1].to_string().as_bytes()[0] {
            if let Some(new_connection) = connections.get_mut(&[i, j]) {
                new_connection.push([i, j - 1]);
            }
        }
    }
    if j < map[0].len() - 1 {
        if current.to_string().as_bytes()[0] + 1 >= map[i][j + 1].to_string().as_bytes()[0] {
            if let Some(new_connection) = connections.get_mut(&[i, j]) {
                new_connection.push([i, j + 1]);
            }
        }
    }
}