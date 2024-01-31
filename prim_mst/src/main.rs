#[derive(Clone, Copy, PartialEq)]
struct Edge {
    v: usize,
    w: usize,
    weight: f64,
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.partial_cmp(&other.weight).unwrap()
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Eq for Edge {}

impl Edge {
    fn new(v: usize, w: usize, weight: f64) -> Edge {
        Edge { v, w, weight }
    }

    fn weight(&self) -> f64 {
        self.weight
    }

    fn either(&self) -> usize {
        self.v
    }

    fn other(&self, v: usize) -> usize {
        if v == self.v {
            self.w
        } else {
            self.v
        }
    }

    fn to_string(&self) -> String {
        format!("{}-{} {:.5}", self.v, self.w, self.weight)
    }
}

struct MinPQ<T> {
    pq: Vec<T>,
}

impl<T: std::cmp::Ord + std::marker::Copy> MinPQ<T> {
    fn new() -> MinPQ<T> {
        MinPQ { pq: Vec::new() }
    }

    fn is_empty(&self) -> bool {
        self.pq.is_empty()
    }

    fn insert(&mut self, x: T) {
        self.pq.push(x);
        let n = self.pq.len();
        self.swim(n - 1);
    }

    fn del_min(&mut self) -> T {
        let n = self.pq.len();
        self.pq.swap(0, n - 1);
        let min = self.pq.pop().unwrap();
        self.sink(0);
        min
    }

    fn swim(&mut self, mut k: usize) {
        while k > 0 && self.less(k, (k - 1) / 2) {
            self.pq.swap(k, (k - 1) / 2);
            k = (k - 1) / 2;
        }
    }

    fn sink(&mut self, mut k: usize) {
        let n = self.pq.len();
        while 2 * k + 1 < n {
            let mut j = 2 * k + 1;
            if j + 1 < n && self.less(j + 1, j) {
                j += 1;
            }
            if self.less(k, j) {
                break;
            }
            self.pq.swap(k, j);
            k = j;
        }
    }

    fn less(&self, i: usize, j: usize) -> bool {
        self.pq[i] < self.pq[j]
    }
}

struct EdgeWeightedGraph {
    v: usize,
    e: usize,
    adj: Vec<Vec<Edge>>,
}

impl EdgeWeightedGraph {
    fn new(v: usize) -> EdgeWeightedGraph {
        EdgeWeightedGraph {
            v,
            e: 0,
            adj: vec![Vec::new(); v],
        }
    }

    fn v(&self) -> usize {
        self.v
    }

    fn add_edge(&mut self, e: Edge) {
        let v = e.either();
        let w = e.other(v);
        self.adj[v].push(e);
        self.adj[w].push(e);
        self.e += 1;
    }

    fn adj(&self, v: usize) -> &Vec<Edge> {
        &self.adj[v]
    }
}

struct LazyPrimMST {
    marked: Vec<bool>,
    mst: Vec<Edge>,
    pq: MinPQ<Edge>,
}

impl LazyPrimMST {
    fn new(g: &EdgeWeightedGraph) -> LazyPrimMST {
        let mut mst = LazyPrimMST {
            marked: vec![false; g.v()],
            mst: Vec::new(),
            pq: MinPQ::new(),
        };
        mst.visit(g, 0);
        while !mst.pq.is_empty() {
            let e = mst.pq.del_min();
            let v = e.either();
            let w = e.other(v);
            if mst.marked[v] && mst.marked[w] {
                continue;
            }
            mst.mst.push(e.clone());
            if !mst.marked[v] {
                mst.visit(g, v);
            }
            if !mst.marked[w] {
                mst.visit(g, w);
            }
        }
        mst
    }

    fn visit(&mut self, g: &EdgeWeightedGraph, v: usize) {
        self.marked[v] = true;
        for e in g.adj(v) {
            if !self.marked[e.other(v)] {
                self.pq.insert(e.clone());
            }
        }
    }

    fn edges(&self) -> &Vec<Edge> {
        &self.mst
    }

    fn weight(&self) -> f64 {
        let mut weight = 0.0;
        for e in self.edges() {
            weight += e.weight();
        }
        weight
    }
}

fn main() {
    let mut g = EdgeWeightedGraph::new(8);
    g.add_edge(Edge::new(4, 5, 0.35));
    g.add_edge(Edge::new(4, 7, 0.37));
    g.add_edge(Edge::new(5, 7, 0.28));
    g.add_edge(Edge::new(0, 7, 0.16));
    g.add_edge(Edge::new(1, 5, 0.32));
    g.add_edge(Edge::new(0, 4, 0.38));
    g.add_edge(Edge::new(2, 3, 0.17));
    g.add_edge(Edge::new(1, 7, 0.19));
    g.add_edge(Edge::new(0, 2, 0.26));
    g.add_edge(Edge::new(1, 2, 0.36));
    g.add_edge(Edge::new(1, 3, 0.29));
    g.add_edge(Edge::new(2, 7, 0.34));
    g.add_edge(Edge::new(6, 2, 0.40));
    g.add_edge(Edge::new(3, 6, 0.52));
    g.add_edge(Edge::new(6, 0, 0.58));
    g.add_edge(Edge::new(6, 4, 0.93));

    let mst = LazyPrimMST::new(&g);
    for e in mst.edges() {
        println!("{}", e.to_string());
    }
    println!("{:.5}", mst.weight());
}
