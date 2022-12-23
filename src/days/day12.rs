pub fn hill_climbing(data_string: String) {
    let lines = data_string.lines();
    let mut start: [usize; 2] = [0; 2];
    let mut end: [usize; 2] = [0; 2];
    let mut map = vec![Vec::new()];
    let mut path = vec![Vec::new()]; // debug path
    for (i, line) in lines.enumerate(){
        if i == map.len() {map.push(Vec::new())}
        if i == path.len() {path.push(Vec::new())} // debug path
        for h in line.chars() {
            if h == 'S' {start = [i, map[i].len()];}
            else if h == 'E' { end = [i, map[i].len()];}
            map[i].push(h);
            path[i].push('.'); // debug path
        }
    }
    check_map(&map,&path);
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