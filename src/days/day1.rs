pub fn elf_calories(data_string: String){
    let lines = data_string.lines();
    let mut calories: u32 = 0;
    let mut calories_per_elf: Vec<u32> = Vec::new();
    for line in lines {
        if line.is_empty() {
            // println!("new_elf");
            calories_per_elf.push(calories);
            calories = 0;
        }
        else{
            calories += line.trim().parse::<u32>().unwrap();
        }
    }
    calories_per_elf.sort_by(|a, b| b.cmp(a));
    println!("star 1: {}", calories_per_elf[0]);
    println!("star 2: {}", calories_per_elf[0] + calories_per_elf[1] + calories_per_elf[2]);
}