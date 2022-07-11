use std::collections::HashMap;

fn main() {
    let mut wg = WeightedGraph::new();

    wg.add_edge(String::from("sheet"), String::from("disc"), 5);
    wg.add_edge(String::from("sheet"), String::from("poster"), 0);
    wg.add_edge(String::from("poster"), String::from("guitar"), 30);
    wg.add_edge(String::from("poster"), String::from("drum"), 35);
    wg.add_edge(String::from("disc"), String::from("drum"), 20);
    wg.add_edge(String::from("disc"), String::from("guitar"), 15);
    wg.add_edge(String::from("guitar"), String::from("piano"), 20);
    wg.add_edge(String::from("drum"), String::from("piano"), 10);

    let res = wg.dijkstra(String::from("sheet"), String::from("piano"));

    println!("{:?}", res);
    assert_eq!(res, Some(35));
}

struct WeightedGraph {
    g: HashMap<String, HashMap<String, usize>>
}

impl WeightedGraph {
    fn new() -> Self {
        WeightedGraph {
            g: HashMap::new()
        }
    }
    fn add_edge(&mut self, from: String, to: String, weight: usize) {
        self.g.entry(from).or_insert_with(HashMap::new).insert(to, weight);
    }
    fn dijkstra(&self, start: String, end: String) -> Option<usize> {
        let mut costs: HashMap<String, usize> = HashMap::new();
        let mut parents: HashMap<String, String> = HashMap::new();
        let mut processed: Vec<String> = Vec::new();

        costs.insert(start, 0);

        if let Some(mut node) = find_lowest_cost_node(&costs, &processed) {
            while node != end {
                let cost = costs[&node];
                let neighbors = self.g[&node].iter().map(|(n, w)| (n.clone(), *w));
                for (n, w) in neighbors {
                    let new_cost = cost + w;
                    if !costs.contains_key(&n) || costs[&n] > new_cost {
                        costs.insert(n.clone(), new_cost);
                        parents.insert(n.clone(), node.clone());
                    }
                }
                processed.push(node.clone());
                node = find_lowest_cost_node(&costs, &processed).unwrap();
            }
            Some(costs[&end])
        } else {
            None
        }
    }
    
    
}

fn find_lowest_cost_node(costs: &HashMap<String, usize>, processed: &[String]) -> Option<String> {
    let mut lowest_cost = std::usize::MAX;
    let mut lowest_cost_node = None;
    for (node, cost) in costs.iter() {
        if processed.contains(node) {
            continue;
        }
        if *cost < lowest_cost {
            lowest_cost = *cost;
            lowest_cost_node = Some(node.clone());
        }
    }
    lowest_cost_node
}