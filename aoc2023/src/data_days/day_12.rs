pub fn day_12(data_string: String) {
    let lines = data_string.lines();
    for line in lines {
        let spring_numbers = line.split(' ').collect::<Vec<&str>>();
        let mut numbers = Vec::new();
        let mut numbr = String::new();
        let mut i = 0;
        for num in spring_numbers[1].chars() {
            i += 1;
            if num.is_numeric() {numbr.push(num)}
            if !num.is_numeric() || i == spring_numbers[1].len() {
                numbers.push(numbr.parse::<usize>().unwrap());
                numbr = String::new();
            }
        }
        // i = 0;
        // let mut j = 0;
        // let mut group = "";
        // let mut num_idx = 0;
        // while i < spring_numbers[0].len(){ // while you haven't found all groups
        //     if spring_numbers[0].chars().nth(i).unwrap() == '.' {
        //         i += 1;
        //         group = "";
        //     }
        //     else {
        //         while spring_numbers[0].chars().nth(i+j).unwrap() != '.' {
        //             j += 1;
        //         }
        //         group = &spring_numbers[0][i..i+j]; // created a group
        //         // while num_idx != numbers.len(){
        //         let mut spring_idx = 0;
        //         // let mut numbers_in_group = Vec::new();
        //         // if numbers[num_idx] < group.len() - (numbers_in_group.iter().sum() + numbers_in_group.len()){ // if the number fits, excluding the saved numbers, and their extra spacing
        //         //     let mut new_spring_idx_found = false;
        //         //     while !new_spring_idx_found {
        //         //         if group.chars().nth(numbers[num_idx] + spring_idx).unwrap() != '#' { // it is possible
        //         //             spring_idx += numbers[num_idx]; // update spring index
        //         //             numbers_in_group.push(numbers[num_idx]); // remember which number has been grouped
        //         //             num_idx += 1; // go to the next number
        //         //             new_spring_idx_found = true;
        //         //         } else {
        //         //             spring_idx += 1;
        //         //         }
        //         //     }
        //         // }
        //         // else if numbers[num_idx] < group.len() - (numbers_in_group.iter().sum() + numbers_in_group.len()) { // if it fits exactly
        //         //     numbers_in_group.push(numbers[num_idx]);
        //         //     num_idx += 1;
        //         // }
        //     }
        // }
    }
}