use std::collections::{VecDeque, HashMap};

fn main() {
    let mut g = Graph::new();

    let to_be_found = String::from("thom");

    g.add_edge(String::from("you"), String::from("alice"));
    g.add_edge(String::from("you"), String::from("bob"));
    g.add_edge(String::from("you"), String::from("claire"));
    g.add_edge(String::from("bob"), String::from("anuj"));
    g.add_edge(String::from("bob"), String::from("peggy"));
    g.add_edge(String::from("alice"), String::from("peggy"));
    g.add_edge(String::from("claire"), String::from("thom"));
    g.add_edge(String::from("claire"), String::from("jonny"));

    println!("{:?}", g.bfs(String::from("you"), |name| name == to_be_found));

    println!("{:?}", g.dfs(String::from("you"), |name| name == to_be_found));
}

struct Graph {
    g: HashMap<String, Vec<String>>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            g: HashMap::new(),
        }
    }
    fn add_edge(&mut self, from: String, to: String) {
        self.g.entry(from).or_insert(Vec::new()).push(to);
    }
    fn bfs(&self, start: String, validator: impl Fn(String) -> bool) -> Option<String> {
        let mut visited: HashMap<String, bool> = HashMap::new();
        let mut queue: VecDeque<String> = VecDeque::new();
        queue.push_back(start.clone());
        visited.insert(start, true);
        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            println!("{}", node);
            for n in self.g[&node].iter() {
                if !visited.contains_key(n) {
                    if validator(n.clone()) {
                        return Some(n.clone());
                    } else {
                        queue.push_back(n.clone());
                        visited.insert(n.clone(), true);
                    }
                }
            }
        }
        None
    }
    fn dfs(&self, start: String, validator: impl Fn(String) -> bool) -> Option<String> {
        let mut visited: HashMap<String, bool> = HashMap::new();
        let mut stack: Vec<String> = vec![start.clone()];
        visited.insert(start, true);
        while !stack.is_empty() {
            let node = stack.pop().unwrap();
            println!("{}", node);
            for n in self.g[&node].iter() {
                if !visited.contains_key(n) {
                    if validator(n.clone()) {
                        return Some(n.clone());
                    } else {
                        stack.push(n.clone());
                        visited.insert(n.clone(), true);
                    }
                }
            }
        }
        None
    }
}