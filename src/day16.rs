use std::{cmp::Reverse, usize};

use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Maze {
    End,
    Path,
    Start,
    Wall,
}

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    start: usize,
    start_dir: Ortho,
    end: usize,
    end_dir: Ortho,
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

// fn get_node_num(nodes: &HashMap<usize, Node>, pos: (usize, usize)) -> Option<&usize> {
//     for (key, node) in nodes.iter() {
//         if node.pos == pos {
//             return Some(key);
//         }
//     }

//     None
// }

fn get_node_num(node_list: &mut HashMap<usize, Node>, pos: (usize, usize), neighbours: &Vec<((usize, usize), Ortho)>) -> usize {
    if let Some((key, _)) = node_list.iter().find(|&(_, node)| node.pos == pos) {
        return *key;
    }

    let new_node = node_list.len();
    node_list.insert(new_node, Node::new_with_exits(&pos, neighbours));
    new_node
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

    // pub fn stats_heuristic(&self) -> usize {
    //     let w = self.edges.iter().map(|(_, edge)| edge.weight as f64).sum::<f64>();
    //     let mean = w / self.edges.len() as f64;
    //     let sd = self.edges.iter().map(|(_, edge)| (edge.weight as f64 - mean).powi(2)).sum::<f64>().sqrt();

    //     (mean + 2. * sd) as usize
    // }

    pub fn dijkstra(&self, start: usize, end: usize) -> usize {
        let mut distances: HashMap<(usize, Ortho), usize> = HashMap::new();
        let mut previous: HashMap<(usize, Ortho), (usize, Ortho)> = HashMap::new();
        let mut heap: BinaryHeap<Reverse<(usize, usize, Ortho)>> = BinaryHeap::new();

        for &id in self.nodes.keys() {
            for dir in Ortho::iter() {
                distances.insert((id, dir), usize::MAX);
            }
        }

        distances.insert((start, Ortho::East), 0);
        heap.push(Reverse((0, start, Ortho::East)));

        while let Some(Reverse((current_dist, current_node, entry_dir))) = heap.pop() {
            if current_node == end {
                // let mut path = vec![end];
                // let mut current = (end, entry_dir);
                // while let Some(&prev) = previous.get(&current) {
                //     path.push(prev.0);
                //     current = prev;
                // }
                // path.reverse();
                // return Some((path, current_dist));
                return current_dist
            }

            if current_dist > distances[&(current_node, entry_dir)] {
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

                let new_dist = distances[&(current_node, entry_dir)]
                    .saturating_add(1)
                    .saturating_add(turning_cost)
                    .saturating_add(next.weight);

                if new_dist < distances[&(exit_node, exit_dir)] {
                    distances.insert((exit_node, exit_dir), new_dist);
                    previous.insert((exit_node, exit_dir), (current_node, entry_dir));
                    heap.push(Reverse((new_dist, exit_node, exit_dir)));
                }
            }
        }

        0
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
    neighbours_start.iter().for_each(|(pos, dir)| queue.push((*pos, *dir, 0, 0, dir.clone())));
    visited.insert(start.clone());
    node_list.insert(0, Node::new_with_exits(&start, &neighbours_start));

    let neighbours_end =  maze.get_neighbours(&end);
    visited.insert(end.clone());
    node_list.insert(1, Node::new_with_exits(&end, &neighbours_end));

    // Queue: (curr_pos: (usize, usize), curr_dir: Ortho, weight: usize, origin: usize, start_dir: Ortho)
    while let Some((curr_pos, curr_dir, weight, origin, start_dir)) = queue.pop() {
        let neighbours = maze.get_neighbours(&curr_pos);

        if curr_pos == end {
            let (start_node, end_node, start_direction, end_direction) = normalise_edge(origin, 1, start_dir, curr_dir.flip());
            let edge = Edge {
                start: start_node,
                start_dir: start_direction,
                end: end_node,
                end_dir: end_direction,
                weight,
            };
            edge_list.insert((start_node, end_node, start_direction, end_direction), edge);
            continue;
        }

        visited.insert(curr_pos);
        if neighbours.len() > 2 { // Node
            let node_idx = get_node_num(&mut node_list, curr_pos, &neighbours);
            let (start_node, end_node, start_direction, end_direction) = normalise_edge(origin, node_idx, start_dir, curr_dir.flip());
            let edge = Edge {
                start: start_node,
                start_dir: start_direction,
                end: end_node,
                end_dir: end_direction,
                weight,
            };
            let key = (start_node, end_node, start_direction, end_direction);
            edge_list.entry(key).or_insert(edge);

            for (next_pos, next_dir) in neighbours {
                if !visited.contains(&next_pos) {
                    queue.push((next_pos, next_dir, 0, node_idx, next_dir));
                }
            }
        } else { // Edge, dead ends die here
            for (next_pos, next_dir) in neighbours {
                let new_weight = if curr_dir == next_dir { weight + 1 } else { weight + 1001 };
                if next_dir != curr_dir.flip() {
                    queue.push((next_pos, next_dir, new_weight, origin, start_dir));
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
    let graph = build_graph(&maze, start, end);

    // Testing block, remove later
    // let mut blocks = HashMap::new();
    // blocks.insert(Maze::Path, '.');
    // blocks.insert(Maze::Wall, '#');
    // let mut node_list = HashMap::new();
    // let nodes = graph.nodes.clone();
    // for (node_num, node) in nodes {
    //     let node_num_char = if node_num < 10 {
    //         (node_num as u8 + 48) as char
    //     } else {
    //         (node_num as u8 + 55) as char
    //     };
    //     node_list.insert(node.pos, node_num_char);
    // }
    // maze.draw_enum_node_map(&blocks, &node_list);

    graph
}

#[aoc(day16, part1)]
pub fn solve_part1(graph: &Graph) -> usize {
    graph.dijkstra(0, 1)
}

// #[aoc(day16, part2)]
// pub fn solve_part2((maze, start): &(Grid<Maze>, (usize, usize))) -> usize {
//     0
// }

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

    // #[test]
    // fn part2_test() {
    //     assert_eq!(solve_part2(&input_generator(TEST)), 10092);
    // }
}