pub fn day_04a(data_string: String){
    let mut points = 0;
    let lines = data_string.lines();

    let mut card = 0;
    // let mut cards = 0;
    let mut copies = Vec::new();
    for line in lines {
        // cards += 1;
        card += 1;
        // println!("Card {}", &n);
        let card_numbers = line.split(": ").collect::<Vec<&str>>();
        // println!("{:?}", &card_numbers);
        let numbers = card_numbers[1].split(" | ").collect::<Vec<&str>>();
        let winning_numbers = numbers[0].split(" ").collect::<Vec<&str>>();
        let placeholder = numbers[1].split(" ").collect::<Vec<&str>>();
        let mut elf_numbers = Vec::new();
        for i in placeholder {
            if i.parse::<i32>().is_ok() {
                elf_numbers.push(i);
            }
        }
        // println!("{:?}", &elf_numbers);
        let mut matches = 0;
        let mut copy_card = 1;
        let mut copy_value = 0;
        for en in elf_numbers {
            copy_value = copy_card + card;
            if winning_numbers.contains(&en) {
                if matches == 0 { matches = 1; }
                else {matches *= 2;}
                // println!("{} matches!", &en);
                copies.push(copy_value);
                let mut r = 0;
                for c in &copies {
                    if *c == card {
                        r += 1;
                    }
                }
                for _ in 0..r {
                    copies.push(copy_value);
                }
                copy_card += 1;
            }
        }
        points += matches;
        // println!("{} matches, total points: {}", &matches, &points);
        // println!("{:?}", &copies)
    }
    let mut total_cards = card;
    for i in copies {
        if i <= card {
            total_cards += 1
        }
    }
    println!("{points}");
    println!("{total_cards}");
}