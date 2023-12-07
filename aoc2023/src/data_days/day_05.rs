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
                if some_map == "seed-to-soil map"{
                    map = "sts";
                    println!("{some_map}");
                    continue;
                } else if some_map == "soil-to-fertilizer map" {
                    map = "stf";
                    println!("{some_map}");
                    continue;
                } else if some_map == "fertilizer-to-water map" {
                    map = "ftw";
                    println!("{some_map}");
                    continue;
                } else if some_map == "water-to-light map" {
                    map = "wtl";
                    println!("{some_map}");
                    continue;
                } else if some_map =="light-to-temperature map" {
                    map = "ltt";
                    println!("{some_map}");
                    continue;
                }else if some_map == "temperature-to-humidity map" {
                    map = "tth";
                    println!("{some_map}");
                    continue;
                } else if some_map == "humidity-to-location map" {
                    map = "htl";
                    println!("{some_map}");
                    continue;
                } else if some_map == "" {
                    println!("empty line!");
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
                println!("pairs to check: {}", &pairs_to_check);

                // check if the seed is in range
                let seed_low = seeds.remove(0);
                let seed_range = seeds.remove(0);
                let seed_high = seed_low + seed_range - 1;

                let change = dsr[0] - dsr[1];
                let map_low = dsr[1];
                let map_high = dsr[2] + dsr[1] - 1;
                println!("check if seeds {} - {} fit on map range {} - {}", &seed_low, &seed_high, &map_low, map_high);
                if seed_low >= dsr[1] && seed_high < dsr[1] + dsr[2] {
                    // complete seed range is inside the map: change the whole range
                    println!("changing by {}", &change);
                    println!("changing all seeds from {} to {}", &seed_low, &seed_high);
                    changed_seeds.push(seed_low + change);
                    changed_seeds.push(seed_range);
                    pairs_to_check -= 1;
                }
                else if (seed_low >= dsr[1] && seed_low < dsr[1] + dsr[2]) || // lower half is in the range
                    (seed_high >= dsr[1] && seed_high < dsr[1] + dsr[2]) || // upper half is in the range
                    (dsr[1] >= seed_low && dsr[1] + dsr[2] < seed_high) { // middle is in the range
                    // only the lowest x seed is inside the map range
                    println!("changing by {}", &change);
                    let (new_seeds, old_seeds) = seeds_splitter(seed_low, seed_range, dsr[1], dsr[2], change);
                    if old_seeds.len() > 2 {pairs_to_check += 1;}
                    else {pairs_to_check -= 1;}
                    for s in old_seeds{seeds.push(s);}
                    for s in new_seeds{changed_seeds.push(s);}
                }
                else { // None of the seeds are inside the map range
                    println!("no seeds changed");
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
    // println!("final seeds: {:?}", &seeds);
    let mut i = seeds.len() - 1;
    // println!("{}", seeds.len());
    while i > 0 {
        let _removed = seeds.remove(i);
        // println!("removed range: {_removed}");
        if i > 1 {
            i -= 2;
        }
        else {
            i = 0;
        }
    }
    seeds.sort();
    // println!("lowest seeds per range pair\
    // : {:?}", &seeds);
    println!("closest seed: {}", seeds[0]);
    // println!("seed ranges: {:?}", seeds_ranges);
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

    if seed_low >= source && seed_low <= cut_off {
        println!("changing the seeds {} - {}", &seed_low, &cut_off);
        // println!("from the range: {} - {}", &seed_low, &seed_upper);
        println!("change A");
        seeds_changed.push(seed_low + change);
        let new_range = cut_off - seed_low + 1;
        seeds_changed.push(new_range);
        seeds_unchanged.push(cut_off + 1);
        seeds_unchanged.push(seed_range - new_range);
    }
    else if seed_upper >= source && seed_upper <= cut_off {
        println!("changing the seeds {} - {}", &source, &seed_upper);
        // println!("from the range: {} - {}", &seed_low, &seed_upper);
        println!("change B");
        seeds_changed.push(source + change);
        seeds_changed.push(seed_upper - source + 1);
        seeds_unchanged.push(seed_low);
        seeds_unchanged.push(source - seed_low);
    }
    else if seed_low < source && cut_off < seed_upper {
        println!("changing the seeds {} - {}", &source, &cut_off);
        // println!("from the range: {} - {}", &seed_low, &seed_upper);
        println!("change C");
        seeds_changed.push(source + change);
        seeds_changed.push(source_range);
        seeds_unchanged.push(seed_low);
        seeds_unchanged.push(source - seed_low);
        seeds_unchanged.push(cut_off + 1);
        seeds_unchanged.push(seed_upper - cut_off);
    }
    // println!("new seeds: {:?}", seeds_changed);
    // println!("unchanged: {:?}", seeds_unchanged);

    return (seeds_changed, seeds_unchanged);
}