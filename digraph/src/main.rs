struct Digraph {
    v: usize,
    e: usize,
    adj: Vec<Vec<usize>>,
}

impl Digraph {
    fn new(v: usize) -> Self {
        Digraph {
            v,
            e: 0,
            adj: vec![vec![]; v],
        }
    }

    fn add_edge(&mut self, v: usize, w: usize) {
        self.adj[v].push(w);
        self.e += 1;
    }

    fn reverse(&self) -> Digraph {
        let mut r = Digraph::new(self.v);
        for v in 0..self.v {
            for w in self.adj[v].iter() {
                r.add_edge(*w, v);
            }
        }
        r
    }
}

fn main() {
    let mut g = Digraph::new(13);
    g.add_edge(4, 2);
    g.add_edge(2, 3);
    g.add_edge(3, 2);
    g.add_edge(6, 0);
    g.add_edge(0, 1);
    g.add_edge(2, 0);
    g.add_edge(11, 12);
    g.add_edge(12, 9);
    g.add_edge(9, 10);
    g.add_edge(9, 11);
    g.add_edge(8, 9);
    g.add_edge(10, 12);
    g.add_edge(11, 4);
    g.add_edge(4, 3);
    g.add_edge(3, 5);
    g.add_edge(7, 8);
    g.add_edge(8, 7);
    g.add_edge(5, 4);
    g.add_edge(0, 5);
    g.add_edge(6, 4);
    g.add_edge(6, 9);
    g.add_edge(7, 6);

    let r = g.reverse();
    println!("{:?}", r.adj);
}
