use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_lines(path: &str) -> Result<Vec<String>, io::Error> {
    BufReader::new(File::open(path)?).lines().collect()
}

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

struct DSU {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        DSU {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) {
        let pa = self.find(a);
        let pb = self.find(b);

        if pa != pb {
            if self.size[pa] < self.size[pb] {
                self.parent[pa] = pb;
                self.size[pb] += self.size[pa];
            } else {
                self.parent[pb] = pa;
                self.size[pa] += self.size[pb];
            }
        }
    }
}

fn main() {
    let lines = read_lines("data.txt").expect("Could not read input");

    let mut points = Vec::new();
    for line in lines {
        let parts: Vec<_> = line.split(',').collect();
        points.push(Point {
            x: parts[0].parse().unwrap(),
            y: parts[1].parse().unwrap(),
            z: parts[2].parse().unwrap(),
        });
    }

    let n = points.len();
    let mut edges = Vec::new();

    // Compute all pairwise distances
    for i in 0..n {
        for j in i + 1..n {
            let dx = points[i].x - points[j].x;
            let dy = points[i].y - points[j].y;
            let dz = points[i].z - points[j].z;
            let dist2 = dx * dx + dy * dy + dz * dz;
            edges.push((dist2, i, j));
        }
    }

    // Sort pairs by distance
    edges.sort_by_key(|e| e.0);

    let mut dsu = DSU::new(n);

    // Connect 1000 closest pairs
    for k in 0..1000 {
        let (_, a, b) = edges[k];
        dsu.union(a, b);
    }

    // Count component sizes
    let mut count = std::collections::HashMap::new();
    for i in 0..n {
        let root = dsu.find(i);
        *count.entry(root).or_insert(0usize) += 1;
    }

    // Take the three largest components
    let mut sizes: Vec<_> = count.values().cloned().collect();
    sizes.sort();
    sizes.reverse();

    let result = sizes[0] * sizes[1] * sizes[2];

    println!("Part 1: {}", result);

    // Part 2

    let mut dsu2 = DSU::new(n);
    let mut last_pair = (0usize, 0usize);

    let mut components = n;

    for &(_dist2, a, b) in &edges {
        let pa = dsu2.find(a);
        let pb = dsu2.find(b);
        if pa != pb {
            dsu2.union(pa, pb);
            components -= 1;
            last_pair = (a, b);

            if components == 1 {
                break;
            }
        }
    }

    let p1 = &points[last_pair.0];
    let p2 = &points[last_pair.1];

    let answer_part2 = p1.x * p2.x;

    println!("Part 2: {}", answer_part2);
}
