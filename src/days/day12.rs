use std::collections::HashMap;

#[derive(Debug)]
struct Graph {
    map: Vec<Vec<u8>>,
    nodes: Vec<usize>,
    edges: HashMap<usize, Vec<usize>>,
    queue: Vec<usize>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            map: vec![Vec::new()],
            nodes: Vec::new(),
            edges: HashMap::new(),
            queue: Vec::new(),
        }
    }

    fn add_node(&mut self, node: usize) {
        self.nodes.push(node);
        self.edges.insert(node, Vec::new());
    }

    fn add_edge(&mut self, node1: usize, height1: u8, node2: usize, height2: u8) {
        if height1 + 1 >= height2 {
            if let Some(edge) = self.edges.get_mut(&node1) {
                edge.push(node2);
            }
        }
    }

    fn look_around_add(&mut self, h: usize, w: usize) {
        let cn = w + h * self.map[0].len();
        let width = self.map[0].len();
        let ch = self.map[h][w];
        if h > 0 {
            self.add_edge(cn, ch, cn - width, self.map[h - 1][w]);
        }
        if h < self.map.len() - 1 {
            self.add_edge(cn, ch, cn + width, self.map[h + 1][w]);
        }
        if w > 0 {
            self.add_edge(cn, ch, cn - 1, self.map[h][w - 1]);
        }
        if w < width - 1 {
            self.add_edge(cn, ch, cn + 1, self.map[h][w + 1]);
        }
    }
}

pub fn hill_climbing(data_string: String) {
    let lines = data_string.lines();
    let mut graph = Graph::new();
    let mut start: usize = 0;
    let mut end: usize = 0;
    let mut a_list = Vec::new(); // part 2
    let mut path_to = vec![Vec::new()];
    for (i, line) in lines.enumerate() {
        if i == graph.map.len() {
            graph.map.push(Vec::new());
            path_to.push(Vec::new());
        }
        let mut n = 0;
        for h in line.chars() {
            path_to[i].push(-1);
            graph.add_node(n + i * graph.map[0].len());
            if h == 'S' {
                start = n + i * graph.map[0].len();
                graph.map[i].push(String::from('a').as_bytes()[0]);
            }
            else if h == 'E' {
                end = n + i * graph.map[0].len();
                graph.map[i].push(String::from('z').as_bytes()[0]);
            }
            else {
                if h == 'a' { a_list.push(n + i * graph.map[0].len());} // part 2
                graph.map[i].push(h.to_string().as_bytes()[0]);
            }
            n += 1;
        }
    }
    for h in 0..graph.map.len() {
        for w in 0..graph.map[0].len() {
            graph.look_around_add(h, w);
        }
    }

    path_to[start/graph.map[0].len()][start%graph.map[0].len()] = 0;
    graph.queue.push(start);
    while !graph.queue.is_empty() {
        let current_node = graph.queue.remove(0);
        let w = current_node % graph.map[0].len();
        let h = current_node / graph.map[0].len();
        let path = path_to[h][w] + 1;
        if current_node != end {
            if let Some(neighbors) = graph.edges.get(&current_node) {
                for n_bors in neighbors {
                    let hn = n_bors / graph.map[0].len();
                    let wn = n_bors % graph.map[0].len();
                    if path_to[hn][wn] == -1 || path < path_to[hn][wn] {
                        if a_list.contains(n_bors) { // part 2
                            path_to[hn][wn] = 0;
                        }
                        else { // end part 2
                            path_to[hn][wn] = path;
                        }
                        graph.queue.push(*n_bors);
                    }
                }
            }
        }
        else {
            println!("steps to reach the end: {}", path_to[h][w]);
        }
    }
}
