const INPUT: &str = include_str!("../../files/day_2.txt");

#[allow(dead_code)]
pub fn run() -> String {
    let mut safes: i32 = 0;
    
    for line in INPUT.lines() {
        let mut numbers: Vec<i32> = Vec::with_capacity(100);
        let values = line.split_ascii_whitespace();
        for value in values {
            let number: i32 = value.parse().unwrap();
            numbers.push(number);
        }
        if is_safe(&numbers) {
            safes += 1;
        }
    }

    let result = safes.to_string();

    result
}

fn is_safe(numbers: &Vec<i32>) -> bool {
    let is_increasing = numbers[0] < numbers[1];
    
    for i in 0..numbers.len() - 1 {
        let diff = numbers[i + 1] - numbers[i];
        if diff.abs() < 1 || diff.abs() > 3 || (is_increasing && diff < 0) || (!is_increasing && diff > 0) {
            return false;
        }
    }

    true
}
