pub fn tune_device(data_string: String) {
    let lines = data_string.lines();
    let mut counter:usize = 0;
    let mut marker = Vec::new();
    let mut marker_found: bool = false;
    let mut marker_check = Vec::new();
    for line in lines {
        for letter in line.chars() {
            marker.push(letter);
            if marker.len() >= 14 {
                marker_found = true;
                for i in 1..=14 {
                    if marker_check.contains(&marker[marker.len() - i]) {
                        marker_found = false;
                    } else { marker_check.push(marker[marker.len() - i]) }
                }
            }
            marker_check.drain(..);
            counter += 1;
            if marker_found {break}
        }
    }
    println!("marker found after {counter}");
}