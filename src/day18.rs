use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mem {
    Safe,
    Corrupted,
    Visited,
}

#[derive(Clone, Eq, PartialEq)]
struct Node {
    pos: (usize, usize),
    g: i32,
    f: i32,
    par: Option<(usize, usize)>,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f.cmp(&self.f)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn pathfinder(
    grid: &Grid<Mem>,
    start: (usize, usize),
    target: (usize, usize)
) -> Option<Vec<(usize, usize)>> {
    let mut open_set = BinaryHeap::new();
    let mut closed_set = HashSet::new();
    let mut from = HashMap::new();

    open_set.push(Node {
        pos: start,
        g: 0,
        f: manhattan_distance(start, target),
        par: None,
    });

    while let Some(current) = open_set.pop() {
        if current.pos == target {
            return Some(reconstruct_path(&from, start, target));
        }

        if closed_set.contains(&current.pos) {
            continue;
        }

        closed_set.insert(current.pos);

        for n_pos in grid.neighbours(&current.pos) {
            if closed_set.contains(&n_pos) || matches!(grid[(n_pos.0, n_pos.1)], Mem::Corrupted) {
                continue;
            }

            // Increase cost per step as +1 was giving precision issues
            let tentative_g = current.g + 2;

            let h = manhattan_distance(n_pos, target);
            let f = tentative_g + h;

            let neighbor = Node {
                pos: n_pos,
                g: tentative_g,
                f,
                par: Some(current.pos),
            };

            from.insert(n_pos, current.pos);
            open_set.push(neighbor);
        }
    }

    None
}

fn manhattan_distance(start: (usize, usize), target: (usize, usize)) -> i32 {
    (start.0 as i32 - target.0 as i32).abs() + (start.1 as i32 - target.1 as i32).abs()
}

fn reconstruct_path(
    came_from: &HashMap<(usize, usize), (usize, usize)>,
    start: (usize, usize),
    target: (usize, usize)
) -> Vec<(usize, usize)> {
    let mut path = vec![target];
    let mut current = target;

    while current != start {
        match came_from.get(&current) {
            Some(&pos) => {
                path.push(pos);
                current = pos;
            }
            None => break,
        }
    }

    // path.reverse();
    path
}

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(',').unwrap();
            (l.parse().unwrap(), r.parse().unwrap())
        }).collect()
}

#[aoc(day18, part1)]
pub fn solve_part1(input: &Vec<(usize, usize)>) -> usize {
    // let (width, height) = (7, 7);
    // let corrupted = &input[..12].to_vec();
    let (width, height) = (71, 71);
    let corrupted = &input[..1024].to_vec();

    let mut mem = Grid::new_fill(width, height, Mem::Safe);
    mem.place_at(corrupted, Mem::Corrupted);

    let path = pathfinder(&mem, (0, 0), (70, 70)).unwrap();

    // Uncomment to see a pretty map
    // let mut char_map = HashMap::new();
    // char_map.insert(Mem::Safe, '.');
    // char_map.insert(Mem::Corrupted, '#');
    // char_map.insert(Mem::Visited, 'O');
    // mem.place_at(&path, Mem::Visited);
    // mem.draw_enum_map(&char_map);

    path.len() - 1
}

#[aoc(day18, part2)]
pub fn solve_part2(input: &Vec<(usize, usize)>) -> String {
    // let (width, height) = (7, 7);
    // let corrupted = &input[..12].to_vec();
    // let remaining = &input[12..].to_vec();
    let (width, height) = (71, 71);
    let corrupted = &input[..1024].to_vec();
    let remaining = &input[1024..].to_vec();

    let mut mem = Grid::new_fill(width, height, Mem::Safe);
    mem.place_at(corrupted, Mem::Corrupted);

    let mut path = pathfinder(&mem, (0, 0), (70, 70)).unwrap();
    // path_set may seem redundant, but it consistently runs milliseconds faster using it
    let mut path_set: HashSet<_> = path.iter().collect();
    
    let mut first = (0, 0);
    for pos in remaining {
        mem.place_at(&vec![*pos], Mem::Corrupted);
        // Dropped from 2s to 40ms!!
        if path_set.contains(pos) {
            if let Some(new_path) = pathfinder(&mem, (0, 0), (70, 70)) {
                path = new_path;
                path_set = path.iter().collect();
            } else {
                first = *pos;
                break;
            }
        }
    }

    format!("{},{}", first.0, first.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn part1_test() {
        assert_eq!(solve_part1(&input_generator(TEST)), 22);
    }

    #[test]
    fn part2_test() {
        assert_eq!(solve_part2(&input_generator(TEST)), "6,1");
    }
}