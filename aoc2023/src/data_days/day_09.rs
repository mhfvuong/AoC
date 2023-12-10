pub fn day_09(data_string: String) {
    let lines = data_string.lines();
    let mut history_sum_last = 0;
    let mut history_sum_start = 0;
    for line in lines{
        let mut history_values = Vec::new();
        let numbers = line.split(" ").collect::<Vec<&str>>();
        for number in numbers {
            history_values.push(number.parse::<i32>().unwrap());
        }
        let mut done = false;
        let mut history_length = vec![0, history_values.len()];
        let mut i = 0;

        while done == false {

            let mut zeros = 0;
            for idx in history_length[i]..(history_length[i + 1] - 1) {
                let diff = history_values[idx + 1] - history_values[idx];
                if diff == 0 { zeros += 1; }
                history_values.push(diff);
            }
            if zeros == history_length[i+1] - history_length[i] - 1{
                done = true;
            }
            i += 1;
            history_length.push(history_values.len());
        }
        let mut append_value = 0;
        let mut begin_value = 0;
        while i != 0 {
            append_value += history_values[history_length[i] - 1];
            i -= 1;
            begin_value = history_values[history_length[i]] - begin_value;
        }
        history_sum_last += append_value;
        history_sum_start += begin_value;
    }
    println!("{}", history_sum_last);
    println!("{}", history_sum_start);
}