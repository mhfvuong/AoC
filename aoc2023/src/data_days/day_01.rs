use std::collections::HashMap;

pub fn day_01a(data_string: &String){
    let lines = data_string.lines();

    let mut numbers = Vec::new();

    for line in lines{
        let mut first_number= String::new();
        let mut last_number = String::new();
        for letter in line.chars(){
            if letter.is_numeric(){
                if first_number == "" {
                    first_number = letter.to_string();
                }
                last_number = letter.to_string();
            }
        }
        let combined = format!("{first_number}{last_number}");
        numbers.push(combined);
    }
    let mut sum = 0;
    for number in numbers{
        println!("{}", &number);
        sum += number.parse::<i32>().unwrap();
    }
    print!("{sum}")
}

pub fn day_01b(data_string: String){
    let lines = data_string.lines();

    let mut numbers = Vec::new();
    let number_as_words = HashMap::from([
        ("one", "1"), ("two", "2"), ("three", "3"), ("four", "4"), ("five", "5"),
        ("six", "6"), ("seven", "7"), ("eight", "8"), ("nine", "9")
    ]);
    let words = "one two three four five six seven eight nine";

    for line in lines{
        let mut first_number= String::new();
        let mut last_number = String::new();
        let mut word_slider = String::new();
        let mut skip_first_letter;
        for letter in line.chars(){
            if letter.is_numeric(){
                word_slider = String::new();
                if first_number == "" {
                    first_number = letter.to_string();
                }
                last_number = letter.to_string();
            }
            else {
                word_slider.push(letter);

                if words.contains(&word_slider) {
                    if number_as_words.contains_key(&*word_slider) {
                        if first_number == "" {
                            match number_as_words.get(&*word_slider) {
                                None => (),
                                Some(t) => first_number = t.parse().unwrap(),
                            }
                        }
                        match number_as_words.get(&*word_slider) {
                            None => (),
                            Some(t) => last_number = t.parse().unwrap(),
                        }
                        word_slider = String::new();
                        word_slider.push(letter);
                    }
                }
                else {
                    if words.contains(&word_slider[1..]){
                        skip_first_letter = word_slider.chars();
                        skip_first_letter.next();
                        word_slider = skip_first_letter.as_str().parse().unwrap();
                    }
                    else{
                        word_slider = String::new();
                        word_slider.push(letter);
                    }
                }
            }
        }
        let combined = format!("{first_number}{last_number}");
        numbers.push(combined);
    }
    let mut sum = 0;
    for number in numbers{
        println!("{}", &number);
        sum += number.parse::<i32>().unwrap();
    }
    print!("{sum}")
}