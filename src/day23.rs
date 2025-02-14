use crate::prelude::*;

pub struct Graph {
    edges: HashMap<String, HashSet<String>>,
    speed_index: HashMap<char, HashSet<String>>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
            speed_index: HashMap::new()
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

    pub fn get(&self, node: &String) -> &HashSet<String> {
        self.edges.get(node).unwrap()
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
    "Nothing".to_string()
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