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
        let (row, col, _) = guard;
        visited.insert((row, col));

        guard = next_guard_position(guard, &matrix);

        if guard.0 >= matrix.len() - 1
            || guard.1 >= matrix[guard.0].len() - 1
            || guard.0 == 0
            || guard.1 == 0
        {
            visited.insert((guard.0, guard.1));
            break;
        }
    }

    visited.len().to_string()
}

fn next_guard_position(guard: (usize, usize, char), matrix: &[Vec<char>]) -> (usize, usize, char) {
    let (row, col, dir) = guard;
    match dir {
        '^' if matrix[row - 1][col] != '#' => (row - 1, col, '^'),
        '>' if matrix[row][col + 1] != '#' => (row, col + 1, '>'),
        'v' if matrix[row + 1][col] != '#' => (row + 1, col, 'v'),
        '<' if matrix[row][col - 1] != '#' => (row, col - 1, '<'),
        '^' => (row, col, '>'),
        '>' => (row, col, 'v'),
        'v' => (row, col, '<'),
        '<' => (row, col, '^'),
        _ => unreachable!(),
    }
}
