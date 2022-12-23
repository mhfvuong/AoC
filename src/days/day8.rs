pub fn visible_trees(data_string: String) {
    let lines = data_string.lines();
    let mut grid: Vec<Vec<u8>> = Vec::new();
    let mut visible_grid: Vec<Vec<bool>> = Vec::new();
    for (i, line) in lines.enumerate() {
        grid.push(Vec::new());
        visible_grid.push(Vec::new());
        for tree in line.chars() {
            grid[i].push(tree.to_string().parse::<u8>().unwrap());
            visible_grid[i].push(false);
        }
    }
    let mut visible_trees = 0;
    let mut highest_scenic_score = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let mut score_x = [0;2];
            let mut score_y = [0;2];
            let mut taller_x = [false; 2];
            let mut taller_y = [false; 2];
            for x in j+1..grid.len(){
                if !taller_x[0] { score_x[0] += 1; }
                if grid[i][x] >= grid[i][j] { taller_x[0] = true; }
            }
            let mut x = j;
            while x > 0 {
                x -= 1;
                if !taller_x[1] { score_x[1] += 1; }
                if grid[i][x] >= grid[i][j] { taller_x[1] = true; }
            }
            for y in i+1..grid.len(){
                if !taller_y[0] { score_y[0] += 1; }
                if grid[y][j] >= grid[i][j] { taller_y[0] = true; }
            }
            let mut y = i;
            while y > 0 {
                y -= 1;
                if !taller_y[1] { score_y[1] += 1; }
                if grid[y][j] >= grid[i][j] { taller_y[1] = true; }
            }
            let scenic_score = score_x[1] * score_x[0] * score_y[1] * score_y[0];
            if scenic_score > highest_scenic_score {highest_scenic_score = scenic_score;}
            if !(taller_x[0] && taller_x[1] && taller_y[0] && taller_y[1]) {
                visible_trees += 1;
                visible_grid[i][j] = true;
            }
        }
    }
    println!("visible trees: {visible_trees}");
    println!("highest scenic score: {}", highest_scenic_score);
}