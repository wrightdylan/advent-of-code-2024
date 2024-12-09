#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<Option<usize>> {
    let mut map = Vec::new();
    let mut id = 0;
    let mut file = true;

    input
    .chars()
    .for_each(|ch| {
        let num = ch.to_digit(10).unwrap();
        if file {
            for _ in 0..num {
                map.push(Some(id));
            }
            id += 1;
            file = false
        } else {
            for _ in 0..num {
                map.push(None);
            }
            file = true;
        }
    });
    
    map
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &Vec<Option<usize>>) -> usize {
    let mut map = input.clone();
    let mut front_idx = 0;
    let mut back_idx = input.len() - 1;
    let mut checksum = 0;

    while front_idx < back_idx {
        while front_idx < back_idx && map[front_idx].is_some() {
            front_idx += 1;
        }
        while front_idx < back_idx && map[back_idx].is_none() {
            back_idx -= 1;
        }
        if front_idx < back_idx {
            map.swap(front_idx, back_idx);
        }
    }
    
    for (idx, &num) in map.iter().enumerate() {
        if let Some(number) = num {
            checksum += idx * number;
        } else {
            break;
        }
    }
    
    checksum
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &Vec<Option<usize>>) -> usize {
    let mut map = input.clone();
    let mut block_map = Vec::new();
    let mut gap_map = Vec::new();
    let mut checksum = 0;

    let mut block_start = 0;
    let mut block_size: usize = 0;
    let mut gap_start = 0;
    let mut gap_size: usize = 0;
    let mut current_id: Option<usize> = None;

    for (idx, entry) in map.iter().enumerate() {
        match entry {
            Some(id) => {
                    if let Some(current_id) = current_id {
                        if *id != current_id {
                            block_map.push((block_start, block_size));
                            block_start = idx;
                            block_size = 0;
                        }
                    } else {
                        if gap_size > 0 && idx > 0 {
                            gap_map.push((gap_start, gap_size));
                            block_start = idx;
                            gap_size = 0;
                        }
                    }
                    current_id = Some(*id);
                    block_size += 1;
                },
            None     => {
                    if let Some(_) = current_id {
                        block_map.push((block_start, block_size));
                        current_id = None;
                        block_start = idx;
                        block_size = 0;
                    }

                    if gap_size == 0 {
                        gap_start = idx;
                    }
                    gap_size += 1;
                },
        }
    }

    if block_size > 0 {
        block_map.push((block_start, block_size));
    }

    for &(block_start, block_size) in block_map.iter().rev() {
        for (gap_idx, &(gap_start, gap_size)) in gap_map.iter().enumerate() {
            if gap_size >= block_size  && block_start > gap_start {
                for i in 0..block_size {
                    map.swap(gap_start + i, block_start + i);
                }
                gap_map[gap_idx].0 += block_size;
                gap_map[gap_idx].1 -= block_size;
                // This /should/ improve performance
                if gap_map[gap_idx].1 == 0 {
                    gap_map.retain(|&(_, size)| size > 0);
                }
                break;
            }
        }
    }

    for (idx, &num) in map.iter().enumerate() {
        if let Some(number) = num {
            checksum += idx * number;
        }
    }

    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "2333133121414131402";

    #[test]
    fn part1_test() {
        assert_eq!(solve_part1(&input_generator(TEST)), 1928);
    }

    #[test]
    fn part2_test() {
        assert_eq!(solve_part2(&input_generator(TEST)), 2858);
    }
}