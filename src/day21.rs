use crate::prelude::*;

// Numpad
// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+


// Dirpad
//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+

pub struct Keypad {
    paths: HashMap<(char, char), String>,
}

impl Keypad {
    pub fn new(layout: Vec<(char, (usize, usize))>, invalid: (usize, usize)) -> Self {
        let mut paths = HashMap::new();

        for (c1, (x1, y1)) in &layout {
            for (c2, (x2, y2)) in &layout {
                let mut path = String::new();

                path += &"<".repeat(x1.saturating_sub(*x2));
                path += &"v".repeat(y2.saturating_sub(*y1));
                path += &"^".repeat(y1.saturating_sub(*y2));
                path += &">".repeat(x2.saturating_sub(*x1));
                
                if invalid == (*x1, *y2) || invalid == (*x2, *y1) {
                    path = path.chars().rev().collect();
                }

                path += &"A";

                paths.insert((c1.clone(), c2.clone()), path);
            }
        }

        Self { paths }
    }

    pub fn enter_code(
        &self,
        dirpad: &Keypad,
        cache: &mut HashMap<(String, usize), usize>,
        sequence: &String,
        depth: usize,
    ) -> usize {
        let mut prev = 'A';
        let mut complexity = 0;
        for ch in sequence.chars() {
            if depth < 5 {
                complexity += dirpad._enter_code( &self.get_path(prev, ch), depth);
            } else {
                complexity += dirpad._enter_code_cached(&self.get_path(prev, ch), depth, cache);
            }
            prev = ch;
        }
        return complexity
    }

    // Enter dirpad code
    fn _enter_code(
        &self,
        sequence: &String,
        depth: usize,
    ) -> usize {
        if depth == 0 {
            return sequence.len()
        }
        
        let mut prev = 'A';
        let mut length = 0;
        for ch in sequence.chars() {
            length += self._enter_code(&self.get_path(prev, ch), depth - 1);
            prev = ch;
        }
        return length
    }

    // Enter dirpad code with caching
    fn _enter_code_cached(
        &self,
        sequence: &String,
        depth: usize,
        cache: &mut HashMap<(String, usize), usize>,
    ) -> usize {
        let cache_key = (sequence.clone(), depth);
        if let Some(&cached_result) = cache.get(&cache_key) {
            return cached_result;
        }

        let result = if depth == 0 {
            sequence.len()
        } else {
            let mut prev = 'A';
            let mut length = 0;
            for ch in sequence.chars() {
                length += self._enter_code_cached(&self.get_path(prev, ch), depth - 1, cache);
                prev = ch;
            }
            length
        };

        cache.insert(cache_key, result);
        result
    }

    pub fn get_path(&self, from: char, to: char) -> String {
        self.paths.get(&(from, to)).unwrap().clone()
    }
}

#[aoc_generator(day21)]
pub fn input_generator(input: &str) -> (Keypad, Keypad, Vec<(usize, String)>) {
    let numpad = vec![
        ('7', (0, 0)), ('8', (1, 0)), ('9', (2, 0)),
        ('4', (0, 1)), ('5', (1, 1)), ('6', (2, 1)),
        ('1', (0, 2)), ('2', (1, 2)), ('3', (2, 2)),
                       ('0', (1, 3)), ('A', (2, 3)),
    ];
    let dirpad = vec![
                       ('^', (1, 0)), ('A', (2, 0)),
        ('<', (0, 1)), ('v', (1, 1)), ('>', (2, 1)),
    ];

    (
        Keypad::new(numpad, (0, 3)),
        Keypad::new(dirpad, (0, 0)),
        input
            .lines()
            .map(|line| {
                (
                    line[..(line.len()-1)].parse::<usize>().unwrap(),
                    line.to_string()
                )
            }).collect()
    )
}

// Is there some optmisation other than caching?
// Code   : 0                 |2           |9                   |A
// Depth 0: <         A       |^       A   |^^       >      A   |vvv        A
// Depth 1: v  << A   >> ^  A |<   A   > A |<   AA   v  > A ^ A |<   v AAA  ^ >  A
// Depth 2: <vA<AA>>^AvAA<^A>A|v<<A>>^AvA^A|v<<A>>^AA<vA>A^A<A>A|v<<A>A^>AAA<Av>A^A


#[aoc(day21, part1)]
pub fn solve_part1(
    (numpad, dirpad, codes): &(Keypad, Keypad, Vec<(usize, String)>)
) -> usize {
    let mut cache = HashMap::new();
    codes
        .iter()
        .map(|(num, code)| numpad.enter_code(dirpad, &mut cache, code, 2) * num).sum()
}

#[aoc(day21, part2)]
pub fn solve_part2(
    (numpad, dirpad, codes): &(Keypad, Keypad, Vec<(usize, String)>)
) -> usize {
    let mut cache = HashMap::new();
    codes
        .iter()
        .map(|(num, code)| numpad.enter_code(dirpad, &mut cache, code, 25) * num).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "029A
980A
179A
456A
379A";

    #[test]
    fn part1_test() {
        assert_eq!(solve_part1(&input_generator(TEST)), 126384);
    }
}