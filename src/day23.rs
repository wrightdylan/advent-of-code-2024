use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Graph {
    edges: HashMap<String, HashSet<String>>,
    speed_index: HashMap<char, HashSet<String>>,
    node_to_index: HashMap<String, usize>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
            speed_index: HashMap::new(),
            node_to_index: HashMap::new(),
        }
    }

    pub fn add_edges(&mut self, node1: String, node2: String) {
        self.edges.entry(node1.clone()).or_insert_with(|| HashSet::new()).insert(node2.clone());
        self.edges.entry(node2.clone()).or_insert_with(|| HashSet::new()).insert(node1.clone());

        let first1 = node1.chars().next().unwrap();
        self.speed_index.entry(first1).or_insert_with(HashSet::new).insert(node1);

        let first2 = node2.chars().next().unwrap();
        self.speed_index.entry(first2).or_insert_with(HashSet::new).insert(node2);
    }

    pub fn fast_find(&self, c: &char) -> Option<&HashSet<String>> {
        self.speed_index.get(c)
    }

    pub fn find_maximal_clique(&mut self) -> Vec<Vec<String>> {
        let mut result = Vec::new();

        // Get all vertices
        let vertices: Vec<String> = self.edges.keys().cloned().collect();
        let n = vertices.len();
        let vertices = sort_vertices(vertices);

        // Create mapping from vertex name to index
        for (i, v) in vertices.iter().enumerate() {
            self.node_to_index.insert(v.clone(), i);
        }

        // Create BitVecs for sets R, P, and X
        let mut r = BitVec::with_capacity(n);
        let mut p = BitVec::with_capacity(n);
        let mut x = BitVec::with_capacity(n);
        
        // Initialize P with all vertices
        for i in 0..n {
            p.set_bit(i, true);
        }

        // Run the algorithm
        self.bron_kerbosch(&vertices, &mut r, &mut p, &mut x, &mut result);
        
        result
    }

    // Bron-Kerbosch with degeneracy ordering and bitwise set operations
    fn bron_kerbosch(
        &self,
        vertices: &[String],
        r: &mut BitVec,
        p: &mut BitVec,
        x: &mut BitVec,
        result: &mut Vec<Vec<String>>
    ) {
        // If P and X are both empty, R is a maximal clique
        if p.is_empty() && x.is_empty() {
            let clique: Vec<String> = (0..vertices.len())
                .filter(|&i| r.get_bit(i))
                .map(|i| vertices[i].clone())
                .collect();
            
            if !clique.is_empty() {
                result.push(clique);
            }
            return;
        }
        
        // Choose pivot
        let pivot_idx = self.choose_pivot(vertices, p, x);
        let mut pivot_neighbors = BitVec::with_capacity(vertices.len());

        if let Some(pivot_idx) = pivot_idx {
            // Get neighbors of pivot
            if let Some(neighbors) = self.edges.get(&vertices[pivot_idx]) {
                for neighbor in neighbors {
                    if let Some(&idx) = self.node_to_index.get(neighbor) {
                        pivot_neighbors.set_bit(idx, true);
                    }
                }
            }
        }
        
        // For each vertex in P \ N(pivot)
        let mut p_minus_pivot_neighbors = p.clone();
        p_minus_pivot_neighbors.diff(&pivot_neighbors);
        
        let mut v_idx = 0;
        while let Some(idx) = p_minus_pivot_neighbors.next_set_bit(v_idx) {
            v_idx = idx + 1;
            
            // Add v to R
            r.set_bit(idx, true);
            
            // Get neighbors of v
            let mut v_neighbors = BitVec::with_capacity(vertices.len());
            if let Some(neighbors) = self.edges.get(&vertices[idx]) {
                for neighbor in neighbors {
                    if let Some(&n_idx) = self.node_to_index.get(neighbor) {
                        v_neighbors.set_bit(n_idx, true);
                    }
                }
            }
            
            // Create new P and X as intersection with N(v)
            // P ∩ N(v)
            let mut new_p = p.clone();
            new_p.intersec(&v_neighbors);
            
            // X ∩ N(v)
            let mut new_x = x.clone();
            new_x.intersec(&v_neighbors);
            
            // Recursive call
            self.bron_kerbosch(vertices, r, &mut new_p, &mut new_x, result);
            
            // Remove v from R, add to X
            r.set_bit(idx, false);
            x.set_bit(idx, true);
            
            // Remove v from P (already done by p_minus_pivot_neighbors iteration)
            p.set_bit(idx, false);
        }
    }

    fn choose_pivot(
        &self,
        vertices: &[String],
        p: &BitVec,
        x: &BitVec
    ) -> Option<usize> {
        // Strategy: choose the vertex with the most neighbors in P
        let mut max_neighbors = 0;
        let mut pivot = None;
        
        let mut p_or_x = p.clone();
        p_or_x.union(x);
        
        let mut idx = 0;
        while let Some(v_idx) = p_or_x.next_set_bit(idx) {
            idx = v_idx + 1;
            
            // Count neighbors in P
            let mut count = 0;
            if let Some(neighbors) = self.edges.get(&vertices[v_idx]) {
                for neighbor in neighbors {
                    if let Some(&n_idx) = self.node_to_index.get(neighbor) {
                        if p.get_bit(n_idx) {
                            count += 1;
                        }
                    }
                }
            }
            
            if count > max_neighbors {
                max_neighbors = count;
                pivot = Some(v_idx);
            }
        }
        
        pivot
    }

    pub fn find_triangle_cliques(&self, node_start: char) -> HashSet<Vec<&String>> {
        let mut cliques = HashSet::new();

        if let Some(nodes) = self.fast_find(&node_start) {
            for node in nodes {
                if let Some(t_neighbors) = self.edges.get(node) {
                    for (i, a) in t_neighbors.iter().enumerate() {
                        for b in t_neighbors.iter().skip(i + 1) {
                            if let Some(a_neighbors) = self.edges.get(a) {
                                if a_neighbors.contains(b) {
                                    let mut clique = vec![node, a, b];
                                    clique.sort();
                                    cliques.insert(clique);
                                }
                            }
                        }
                    }
                }
            }
        }

        cliques
    }
}

fn sort_vertices(mut vertices: Vec<String>) -> Vec<String> {
    vertices.sort_by(|a, b| {
        let a_chars: Vec<char> = a.chars().collect();
        let b_chars: Vec<char> = b.chars().collect();

        if a_chars[0] == b_chars[0] {
            a_chars[1].cmp(&b_chars[1])
        } else {
            a_chars[0].cmp(&b_chars[0])
        }
    });

    vertices
}

#[aoc_generator(day23)]
pub fn input_generator(input: &str) -> Graph {
    let mut graph = Graph::new();

    input.lines()
         .filter_map(|line| line.split_once('-'))
         .for_each(|(node1, node2)| graph.add_edges(node1.to_string(), node2.to_string()));

    graph
}

#[aoc(day23, part1)]
pub fn solve_part1(graph: &Graph) -> usize {
    graph.find_triangle_cliques('t').len()
}

#[aoc(day23, part2)]
pub fn solve_part2(graph: &Graph) -> String {
    let cliques = graph.clone().find_maximal_clique();

    if let Some(maximal_clique) = cliques.iter().max_by_key(|clique| clique.len()) {
        maximal_clique.join(",")
    } else {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn part1_test() {
        assert_eq!(solve_part1(&input_generator(TEST)), 7);
    }

    #[test]
    fn part2_test() {
        assert_eq!(solve_part2(&input_generator(TEST)), "co,de,ka,ta");
    }
}