const INPUT: &str = include_str!("../../files/day_4.txt");

#[allow(dead_code)]
pub fn run() -> String {
    let mut count: u32 = 0;
    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in INPUT.lines() {
        let values: Vec<char> = line.chars().collect();
        matrix.push(values);
    }

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            count += count_xmas(&matrix, i, j);
        }
    }

    count.to_string()
}

fn count_xmas(matrix: &Vec<Vec<char>>, i: usize, j: usize) -> u32 {
    if matrix[i][j] != 'X' {
        return 0;
    }

    let mut count: u32 = 0;

    // <-
    if j > 2 {
        if matrix[i][j - 1] == 'M' && matrix[i][j - 2] == 'A' && matrix[i][j - 3] == 'S' {
            count += 1;
        }
    }

    // ->
    if j < matrix[i].len() - 3 {
        if matrix[i][j + 1] == 'M' && matrix[i][j + 2] == 'A' && matrix[i][j + 3] == 'S' {
            count += 1;
        }
    }

    // ^
    if i > 2 {
        if matrix[i - 1][j] == 'M' && matrix[i - 2][j] == 'A' && matrix[i - 3][j] == 'S' {
            count += 1;
        }
    }

    // v
    if i < matrix.len() - 3 {
        if matrix[i + 1][j] == 'M' && matrix[i + 2][j] == 'A' && matrix[i + 3][j] == 'S' {
            count += 1;
        }
    }

    // <^
    if j > 2 && i > 2 {
        if matrix[i - 1][j - 1] == 'M' && matrix[i - 2][j - 2] == 'A' && matrix[i - 3][j - 3] == 'S'
        {
            count += 1;
        }
    }

    // <v
    if j > 2 && i < matrix.len() - 3 {
        if matrix[i + 1][j - 1] == 'M' && matrix[i + 2][j - 2] == 'A' && matrix[i + 3][j - 3] == 'S'
        {
            count += 1;
        }
    }

    // >^
    if j < matrix[i].len() - 3 && i > 2 {
        if matrix[i - 1][j + 1] == 'M' && matrix[i - 2][j + 2] == 'A' && matrix[i - 3][j + 3] == 'S'
        {
            count += 1;
        }
    }

    // >v
    if j < matrix[i].len() - 3 && i < matrix.len() - 3 {
        if matrix[i + 1][j + 1] == 'M' && matrix[i + 2][j + 2] == 'A' && matrix[i + 3][j + 3] == 'S'
        {
            count += 1;
        }
    }

    count
}
