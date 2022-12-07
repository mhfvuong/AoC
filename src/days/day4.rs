// use std::slice::range;

pub fn cleanup_pairs(data_string: String) {
    let lines = data_string.lines();

    let mut fully_contained: usize = 0;
    let mut overlapping: usize = 0;

    for line in lines {
        let pairs = line.split(',').collect::<Vec<&str>>();
        let range_1 = pairs[0].split('-').collect::<Vec<&str>>();
        let range_2 = pairs[1].split('-').collect::<Vec<&str>>();
        let low_1 = range_1[0].parse::<u8>().unwrap();
        let high_1 = range_1[1].parse::<u8>().unwrap();
        let low_2 = range_2[0].parse::<u8>().unwrap();
        let high_2 = range_2[1].parse::<u8>().unwrap();
        if (low_2 <= low_1 && high_1 <= high_2)|| (low_1 <= low_2 && high_2 <= high_1){
            fully_contained += 1;
            overlapping += 1;
        }
        else if (low_2 <= low_1 && low_1 <= high_2) || (low_2 <= high_1 && high_1 <= high_2) || (low_1 <= low_2 && low_2 <= high_1) || (low_1 <= high_2 && high_2 <= high_1) {
            overlapping += 1
        }
    }
    println!("fully contained:   {}", fully_contained);
    println!("overlapping pairs: {}", overlapping);
}