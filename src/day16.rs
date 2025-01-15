#![allow(dead_code)]
use std::usize;
use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Maze {
    End,
    Path,
    Start,
    Wall,
}

#[derive(Debug, Clone)]
pub struct Edge {
    start: usize,
    start_dir: Ortho,
    end: usize,
    end_dir: Ortho,
    path: HashSet<(usize, usize)>,
    weight: usize,
}

impl Edge {
    fn has_node(&self, node: usize) -> bool {
        if self.start == node || self.end == node {
            return true;
        }

        false
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pos: (usize, usize),
    exits: Vec<((usize, usize), Ortho)>
}

impl Node {
    fn new(pos: &(usize, usize)) -> Self {
        Node { pos: pos.clone(), exits: Vec::new() }
    }

    fn new_with_exits(pos: &(usize, usize), exits: &Vec<((usize, usize),Ortho)>) -> Self {
        Node { pos: pos.clone(), exits: exits.clone() }
    }
}

fn get_node_num(node_list: &mut HashMap<usize, Node>, pos: &(usize, usize), neighbours: &Vec<((usize, usize), Ortho)>) -> usize {
    if let Some((key, _)) = node_list.iter().find(|&(_, node)| node.pos == pos.clone()) {
        return *key;
    }

    let new_node = node_list.len();
    node_list.insert(new_node, Node::new_with_exits(&pos, neighbours));
    new_node
}

#[derive(Eq, PartialEq)]
struct State {
    dist: usize,
    node: usize,
    dir: Ortho,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.dist.cmp(&self.dist)
            .then_with(|| self.node.cmp(&other.node))
            .then_with(|| self.dir.cmp(&other.dir))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn reconstruct_paths(
    end: usize,
    entry_dir: Ortho,
    previous: &HashMap<(usize, Ortho), Vec<(usize, Ortho)>>
) -> Vec<Vec<usize>> {
    let mut paths = Vec::new();
    let mut stack = vec![vec![end]];
    let mut current_states = vec![(end, entry_dir)];

    while let Some(current_path) = stack.pop() {
        let current_state = current_states.pop().unwrap();
        
        if let Some(prev_states) = previous.get(&current_state) {
            if prev_states.is_empty() {
                if current_path.len() > 1 {
                    paths.push(current_path);
                }
            } else {
                for &prev_state in prev_states {
                    let mut new_path = current_path.clone();
                    new_path.push(prev_state.0);
                    stack.push(new_path);
                    current_states.push(prev_state);
                }
            }
        } else if current_path.len() > 1 {
            paths.push(current_path);
        }
    }

    // Reverse all paths since we built them backwards
    for path in &mut paths {
        path.reverse();
    }

    paths
}

pub struct Graph {
    nodes: HashMap<usize, Node>,
    edges: HashMap<(usize, usize, Ortho, Ortho), Edge>,
}

impl Graph {
    pub fn new(nodes: HashMap<usize, Node>, edges: HashMap<(usize, usize, Ortho, Ortho), Edge>) -> Self {
        Self { nodes, edges }
    }

    pub fn edges_with_node(&self, node: &usize) -> Vec<Edge> {
        let mut edges = Vec::new();

        for ((key1, key2, _, _), edge) in self.edges.iter() {
            if key1 == node || key2 == node {
                edges.push(edge.clone());
            }
        }
        edges
    }

    pub fn get_edges(&self, node1: usize, node2: usize) -> Vec<Edge> {
        let (key1, key2, _, _) = normalise_edge(node1, node2, Ortho::North, Ortho::North);
        let mut results = Vec::new();
        let mut min = usize::MAX;

        for (&(first, second, _, _), edge) in self.edges.iter() {
            if key1 == first && key2 == second {
                if edge.weight < min {
                    min = edge.weight;
                    results.clear();
                    results.push(edge.clone());
                } else if edge.weight == min {
                    results.push(edge.clone());
                }
            }
        }

        results
    }

    pub fn dijkstra(&self, start: usize, end: usize, all_paths: bool) -> Option<Vec<(Vec<usize>, usize)>> {
        let mut distances: HashMap<(usize, Ortho), usize> = HashMap::new();
        let mut previous: HashMap<(usize, Ortho), Vec<(usize, Ortho)>> = HashMap::new();
        let mut heap: BinaryHeap<State> = BinaryHeap::new();
        let mut visited: HashSet<(usize, Ortho)> = HashSet::new();

        for &id in self.nodes.keys() {
            for dir in Ortho::iter() {
                distances.insert((id, dir), usize::MAX);
            }
        }

        distances.insert((start, Ortho::East), 0);
        heap.push(State {
            dist: 0,
            node: start,
            dir: Ortho::East,
        });

        let mut shortest_dist = None;
        let mut result_paths = Vec::new();

        while let Some(State { dist: current_dist, node: current_node, dir: entry_dir }) = heap.pop() {
            let current_state = (current_node, entry_dir);
            
            if let Some(dist) = shortest_dist {
                if current_dist > dist {
                    continue;
                }
            }

            if visited.contains(&current_state) {
                continue;
            }
            visited.insert(current_state);

            if current_node == end {
                if shortest_dist.is_none() {
                    shortest_dist = Some(current_dist);
                }

                let paths = reconstruct_paths(end, entry_dir, &previous);
                let first_path = paths[0].clone();
                if !all_paths {
                    return Some(vec![(first_path, current_dist)]);
                }
                result_paths.extend(paths.into_iter().map(|path| (path, current_dist)));
                continue;
            }

            if current_dist > distances[&current_state] {
                continue;
            }

            for next in self.edges_with_node(&current_node).iter() {
                let from_start = current_node == next.start;
                let launch_dir = if from_start { next.start_dir } else { next.end_dir };
                if entry_dir == launch_dir.flip() {
                    continue;
                }
                let turning_cost = if entry_dir == launch_dir { 0 } else { 1000 };
                let (exit_node, exit_dir) = if from_start {
                    (next.end, next.end_dir.flip())
                } else {
                    (next.start, next.start_dir.flip())
                };

                let next_state = (exit_node, exit_dir);
                let new_dist = distances[&current_state]
                    .saturating_add(1)
                    .saturating_add(turning_cost)
                    .saturating_add(next.weight);

                if new_dist < distances[&next_state] {
                    distances.insert(next_state, new_dist);
                    previous.insert(next_state, vec![(current_node, entry_dir)]);
                    heap.push(State {
                        dist: new_dist,
                        node: exit_node,
                        dir: exit_dir,
                    });
                } else if new_dist == distances[&next_state] && all_paths {
                    previous.entry(next_state)
                        .or_default()
                        .push((current_node, entry_dir));
                }
            }
        }

        if result_paths.is_empty() {
            None
        } else {
            Some(result_paths)
        }
    }
}

pub trait Neighbours {
    fn get_neighbours(&self, pos: &(usize, usize)) -> Vec<((usize, usize), Ortho)>;
}

impl<T> Neighbours for Grid<T>
where
    T: Clone + Copy + PartialEq<Maze>,
{
    fn get_neighbours(&self, pos: &(usize, usize)) -> Vec<((usize, usize), Ortho)> {
        let mut neighbours = Vec::new();

        for (dr, dc) in &ORTHO {
            let new_r = (pos.1 as i32 + dr) as usize;
            let new_c = (pos.0 as i32 + dc) as usize;

            if new_r < self.height && new_c < self.width {
                let tile = &self[(new_c, new_r)];
                if *tile == Maze::Path || *tile == Maze::End {
                    let en = match (dc, dr) {
                        (0, 1) => Ortho::South,
                        (1, 0) => Ortho::East,
                        (0, -1) => Ortho::North,
                        (-1, 0) => Ortho::West,
                        _ => unreachable!(),
                    };
                    neighbours.push(((new_c, new_r), en));
                }
            }
        }

        neighbours
    }
}

// For easy indexing, start node = 0, end node = 1
fn build_graph(maze: &Grid<Maze>, start: (usize, usize), end: (usize, usize)) -> Graph {
    let mut node_list = HashMap::new();
    let mut edge_list = HashMap::new();
    let mut queue = Vec::new();
    let mut visited = HashSet::new();

    let neighbours_start =  maze.get_neighbours(&start);
    neighbours_start.iter().for_each(|(pos, dir)| queue.push((*pos, *dir, 0, hashset!(start.clone()), 0, dir.clone())));
    visited.insert(start.clone());
    node_list.insert(0, Node::new_with_exits(&start, &neighbours_start));

    let neighbours_end =  maze.get_neighbours(&end);
    visited.insert(end.clone());
    node_list.insert(1, Node::new_with_exits(&end, &neighbours_end));

    // Queue: (curr_pos: (usize, usize), curr_dir: Ortho, weight: usize, visited: HashSet<(usize, usize)>, origin: usize, start_dir: Ortho)
    while let Some((curr_pos, curr_dir, weight, mut path, origin, start_dir)) = queue.pop() {
        let neighbours = maze.get_neighbours(&curr_pos);

        if curr_pos == end {
            path.insert(curr_pos.clone());
            let (start_node, end_node, start_direction, end_direction) = normalise_edge(origin, 1, start_dir, curr_dir.flip());
            let edge = Edge {
                start: start_node,
                start_dir: start_direction,
                end: end_node,
                end_dir: end_direction,
                path,
                weight,
            };
            edge_list.insert((start_node, end_node, start_direction, end_direction), edge);
            continue;
        }

        visited.insert(curr_pos);
        if neighbours.len() > 2 { // Node
            let node_idx = get_node_num(&mut node_list, &curr_pos, &neighbours);
            path.insert(curr_pos.clone());
            let (start_node, end_node, start_direction, end_direction) = normalise_edge(origin, node_idx, start_dir, curr_dir.flip());
            let edge = Edge {
                start: start_node,
                start_dir: start_direction,
                end: end_node,
                end_dir: end_direction,
                path,
                weight,
            };
            let key = (start_node, end_node, start_direction, end_direction);
            edge_list.entry(key).or_insert(edge);

            for (next_pos, next_dir) in neighbours {
                if !visited.contains(&next_pos) {
                    queue.push((next_pos, next_dir, 0, hashset!(curr_pos), node_idx, next_dir));
                }
            }
        } else { // Edge, dead ends die here
            for (next_pos, next_dir) in neighbours {
                let new_weight = if curr_dir == next_dir { weight + 1 } else { weight + 1001 };
                path.insert(curr_pos.clone());
                if next_dir != curr_dir.flip() {
                    queue.push((next_pos, next_dir, new_weight, path.clone(), origin, start_dir));
                }
            }
        }
    }

    Graph::new(node_list, edge_list)
}

fn normalise_edge(
    node1: usize,
    node2: usize,
    dir1: Ortho,
    dir2: Ortho,
) -> (usize, usize, Ortho, Ortho) {
    if node1 < node2 {
        (node1, node2, dir1, dir2)
    } else {
        (node2, node1, dir2, dir1)
    }
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Graph {
    let mut entity = Vec::new();
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let mut start = (0, 0);
    let mut end = (0, 0);

    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let item = match ch {
                'E' => { end = (col, row); Maze::End},
                '.' => Maze::Path,
                'S' => { start = (col, row); Maze::Path },
                '#' => Maze::Wall,
                _ => unreachable!(),
            };
            entity.push(item);
        }
    }
    
    let maze = Grid::new(width, height, entity);
    build_graph(&maze, start, end)
}

#[aoc(day16, part1)]
pub fn solve_part1(graph: &Graph) -> usize {
    graph.dijkstra(0, 1, false).unwrap()[0].1
}

#[aoc(day16, part2)]
pub fn solve_part2(graph: &Graph) -> usize {
    let paths = graph.dijkstra(0, 1, true).unwrap();
    let mut all_paths = HashSet::new();

    for (path, _) in paths {
        for window in path.windows(2) {
            for edge in graph.get_edges(window[0], window[1]) {
                all_paths.extend(edge.path);
            }
        }
    }

    all_paths.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    const TEST2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test]
    fn part1_test1() {
        assert_eq!(solve_part1(&input_generator(TEST1)), 7036);
    }

    #[test]
    fn part1_test2() {
        assert_eq!(solve_part1(&input_generator(TEST2)), 11048);
    }

    #[test]
    fn part2_test1() {
        assert_eq!(solve_part2(&input_generator(TEST1)), 45);
    }

    #[test]
    fn part2_test2() {
        assert_eq!(solve_part2(&input_generator(TEST2)), 64);
    }
}