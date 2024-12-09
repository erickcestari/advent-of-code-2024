use std::collections::HashSet;

const INPUT: &str = include_str!("../../files/day_6.txt");

#[allow(dead_code)]
pub fn run() -> String {
    let mut guard = (0, 0, '^');
    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    for (row, line) in INPUT.lines().enumerate() {
        matrix.push(line.chars().collect());
        for (col, c) in line.chars().enumerate() {
            if c == '^' {
                guard = (row, col, '^');
                visited.insert((row as usize, col as usize));
            }
        }
    }

    loop {
        let (row, col, dir) = guard;

        match dir {
            '^' => {
                if row == 0 || matrix[row - 1][col] == '#' {
                    guard.2 = '>';
                } else {
                    guard = (row - 1, col, dir);
                    visited.insert((row - 1, col));
                }
            }
            '>' => {
                if col == matrix[row].len() - 1 || matrix[row][col + 1] == '#' {
                    guard.2 = 'v';
                } else {
                    guard = (row, col + 1, dir);
                    visited.insert((row, col + 1));
                }
            }
            'v' => {
                if row == matrix.len() - 1 || matrix[row + 1][col] == '#' {
                    guard.2 = '<';
                } else {
                    guard = (row + 1, col, dir);
                    visited.insert((row + 1, col));
                }
            }
            '<' => {
                if col == 0 || matrix[row][col - 1] == '#' {
                    guard.2 = '^';
                } else {
                    guard = (row, col - 1, dir);
                    visited.insert((row, col - 1));
                }
            }
            _ => unreachable!(),
        }

        if guard.0 >= matrix.len() - 1
            || guard.1 >= matrix[guard.0].len() - 1
            || guard.0 == 0
            || guard.1 == 0
        {
            break;
        }
    }

    visited.len().to_string()
}
