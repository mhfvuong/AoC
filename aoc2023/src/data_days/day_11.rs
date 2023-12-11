pub fn day_11(data_string: String){
    let lines = data_string.lines();
    let mut galaxies = Vec::new();
    let mut y = 0;
    let mut galaxy_x = Vec::new();
    for line in lines{
        if galaxy_x.is_empty() {
            for n in 0..line.len(){galaxy_x.push(n);}
        }
        let mut x = 0;
        let mut no_galaxies = true;
        for i in line.chars(){
            if i == '#' {
                galaxies.push([y, x]);
                no_galaxies = false;
                if galaxy_x.contains(&x){ galaxy_x.retain(|idx| idx != &x); }
            }
            x += 1;
        }
        if no_galaxies { y += 1000000; }
        else { y += 1;}
    }
    let mut expanded = 0;
    for empty in galaxy_x {
        let mut expand = false;
        for i in 0..galaxies.len(){
            let x = galaxies[i][1];
            let y = galaxies[i][0];
            if galaxies[i][1] > (empty + expanded) {
                expand = true;
                galaxies.remove(i);
                galaxies.insert(i, [y, x + 999999])
            }
        }
        if expand { expanded += 999999;}
    }
    let mut galaxy_min_pairs = Vec::new();
    for i in 0..galaxies.len() - 1{
        let g1 = galaxies[i];
        let n = i + 1;
        for j in n..galaxies.len() {
            let g2 = galaxies[j];
            let dx = g1[1].abs_diff(g2[1]);
            let dy = g1[0].abs_diff(g2[0]);
            galaxy_min_pairs.push(dx + dy);
        }
    }
    println!("{}", galaxy_min_pairs.iter().sum::<usize>());
}