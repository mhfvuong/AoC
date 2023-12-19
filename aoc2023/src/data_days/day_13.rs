pub fn day_13(data_string: String) {
    let lines = data_string.lines();
    
    let mut hor = Vec::new();
    let mut vert = Vec::new();
    let mut field = Vec::new();
    let mut vert_old = Vec::new();
    let mut hor_old = Vec::new();
    let lines_copy = lines.clone();
    let mut line_idx = 0;
    for line in lines {
        line_idx += 1;
        if line != "" {
            field.push(line.to_string());
        }
        if line == "" || line_idx == lines_copy.clone().count() {


            let tr_field = translate_field(&field);
            let (hor1, vert1) = find_mirror(&field, &tr_field);
            if !hor1.is_empty() {hor_old.push(hor1[0]);}
            else {vert_old.push(vert1[0]);}

            let mut mirror = find_second_mirror(&field, &hor1);
            if !mirror.is_empty() { hor.push(mirror[0]); }
            else {
                mirror = find_second_mirror(&tr_field, &vert1);
                vert.push(mirror[0]);
            }
            field = Vec::new();
        }
    }
    let total_vert_1 = vert_old.iter().sum::<i32>();
    let total_hor_1 = hor_old.iter().sum::<i32>() * 100;
    let total_vert = vert.iter().sum::<i32>();
    let total_hor = hor.iter().sum::<i32>() * 100;
    println!("{}", total_vert_1 + total_hor_1);
    println!("{}", total_vert + total_hor);
}

fn find_mirror_to_edge(field: &Vec<String>, i: usize) -> bool {
    let mut m = i;
    let mut n = i+1;
    let mut edge = false;
    let mut mirror_found = true;
    while !edge && mirror_found{
        if field[m] == field[n] {
            if m != 0 { m -= 1;}
            else {edge = true;}
            if n != field.len() - 1 { n += 1;}
            else {edge = true;}
        }
        else {mirror_found = false;}
    }
    return mirror_found;
}

fn translate_field(field: &Vec<String>) -> Vec<String> {
    let mut tr_field = Vec::new();
    for i in 0..field[0].len() {
        let mut newline = String::new();
        for h in field{
            newline.push(h.chars().nth(i).unwrap());
        }
        tr_field.push(newline);
    }
    return tr_field;
}

fn find_mirror(field: &Vec<String>, tr_field: &Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let mut vert = Vec::new();
    let mut hor = Vec::new();
    for i in 0..field.len() - 1{
        if field[i] == field[i+1] {
            // check if all match until the edge
            let mirror_found = find_mirror_to_edge(&field, i);
            if mirror_found {
                hor.push((i + 1) as i32);
            }
        }
    }
    for i in 0..tr_field.len() - 1{
        if tr_field[i] == tr_field[i+1] {
            // check if all match until the edge
            let mirror_found = find_mirror_to_edge(&tr_field, i);
            if mirror_found {
                vert.push((i + 1) as i32);
            }
        }
    }
    return (hor, vert);
}

fn find_second_mirror(field: &Vec<String>, known_mirror: &Vec<i32>) -> Vec<i32> {
    let mut mirror_location = Vec::new();
    let mut m = 0;
    let mut n = 1;
    let mut traversed = false;
    let mut smudge_mirror = Vec::new();
    while !traversed {
        if n == field.len() { m += 1; n = m + 1;}
        if m == field.len() - 1 { traversed = true; continue;}
        let check_1 = &field[m];
        let check_2 = &field[n];
        let mut diff = 0;
        let mut i = 0;
        while i < check_1.len() && diff < 2 {
            if check_1.chars().nth(i) != check_2.chars().nth(i) {
                diff += 1;
            }
            i += 1;
        }
        if diff == 1 {smudge_mirror.push([m, n]);}
        n += 1;
    }
    for l1l2 in smudge_mirror {
        let mut new_field = field.clone();
        let index = l1l2[0];
        let mirror_idx = l1l2[1];
        let element = new_field[mirror_idx].clone();
        new_field.remove(index);
        new_field.insert(index, element);
        for i in 0..field.len() - 1{
            if new_field[i] == new_field[i+1] {
                // check if all match until the edge
                let mirror_found = find_mirror_to_edge(&new_field, i);
                if mirror_found && !known_mirror.contains(&((i + 1) as i32)) && mirror_location.is_empty(){
                    mirror_location.push((i + 1) as i32);
                }
                
            }
        }
    }
    return mirror_location;
}