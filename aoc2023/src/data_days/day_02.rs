pub fn day_02ab(data_string: String){
    let lines = data_string.lines();
    let mut game_counter = 0;
    let mut possible_games = 0;
    let game_config = [12, 13, 14]; //r,g,b
    let mut power_counter = 0;

    for line in lines{
        game_counter += 1;
        let mut possible = true;
        let id_bag = line.split(": ").collect::<Vec<&str>>();
        println!("{}", &id_bag[1]);
        let bag = id_bag[1].split("; ").collect::<Vec<&str>>();

        let mut min_r = 0;
        let mut min_g = 0;
        let mut min_b = 0;
        for hand in bag{
            let cubes = hand.split(", ").collect::<Vec<&str>>();
            for color in cubes{
                let num_color = color.split(" ").collect::<Vec<&str>>();
                let num = &num_color[0].parse::<i32>().unwrap();
                let color = num_color[1];
                if color == "red"{
                    if num > &game_config[0] {possible = false; println!("impossible game!");}
                    if num > &min_r {min_r = *num;}
                }
                else if color == "green" {
                    if num > &game_config[1] {possible = false; println!("impossible game!");}
                    if num > &min_g {min_g = *num;}
                }
                else if color == "blue" {
                    if num > &game_config[2] {possible = false; println!("impossible game!");}
                    if num > &min_b {min_b = *num;}
                }
            }
        }
        let power = min_r * min_g * min_b;
        println!("power of this set is:{}", &power);
        power_counter += power;
        if possible {
            println!("{} is a possible game", &game_counter);
            possible_games += game_counter;
        }
        // else {
        //     possible = true;
        // }
    }
    println!("{possible_games}");
    println!("{power_counter}");
}