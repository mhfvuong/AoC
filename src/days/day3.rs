use std::collections::HashMap;

pub fn rucksack(data_string: String) {
    let lines = data_string.lines();
    let mut total_priority: u32 = 0;
    let item_values: HashMap<char, u32> = HashMap::from([
        ('a', 1), ('b', 2), ('c', 3), ('d', 4), ('e', 5), ('f', 6), ('g', 7), ('h', 8), ('i', 9), ('j', 10), ('k', 11), ('l', 12), ('m', 13),
        ('n', 14), ('o', 15), ('p', 16), ('q', 17), ('r', 18), ('s', 19), ('t', 20), ('u', 21), ('v', 22), ('w', 23), ('x', 24), ('y', 25), ('z', 26),
        ('A', 27), ('B', 28), ('C', 29), ('D', 30), ('E', 31), ('F', 32), ('G', 33), ('H', 34), ('I', 35), ('J', 36), ('K', 37), ('L', 38), ('M', 39),
        ('N', 40), ('O', 41), ('P', 42), ('Q', 43), ('R', 44), ('S', 45), ('T', 46), ('V', 48), ('W', 49), ('X', 50), ('Y', 51), ('Z', 52),
    ]);
    let mut counter = 0;
    let mut group: Vec<HashMap<char, usize>> = Vec::new();
    for line in lines{
        let ruck_sack: Vec<char> = line.chars().collect();
        // let mut compartment_1: HashMap<char, usize> = HashMap::new();
        // let mut compartment_2: HashMap<char, usize> = HashMap::new();
        let mut elf_rucksack: HashMap<char, usize> = HashMap::new();
        let mut i = 0;
        while i < ruck_sack.len() {
            // if i < ruck_sack.len()/2 {
            //     compartment_1.insert(ruck_sack[i], 1);
            // }
            // else { compartment_2.insert(ruck_sack[i], 1);}
            elf_rucksack.insert(ruck_sack[i], 1);
            i += 1;
        }
        group.push(elf_rucksack);
        counter += 1;
        if counter % 3 == 0 {
            for item in group[group.len() - 1].keys() {
                match group[group.len() - 2].get(item) {
                    None => (),
                    Some(_) => {
                        match group[group.len() - 3].get(item) {
                            None => (),
                            Some(_) => {
                                match item_values.get(item) {
                                    None => (),
                                    Some(t) => total_priority += t,
                                }
                            }
                        }
                    }
                }
            }
        }
        // for item in compartment_1.keys() {
        //     match compartment_2.get(&item) {
        //         None => (),
        //         Some(_) => {
        //             match item_values.get(&item){
        //                 Some(T) => total_priority += T,
        //                 None => ()
        //             }
        //         }
        //     }
        // }
    }
    println!("{total_priority}");
}
