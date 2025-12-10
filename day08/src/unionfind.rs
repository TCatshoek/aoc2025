pub struct UnionFind {
    pub parent: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self { parent: (0..n).collect() }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let px = self.find(x);
        let py = self.find(y);
        self.parent[px] = py;
    }
}