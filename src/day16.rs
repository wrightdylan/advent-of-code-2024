use std::usize;

use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Maze {
    End,
    Path,
    Start,
    Wall,
}

#[derive(Debug)]
pub struct Edge {
    start: usize,
    start_dir: Ortho,
    end: usize,
    end_dir: Ortho,
    weight: usize,
}

#[derive(Debug)]
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

fn get_node_num(nodes: &HashMap<usize, Node>, pos: (usize, usize)) -> Option<&usize> {
    for (key, node) in nodes.iter() {
        if node.pos == pos {
            return Some(key);
        }
    }
    None
}

pub struct Vertex {
    f: usize,
    g: usize,
    h: usize,
    inputs: Vec<usize>,
    from: Option<usize>,
    closed: bool,
}

impl Vertex {
    fn default(h: usize) -> Self {
        Vertex { f: usize::MAX, g: usize::MAX, h: 0, inputs: vec![], from: None, closed: false }
    }
}

pub struct Graph {
    nodes: HashMap<usize, Node>,
    edges: HashMap<(usize, usize), Edge>,
}

impl Graph {
    pub fn new(nodes: HashMap<usize, Node>, edges: HashMap<(usize, usize), Edge>) -> Self {
        Self { nodes, edges }
    }

    pub fn stats_heuristic(&self) -> usize {
        let w = self.edges.iter().map(|(_, edge)| edge.weight as f64).sum::<f64>();
        let mean = w / self.edges.len() as f64;
        let sd = self.edges.iter().map(|(_, edge)| (edge.weight as f64 - mean).powi(2)).sum::<f64>().sqrt();

        (mean + 2. * sd) as usize
    }

    pub fn dijkstra(&self, start: usize, end: usize) {
        // let mut heap = BinaryHeap::new();
        let h = self.stats_heuristic();
        

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

fn build_graph(maze: Grid<Maze>, start: (usize, usize), end: (usize, usize)) -> Graph {
    let mut node_list = HashMap::new();
    let mut edge_list = HashMap::new();
    let mut queue = Vec::new();
    let mut visited = HashSet::new();
    let mut segment = Vec::new();
    let mut node_num: usize = 2;
    let mut _last_origin = 0;

    let neighbours_start =  maze.get_neighbours(&start);
    neighbours_start.iter().for_each(|(pos, dir)| queue.push((*pos, *dir, 0)));
    visited.insert(start.clone());
    node_list.insert(0, Node::new_with_exits(&start, &neighbours_start));

    let neighbours_end =  maze.get_neighbours(&end);
    visited.insert(end.clone());
    node_list.insert(1, Node::new_with_exits(&end, &neighbours_end));

    while let Some((pos, dir, origin)) = queue.pop() {
        let mut neighbours = maze.get_neighbours(&pos);
        _last_origin = origin;

        if pos == end {
            let weight = segment.iter().map(|(_, _, w)| w).sum();
            let edge = Edge { start: origin, start_dir: segment.get(0).unwrap().1, end: 1, end_dir: dir.flip(), weight};
            edge_list.insert((origin, 1), edge);
            segment.clear();
            neighbours.clear();
        } else if neighbours.len() > 2 && !visited.contains(&pos) {
            node_list.insert(node_num, Node::new_with_exits(&pos, &neighbours));
            visited.insert(pos.clone());
            if segment.len() > 0 {
                let weight = segment.iter().map(|(_, _, w)| w).sum();
                let edge = Edge { start: origin, start_dir: segment.get(0).unwrap().1, end: node_num, end_dir: dir.flip(), weight};
                edge_list.insert((origin, node_num), edge);
            }
            segment.clear();
            _last_origin = node_num;
            node_num += 1;
        } else if neighbours.len() > 2 && visited.contains(&pos) {
            let end_num = get_node_num(&node_list, pos).unwrap();
            if segment.len() > 0 {
                let weight = segment.iter().map(|(_, _, w)| w).sum();
                let edge = Edge { start: origin, start_dir: segment.get(0).unwrap().1, end: *end_num, end_dir: dir.flip(), weight};
                edge_list.insert((origin, *end_num), edge);
            }
            segment.clear();
        } else if neighbours.len() == 1 {
            // Dead end
            visited.insert(pos.clone());
            segment.clear();
        } else {
            let mut w = 1;
            if segment.len() > 0 && (dir == segment.last().unwrap().1.turn_left() || dir == segment.last().unwrap().1.turn_right()) {
                w += 1000;
            }
            if !visited.contains(&pos) {
                segment.push((pos.clone(), dir.clone(), w));
                visited.insert(pos.clone());
            }
        }

        for (pos, dir) in neighbours {
            if !visited.contains(&pos) {
                queue.push((pos, dir, _last_origin));
            } else {
                if (maze.get_neighbours(&pos).len() > 2 && segment.len() > 1) || pos == end  {
                    queue.push((pos, dir, _last_origin));
                }
            }
        }
    }

    Graph::new(node_list, edge_list)
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
    
    build_graph(maze, start, end)
}

#[aoc(day16, part1)]
pub fn solve_part1(graph: &Graph) -> usize {

    for node in graph.nodes.iter() {
        println!("{:?}", node);
    }
    for edge in graph.edges.iter() {
        println!("{:?}", edge);
    }

    let edges_with_2: Vec<&(usize, usize)> = graph.edges
        .keys()
        .filter(|(start, end)| *start == 1 || *end == 1)
        .collect();

    println!("{:?}", edges_with_2);
    println!("{:?}", graph.edges.get(&(8, 1)));
    0
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

    // #[test]
    // fn part1_test2() {
    //     assert_eq!(solve_part1(&input_generator(TEST2)), 11048);
    // }

    // #[test]
    // fn part2_test() {
    //     assert_eq!(solve_part2(&input_generator(TEST)), 10092);
    // }
}