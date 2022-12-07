pub fn file_size(data_string: String) {
    let lines = data_string.lines();
    let mut dir: Vec<&str> = Vec::new();
    let mut dir_size: Vec<usize> = Vec::new();
    let mut index = 0;
    let mut index_children: Vec<Vec<usize>> = Vec::new();
    let mut index_parent: Vec<usize> = Vec::new();
    for line in lines {
        if line.starts_with('$') {
            let command = line.split(' ').collect::<Vec<&str>>();
            if command[1] == "cd" {
                match command[2] {
                    ".." => { // go to parent
                        if let Some(x) = index_parent.pop() { index = x }
                    },
                    "/" => {// starting directory
                        dir.push("/");
                        dir_size.push(0);
                        index_children.push(Vec::new());
                        // parent = "/".to_string();
                    },
                    directory => { // go to child
                        index_parent.push(index);
                        for i in &index_children[index] {
                            if dir[*i] == directory {
                                index = *i;
                            }
                        }
                    },
                }
            }
        }
        else {
            let file = line.split(' ').collect::<Vec<&str>>();
            if file[0] == "dir" {
                dir.push(file[1]);
                dir_size.push(0);
                // add child to index children list by creating an empty list
                index_children.push(Vec::new());
                // add child to list of the current file index
                index_children[index].push(dir.len() - 1);
            }
            else {
                dir_size[index] += file[0].parse::<usize>().unwrap();
                for idx in &index_parent {
                    dir_size[*idx] += file[0].parse::<usize>().unwrap();
                }
            }
        }

    }
    let mut sizes = 0;
    let used_memory = dir_size[0];
    let to_free = used_memory - 40000000;
    let mut to_delete: Vec<usize> = Vec::new();
    for i in dir_size.clone() {

        if i <= 100000 {
            sizes += i;
        }
    }
    for i in dir_size {
        if i >= to_free {
            to_delete.push(i);
        }
    }
    to_delete.sort();

    println!("{sizes}");
    println!("{}", to_delete[0]);
}