pub fn day_05(data_string: String){
    let lines = data_string.lines();
    let mut seeds = Vec::new();
    let mut map = "";
    let mut changed_seeds = Vec::new();
    for line in lines {
        let map_num = line.split(":").collect::<Vec<&str>>();
        match map_num[0] {
            "seeds" => {
                seeds = chars_to_vec(&map_num[1]);
            },
            some_map => {
                if some_map == "seed-to-soil map" || some_map == "soil-to-fertilizer map" ||
                    some_map == "fertilizer-to-water map" || some_map == "water-to-light map" ||
                    some_map =="light-to-temperature map"|| some_map == "temperature-to-humidity map" ||
                    some_map == "humidity-to-location map" {
                    map = some_map;
                    // println!("{some_map}");
                    continue;
                } else if some_map == "" {
                    // println!("empty line!");
                    map = "";
                    for _cs in 0..changed_seeds.len() {
                        seeds.push(changed_seeds.remove(0));
                    }
                }
            },
        }

        if map != "" {
            let dsr = chars_to_vec(&map_num[0]);
            let mut pairs_to_check = seeds.len()/2;
            while pairs_to_check > 0 {
                // check if the seed is in range
                let seed_low = seeds.remove(0);
                let seed_range = seeds.remove(0);
                let seed_high = seed_low + seed_range - 1;

                let change = dsr[0] - dsr[1];

                if seed_low >= dsr[1] && seed_high < dsr[1] + dsr[2] {
                    // complete seed range is inside the map: change the whole range
                    changed_seeds.push(seed_low + change);
                    changed_seeds.push(seed_range);
                    pairs_to_check -= 1;
                }
                else if (seed_low >= dsr[1] && seed_low < dsr[1] + dsr[2]) || // lower part is in the range
                    (seed_high >= dsr[1] && seed_high < dsr[1] + dsr[2]) || // upper part is in the range
                    (dsr[1] >= seed_low && dsr[1] + dsr[2] < seed_high) { // middle part is in the range
                    let (new_seeds, old_seeds) = seeds_splitter(seed_low, seed_range, dsr[1], dsr[2], change);
                    if old_seeds.len() > 2 {pairs_to_check += 1;}
                    else {pairs_to_check -= 1;}
                    for s in old_seeds{seeds.push(s);}
                    for s in new_seeds{changed_seeds.push(s);}
                }
                else { // None of the seeds are inside the map range
                    seeds.push(seed_low);
                    seeds.push(seed_range);
                    pairs_to_check -= 1;
                }

            }
        }
    }
    for cs in &changed_seeds {
        seeds.push(*cs);
    }
    let mut i = seeds.len() - 1;
    while i > 0 {
        let _removed = seeds.remove(i);
        if i > 1 {
            i -= 2;
        }
        else {
            i = 0;
        }
    }
    seeds.sort();
    println!("closest seed: {}", seeds[0]);
}

fn chars_to_vec(characters: &str) -> Vec<i64> {
    let mut n = 0;
    let mut number_vec = String::new();
    let mut final_vec = Vec::new();
    for s in characters.chars() {
        if s.is_numeric() {
            number_vec.push(s);
        }
        if !s.is_numeric() || n == characters.len() - 1 {
            if number_vec.len() != 0 {
                final_vec.push(number_vec.parse::<i64>().unwrap());
                number_vec = String::new();
            }
        }
        n += 1;
    }
    return final_vec;
}

fn seeds_splitter (seed_low: i64, seed_range: i64, source: i64, source_range: i64, change: i64) -> (Vec<i64>, Vec<i64>){
    let mut seeds_changed = Vec::new();
    let mut seeds_unchanged = Vec::new();
    let seed_upper = seed_low + seed_range - 1;
    let cut_off = source + source_range - 1;

    if seed_low >= source && seed_low <= cut_off { // lower part is in the range
        seeds_changed.push(seed_low + change);
        let new_range = cut_off - seed_low + 1;
        seeds_changed.push(new_range);
        seeds_unchanged.push(cut_off + 1);
        seeds_unchanged.push(seed_range - new_range);
    }
    else if seed_upper >= source && seed_upper <= cut_off { // upper part is in the range
        seeds_changed.push(source + change);
        seeds_changed.push(seed_upper - source + 1);
        seeds_unchanged.push(seed_low);
        seeds_unchanged.push(source - seed_low);
    }
    else if seed_low < source && cut_off < seed_upper { // middle part is in the range
        seeds_changed.push(source + change);
        seeds_changed.push(source_range);
        seeds_unchanged.push(seed_low);
        seeds_unchanged.push(source - seed_low);
        seeds_unchanged.push(cut_off + 1);
        seeds_unchanged.push(seed_upper - cut_off);
    }
    return (seeds_changed, seeds_unchanged);
}