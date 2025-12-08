use std::collections::HashSet;

pub fn solve_part_1(input: &String) -> u64
{
    let iter_count = 1000;
    let circuit_threshold = 3;

    let points: Vec<_> = input.lines()
        .map(|line| line.split(",")
            .filter_map(|s| s.parse::<i64>().ok())
            .collect::<Vec<_>>())
        .filter_map(|pos| match *pos {
            [x, y, z] => Some((x, y, z)),
            _ => None
        })
        .collect();
    let point_count = points.len();

    let mut edges = vec!();
    for (i1, i2) in (0..point_count)
        .flat_map(|i1| ((i1 + 1)..point_count)
            .map(move |i2| (i1, i2)))
    {
        let (x1, y1, z1) = points[i1];
        let (x2, y2, z2) = points[i2];
        let dx = x2 - x1;
        let dy = y2 - y1;
        let dz = z2 - z1;
        let dist = (dx * dx) + (dy * dy) + (dz * dz);
        edges.push((i1, i2, dist));
    }

    edges.sort_by(|(_, _, d1), (_, _, d2)| d1.cmp(d2));
    let best_edges = &edges[0..(iter_count.min(edges.len()))];

    let mut visited = HashSet::new();
    let mut stack = vec!();
    let mut largest = vec!();
    for start_ind in 0..point_count {
        stack.push(start_ind);

        let mut circuit = HashSet::new();
        while let Some(i) = stack.pop() {
            if visited.contains(&i) {
                continue;
            }
            visited.insert(i);

            circuit.insert(i);
            for &(i1, i2, _) in best_edges.iter() {
                if i == i1 {
                    stack.push(i2);
                }
                if i == i2 {
                    stack.push(i1);
                }
            }
        }

        largest.push(circuit.len());
        if largest.len() > circuit_threshold {
            largest.sort_by(|a, b| b.cmp(a));
            largest.pop();
        }
    }

    return largest.into_iter()
        .map(|n| n as u64)
        .reduce(|a, b| a * b)
        .unwrap_or(0);
}

pub fn solve_part_2(input: &String) -> i64
{
    let points: Vec<_> = input.lines()
        .map(|line| line.split(",")
            .filter_map(|s| s.parse::<i64>().ok())
            .collect::<Vec<_>>())
        .filter_map(|pos| match *pos {
            [x, y, z] => Some((x, y, z)),
            _ => None
        })
        .collect();
    let point_count = points.len();

    let mut edges = vec!();
    for (i1, i2) in (0..point_count)
        .flat_map(|i1| ((i1 + 1)..point_count)
            .map(move |i2| (i1, i2)))
    {
        let (x1, y1, z1) = points[i1];
        let (x2, y2, z2) = points[i2];
        let dx = x2 - x1;
        let dy = y2 - y1;
        let dz = z2 - z1;
        let dist = (dx * dx) + (dy * dy) + (dz * dz);
        edges.push((i1, i2, dist));
    }

    edges.sort_by(|(_, _, d1), (_, _, d2)| d2.cmp(d1));

    struct DSU {
        parents: Vec<usize>,
        sizes: Vec<usize>,
    }

    impl DSU {
        fn new(n: usize) -> Self {
            let mut parents = vec![0; n];
            let sizes = vec![1; n];

            for i in 0..n {
                parents[i] = i;
            }

            return DSU{parents, sizes};
        }

        fn find(&mut self, i: usize) -> usize {
            if self.parents[i] == i {
                return i;
            } else {
                let root = self.find(self.parents[i]);
                self.parents[i] = root;
                return root;
            }
        }

        fn union(&mut self, i: usize, j: usize) {
            let root_i = self.find(i);
            let root_j = self.find(j);

            if root_i == root_j {return}

            if self.sizes[root_i] < self.sizes[root_j] {
                self.parents[root_i] = root_j;
                self.sizes[root_j] += self.sizes[root_i];
            } else {
                self.parents[root_j] = root_i;
                self.sizes[root_i] += self.sizes[root_j];
            }
        }
    }

    let mut dsu = DSU::new(point_count);

    while let Some((u, v, _)) = edges.pop() {
        let u_root = dsu.find(u);
        let v_root = dsu.find(v);
        if u_root != v_root {
            dsu.union(u_root, v_root);
        }

        let root = dsu.find(u);
        if dsu.sizes[root] == point_count {
            return points[u].0 * points[v].0;
        }
    }

    return 0;
}