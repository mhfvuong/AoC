use std::collections::HashMap;

pub fn day_05(data_string: String){
    let lines = data_string.lines();
    let mut seeds = Vec::new();
    let mut seeds_ranges = Vec::new();
    let mut map = "";
    // let mut dsr_map = Vec::new();
    let mut changed_seeds = Vec::new();
    // let mut map_num = Vec::new();
    let mut change_idx = Vec::new();
    for line in lines {
        let map_num = line.split(":").collect::<Vec<&str>>();
        match map_num[0] {
            "seeds" => {
                // println!("{}", &map_num[1].len());
                let all_seeds = chars_to_vec(&map_num[1]);
                for i in 0..all_seeds.len(){
                    if i % 2 != 0 { seeds_ranges.push(all_seeds[i]);}
                    else {seeds.push(all_seeds[i]);}
                }
                // for i in seeds {changed_seeds.insert(i, false);}
                println!("finding seeds:");
                println!("{:?}", &seeds);
            },
            some_map => {
                if some_map == "seed-to-soil map"{
                    map = "sts";
                    println!("map 1");
                    continue;
                } else if some_map == "soil-to-fertilizer map" {
                    map = "stf";
                    println!("map 2");
                    continue;
                } else if some_map == "fertilizer-to-water map" {
                    map = "ftw";
                    println!("map 3");
                    continue;
                } else if some_map == "water-to-light map" {
                    map = "wtl";
                    println!("map 4");
                    continue;
                } else if some_map =="light-to-temperature map" {
                    map = "ltt";
                    println!("map 5");
                    continue;
                }else if some_map == "temperature-to-humidity map" {
                    map = "tth";
                    println!("map 6");
                    continue;
                } else if some_map == "humidity-to-location map" {
                    map = "htl";
                    println!("map final");
                    continue;
                } else if some_map == "" {
                    println!("empty line!");
                    map = "";
                    for cs in &changed_seeds {
                        seeds.push(*cs);
                    }
                    change_idx.sort();
                    change_idx.reverse();
                    for idx in change_idx {
                        seeds.remove(idx);
                    }
                    change_idx = Vec::new();
                    changed_seeds = Vec::new();
                    println!("seeds: {:?}", &seeds);
                    // continue;
                }
            },
        }
        // let mut dsr = Vec::new();
        if map != "" {
            let dsr = chars_to_vec(&map_num[0]);

            for i in 0..seeds.len(){
                // check if the seed is in range
                let sr = seeds[i] + seeds_ranges[i];
                // check for either upper limit or lower limit or middle
                if seeds[i] < dsr[1] + dsr[2] && seeds[i] >= dsr[1]{}
                else if sr < dsr[1] + dsr[2] && sr >= dsr[1] {}
                else if seeds[i] < dsr[1] && dsr[2] < seeds_ranges[i] {}
                else if dsr[1] <= seeds[i] && seeds_ranges[i] < dsr[1] + dsr[2]{

                    let difference = dsr[0] - dsr[1];
                    let abs_diff = difference.abs();
                    let mut change = 0;
                    if difference > 0 { change = seeds[i] + abs_diff; } else { change = seeds[i] - abs_diff; }
                    // unchanged = false;
                    if !changed_seeds.contains(&change){
                        changed_seeds.push(change);
                        change_idx.push(i);
                        println!("CHANGE {} to {}", &seeds[i],&change);
                    }
                }
            }
            println!("new seed locations: {:?}", &changed_seeds);
        }
    }
    for cs in &changed_seeds {
        seeds.push(*cs);
    }
    change_idx.sort();
    change_idx.reverse();
    for idx in change_idx {
        seeds.remove(idx);
    }
    seeds.sort();
    println!("closest seed: {}", seeds[0]);
    println!("seed ranges: {:?}", seeds_ranges);
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

pub fn seeds_splitter (seed: i64, seed_range: i64, source: i64, source_rage: i64) -> Vec<i64> {
    let mut seed_splits = Vec::new();
    let cut_off = source + source_rage;
    let total_seed_range = seed + seed_range;
    let seed_range_1 = cut_off - seed;
    let seed_range_2 = total_seed_range - cut_off;
    seed_splits.push(seed);
    seed_splits.push(seed_range_1);
    seed_splits.push(cut_off);
    seed_splits.push(seed_range_2);
    return seed_splits;
}