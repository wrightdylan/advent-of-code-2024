use std::collections::VecDeque;

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input.split_whitespace().map(|num| num.parse().unwrap()).collect()
}

// #[derive(Debug)]
// struct Node {
//     value: Option<usize>,
//     left:  Option<Box<Node>>,
//     right: Option<Box<Node>>,
// }

// impl Node {
//     fn new_leaf(value: usize) -> Self {
//         Self {
//             value: Some(value),
//             left:  None,
//             right: None,
//         }
//     }

//     fn new_branch() -> Self {
//         Node {
//             value: None,
//             left:  None,
//             right: None,
//         }
//     }

//     fn is_leaf(&self) -> bool {
//         self.left.is_none() && self.right.is_none()
//     }
// }

// #[derive(Debug)]
// struct BTree {
//     root: Option<Box<Node>>,
// }

// impl BTree {
//     fn new() -> Self {
//         Self { root: None }
//     }

//     fn from(values: Vec<usize>) -> Self {
//         let mut tree = BTree::new();
//         if values.is_empty() {
//             return tree;
//         }

//         // Build empty tree structure
//         tree.root = Some(Box::new(Self::build_empty(values.len())));

//         // Populate leaves
//         let mut values_iter = values.into_iter();
//         if let Some(root) = &mut tree.root {
//             Self::populate_leaves(root, &mut values_iter);
//         }

//         // Transform the tree according to rules
//         // if let Some(root) = &mut tree.root {
//         //     Self::transform_node(root);
//         // }

//         tree
//     }

//     fn build_empty(leaf_count: usize) -> Node {
//         if leaf_count <= 1 {
//             return Node::new_branch();
//         }

//         let mut node = Node::new_branch();
//         let left_count = leaf_count / 2;
//         let right_count = leaf_count - left_count;

//         node.left = Some(Box::new(Self::build_empty(left_count)));
//         node.right = Some(Box::new(Self::build_empty(right_count)));

//         node
//     }

//     fn populate_leaves<I>(node: &mut Box<Node>, values: &mut I)
//     where
//         I: Iterator<Item = usize>,
//     {
//         if node.is_leaf() {
//             node.value = values.next();
//             return;
//         }

//         if let Some(left) = &mut node.left {
//             Self::populate_leaves(left, values);
//         }
//         if let Some(right) = &mut node.right {
//             Self::populate_leaves(right, values);
//         }
//     }

//     // Transform the tree according to rules
//     fn transform_leaves(&mut self) {
//         if let Some(root) = &mut self.root {
//             Self::transform_node(root);
//         }
//     }

//     fn transform_node(node: &mut Box<Node>) {
//         if let Some(left) = &mut node.left {
//             Self::transform_node(left);
//         }
//         if let Some(right) = &mut node.right {
//             Self::transform_node(right);
//         }

//         if node.is_leaf() {
//             if let Some(value) = node.value {
//                 if value == 0 {
//                     node.value = Some(1);
//                 } else {
//                     let digit_count = (value as f64).log10() as u32 + 1;
//                     if digit_count % 2 == 0 {
//                         let divisor = 10_usize.pow(digit_count / 2);
//                         let first = value / divisor;
//                         let second = value % divisor;

//                         node.value = None;
//                         node.left  = Some(Box::new(Node::new_leaf(first)));
//                         node.right = Some(Box::new(Node::new_leaf(second)));
//                     } else {
//                         node.value = Some(value * 2024);
//                     }
//                 }
//             }
//         }
//     }

//     fn get_leaves(&self) -> Vec<&usize> {
//         let mut leaves = Vec::new();
//         if let Some(root) = &self.root {
//             Self::collect_leaves(root, &mut leaves);
//         }
//         leaves
//     }

//     fn collect_leaves<'a>(node: &'a Box<Node>, leaves: &mut Vec<&'a usize>) {
//         if node.is_leaf() {
//             if let Some(value) = &node.value {
//                 leaves.push(value);
//             }
//             return;
//         }

//         if let Some(left) = &node.left {
//             Self::collect_leaves(left, leaves);
//         }
//         if let Some(right) = &node.right {
//             Self::collect_leaves(right, leaves);
//         }
//     }

//     fn count_leaves(&self) -> usize {
//         if let Some(root) = &self.root {
//             Self::_count_leaves(root)
//         } else {
//             0
//         }
//     }

//     fn _count_leaves(node: &Box<Node>) -> usize {
//         if node.is_leaf() {
//             if node.value.is_some() { 1 } else { 0 }
//         } else {
//             let left_count = node.left.as_ref().map_or(0, Self::_count_leaves);
//             let right_count = node.right.as_ref().map_or(0, Self::_count_leaves);
//             left_count + right_count
//         }
//     }
// }

#[aoc(day11, part1)]
pub fn solve_part1(input: &Vec<usize>) -> usize {
    // let mut stones = BTree::from(input.clone());
    let mut stones = VecDeque::from_iter(input.clone());
    // println!("{:?}", stones);
    // println!("{}", stones.count_leaves());
    // println!("{:?}", stones.get_leaves());

    for _ in 0..25 {
        for _ in 0..stones.len() {
            let value = stones.pop_front().unwrap();
            if value == 0 {
                stones.push_back(1);
            } else {
                let digit_count = (value as f64).log10() as u32 + 1;
                if digit_count % 2 == 0 {
                    let divisor = 10_usize.pow(digit_count / 2);
                    stones.push_back(value/divisor);
                    stones.push_back(value % divisor);
                } else {
                    stones.push_back(value * 2024);
                }
            }
        }
        // stones.transform_leaves();
        // println!("After {} blink(s)", i + 1);
        // println!("{:?}", stones);
        // println!("{}", stones.count_leaves());
        // println!("{:?}", stones.get_leaves());
    }
    
    stones.len()
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &Vec<usize>) -> usize {
    let mut stones = VecDeque::from_iter(input.clone());

    for _ in 0..75 {
        for _ in 0..stones.len() {
            let value = stones.pop_front().unwrap();
            if value == 0 {
                stones.push_back(1);
            } else {
                let digit_count = (value as f64).log10() as u32 + 1;
                if digit_count % 2 == 0 {
                    let divisor = 10_usize.pow(digit_count / 2);
                    stones.push_back(value/divisor);
                    stones.push_back(value % divisor);
                } else {
                    stones.push_back(value * 2024);
                }
            }
        }
    }

    stones.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "125 17";

    #[test]
    fn part1_test() {
        assert_eq!(solve_part1(&input_generator(TEST)), 55312);
    }
}