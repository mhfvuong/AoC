pub fn rock_paper_scissor(data_string: String){
    let lines = data_string.lines();
    let mut points: u32 = 0;
    for line in lines {
        let split = line.split(' ');
        let vec = split.collect::<Vec<&str>>();
        let opponent = vec[0];
        let you = vec[1];
        match you {
            "Z" => { // win
                points += 6;
                if opponent == "A" {
                    points += 2;
                } else if opponent == "B" {
                    points += 3;
                } else {
                    points += 1;
                }
            },
            "Y" => { // draw
                points += 3;
                if opponent == "A" {
                    points += 1;
                } else if opponent == "B" {
                    points += 2;
                } else {
                    points += 3;
                }
            },
            "X" => { // lose
                // points += 6;
                if opponent == "A" {
                    points += 3;
                } else if opponent == "B" {
                    points += 1;
                } else {
                    points += 2;
                }
            },
            _ => (),
        }
    }
    println!("{points}");
}