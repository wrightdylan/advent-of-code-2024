#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn scan_row(row: &Vec<char>) -> usize {
    let mut window = Vec::new();
    let mut sub_total = 0;
    
    for &ch in row {
        window.push(ch);

        if window.len() > 4 {
            window.remove(0);
        }

        if window == ['X', 'M', 'A', 'S'] || window == ['S', 'A', 'M', 'X'] {
            sub_total += 1;
        }
    }

    sub_total
}

fn scan_submatrix(submatrix: &Vec<Vec<char>>) -> bool {
    if submatrix[1][1] != 'A' {
        return false;
    }

    let mut count = 0;

    let windows = vec![
        vec![submatrix[0][0], submatrix[2][2]],
        vec![submatrix[2][0], submatrix[0][2]],
    ];

    for window in windows {
        if window == ['M', 'S'] || window == ['S', 'M'] {
            count += 1;
        }
    }

    if count == 2 {
        true
    } else {
        false
    }
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &Vec<Vec<char>>) -> usize {
    let mut total = 0;

    // Search horizontal
    for row in input {
        total += scan_row(&row);
    }

    // Search Vertical
    // Simply transpose the matrix
    let num_rows = input.len();
    let num_cols = input[0].len();
    let mut new_grid = vec![vec![' '; num_rows]; num_cols];
    for i in 0..num_rows {
        for j in 0..num_cols {
            new_grid[j][i] = input[i][j]
        }
    }
    for row in new_grid {
        total += scan_row(&row);
    }

    // Search diagonals
    let mut new_grid = Vec::new();
    for i in 0..num_rows + num_cols - 1 {
        let mut diagonal = Vec::new();
        let mut row = i.min(num_rows - 1);
        let mut col = if i < num_rows { 0 } else { i - num_rows + 1};

        while col < num_cols {
            diagonal.push(input[row][col]);
            if row == 0 { break; }
            row -= 1;
            col += 1;
        }
        if diagonal.len() > 3 {
            new_grid.push(diagonal);
        }
    }
    for i in 0..num_rows + num_cols - 1 {
        let mut diagonal = Vec::new();
        let mut row = if i < num_cols { 0 } else { i - num_cols + 1};
        let mut col = if i < num_cols { num_cols - 1 - i} else { 0 };

        while row < num_rows && col < num_cols {
            diagonal.push(input[row][col]);
            row += 1;
            col += 1;
        }

        if diagonal.len() > 3 {
            new_grid.push(diagonal);
        }
    }
    for row in new_grid {
        total += scan_row(&row,);
    }

    total
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &Vec<Vec<char>>) -> usize {
    let mut total = 0;
    for i in 0..input.len() - 2 {
        for j in 0..input[0].len() - 2 {
            let mut submatrix = Vec::new();

            for k in i..i + 3 {
                let mut row = Vec::new();

                for l in j..j + 3 {
                    row.push(input[k][l]);
                }
                submatrix.push(row);
            }

            if scan_submatrix(&submatrix) {
                total += 1;
            }
        }
    }
    
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn part1_test() {
        assert_eq!(solve_part1(&input_generator(TEST)), 18);
    }

    #[test]
    fn part2_test() {
        assert_eq!(solve_part2(&input_generator(TEST)), 9);
    }
}