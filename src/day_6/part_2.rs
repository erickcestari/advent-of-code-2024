use std::collections::HashSet;

const INPUT: &str = include_str!("../../files/day_6.txt");

pub fn run() -> String {
    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut guard_start = (0, 0, '^');

    for (row, line) in INPUT.lines().enumerate() {
        matrix.push(line.chars().collect());
        for (col, c) in line.chars().enumerate() {
            if c == '^' || c == '>' || c == 'v' || c == '<' {
                guard_start = (row, col, c);
            }
        }
    }

    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut count = 0;

    for y in 0..rows {
        for x in 0..cols {
            if (y, x) == (guard_start.0, guard_start.1) {
                continue;
            }
            if matrix[y][x] != '.' {
                continue;
            }

            let mut new_matrix = matrix.clone();
            new_matrix[y][x] = '#';

            if guard_gets_stuck(guard_start, &new_matrix) {
                count += 1;
            }
        }
    }

    count.to_string()
}

fn guard_gets_stuck(guard_start: (usize, usize, char), matrix: &[Vec<char>]) -> bool {
    let mut visited = HashSet::new();
    let mut guard = guard_start;

    loop {
        if !visited.insert(guard) {
            return true;
        }

        let (row, col, dir) = guard;
        let (next_row, next_col, next_dir) = match dir {
            '^' => {
                if row > 0 && matrix[row - 1][col] != '#' {
                    (row - 1, col, '^')
                } else {
                    (row, col, turn_right(dir))
                }
            }
            '>' => {
                if col + 1 < matrix[0].len() && matrix[row][col + 1] != '#' {
                    (row, col + 1, '>')
                } else {
                    (row, col, turn_right(dir))
                }
            }
            'v' => {
                if row + 1 < matrix.len() && matrix[row + 1][col] != '#' {
                    (row + 1, col, 'v')
                } else {
                    (row, col, turn_right(dir))
                }
            }
            '<' => {
                if col > 0 && matrix[row][col - 1] != '#' {
                    (row, col - 1, '<')
                } else {
                    (row, col, turn_right(dir))
                }
            }
            _ => unreachable!(),
        };

        if next_row == 0
            || next_row + 1 == matrix.len()
            || next_col == 0
            || next_col + 1 == matrix[0].len()
        {
            return false;
        }

        guard = (next_row, next_col, next_dir);
    }
}

fn turn_right(dir: char) -> char {
    match dir {
        '^' => '>',
        '>' => 'v',
        'v' => '<',
        '<' => '^',
        _ => unreachable!(),
    }
}
