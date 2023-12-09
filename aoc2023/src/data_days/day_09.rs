pub fn day_09(data_string: String) {
    let lines = data_string.lines();
    let mut history_sum = 0;
    for line in lines{
        let mut history_values = Vec::new();
        let mut number = String::new();
        let mut i = 0;
        for char in line.chars() {
            i += 1;
            if char.is_numeric() {
                number.push(char);
            }
            if !char.is_numeric || i == line.len() {
                history_values.push(number.parse::<i32>().unwrap());
                number = String::new();
            }
        }
        let mut idx = 0;
        let history_length = history_values.len();
        while idx < history_values.len() {
            // do stuff
            pass;
        }
    }
}