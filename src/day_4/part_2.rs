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
    if matrix[i][j] != 'A' {
        return 0;
    }

    let mut count: u32 = 0;

    // M    S
    //   A
    // M   S
    if (i + 1 < matrix.len() && i > 0 && j + 1 < matrix[i].len() && j > 0)
        && (matrix[i - 1][j - 1] == 'M' && matrix[i - 1][j + 1] == 'S')
        && (matrix[i + 1][j - 1] == 'M' && matrix[i + 1][j + 1] == 'S')
    {
        count += 1;
    }

    // M    M
    //   A
    // S   S
    if (i + 1 < matrix.len() && i > 0 && j + 1 < matrix[i].len() && j > 0)
        && (matrix[i - 1][j - 1] == 'M' && matrix[i - 1][j + 1] == 'M')
        && (matrix[i + 1][j - 1] == 'S' && matrix[i + 1][j + 1] == 'S')
    {
        count += 1;
    }

    // S    M
    //   A
    // S   M
    if (i + 1 < matrix.len() && i > 0 && j + 1 < matrix[i].len() && j > 0)
        && (matrix[i - 1][j - 1] == 'S' && matrix[i - 1][j + 1] == 'M')
        && (matrix[i + 1][j - 1] == 'S' && matrix[i + 1][j + 1] == 'M')
    {
        count += 1;
    }

    // S    S
    //   A
    // M   M
    if (i + 1 < matrix.len() && i > 0 && j + 1 < matrix[i].len() && j > 0)
        && (matrix[i - 1][j - 1] == 'S' && matrix[i - 1][j + 1] == 'S')
        && (matrix[i + 1][j - 1] == 'M' && matrix[i + 1][j + 1] == 'M')
    {
        count += 1;
    }

    count
}
