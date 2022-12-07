pub fn file_size(data_string: String) {
    let lines = data_string.lines();
    let mut dir: Vec<&str> = Vec::new();
    let mut dir_size: Vec<usize> = Vec::new();
    // let mut parent:String = String::new();
    let mut index = 0;
    let mut index_parent: Vec<usize> = Vec::new();
    for line in lines {
        if line.chars().nth(0).unwrap() == '$' {
            let command = line.split(' ').collect::<Vec<&str>>();
            match command[1] {
                "cd" => {
                    match command[2] {
                        ".." => { // go to parent
                            match index_parent.pop() {
                                Some(x) => index = x,
                                None => (),
                            }
                        },
                        "/" => {// starting directory
                            dir.push("/");
                            dir_size.push(0);
                            // parent = "/".to_string();
                        },
                        directory => { // go to child
                            index_parent.push(index);
                            index = dir.iter().position(|&r| r == directory).unwrap();
                        },
                    }
                },
                _ => (),
            }
        }
        else {
            let file = line.split(' ').collect::<Vec<&str>>();
            if file[0] == "dir" {
                dir.push(file[1]);
                dir_size.push(0);
            }
            else {
                dir_size[index] += file[0].parse::<usize>().unwrap();
                for idx in index_parent {
                    dir_size[idx] += file[0].parse::<usize>().unwrap();
                }
            }
        }

    }
    let mut sizes = 0;
    println!("{:?}", &dir);
    println!("{:?}", &dir_size);
    for i in dir_size {
        if i <= 100000 {
            sizes += i;
        }
    }
    println!("{sizes}");
}