struct UnionFind {
    id: Vec<usize>,
    sz: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            id: (0..n).collect(),
            sz: vec![1; n],
        }
    }

    fn root(&mut self, mut i: usize) -> usize {
        while i != self.id[i] {
            self.id[i] = self.id[self.id[i]];
            i = self.id[i];
        }

        i
    }

    fn union(&mut self, p: usize, q: usize) {
        let i = self.root(p);
        let j = self.root(q);

        if i == j {
            return;
        }

        if self.sz[i] < self.sz[j] {
            self.id[i] = j;
            self.sz[j] += self.sz[i];
        } else {
            self.id[j] = i;
            self.sz[i] += self.sz[j];
        }
    }
}

fn main() {
    let mut uf = UnionFind::new(10);

    uf.union(4, 3);
    uf.union(3, 8);
    uf.union(6, 5);
    uf.union(9, 4);
    uf.union(2, 1);
    uf.union(8, 9);
    uf.union(5, 0);
    uf.union(7, 2);
    uf.union(6, 1);
    uf.union(1, 0);
    uf.union(6, 7);

    println!("reference input: 4-3 3-8 6-5 9-4 2-1 8-9 5-0 7-2 6-1 1-0 6-7");
    println!("{:?}", uf.id);
    println!("{:?}", uf.sz);

    let mut uf2 = UnionFind::new(10);

    uf2.union(0, 1);
    uf2.union(2, 3);
    uf2.union(4, 5);
    uf2.union(6, 7);
    uf2.union(0, 2);
    uf2.union(4, 6);
    uf2.union(0, 4);

    println!("worst-case input: 0-1 2-3 4-5 6-7 0-2 4-6 0-4");
    println!("{:?}", uf2.id);
    println!("{:?}", uf2.sz);
}
