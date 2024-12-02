use std::fs::File;
use std::io::{BufRead, BufReader, Error};

#[allow(dead_code)]
pub fn run() -> Result<String, Error> {
    let file = File::open("./files/day_2.txt")?;

    let reader = BufReader::new(file);

    let mut safes: i32 = 0;
    
    for line in reader.lines() {
        let mut numbers: Vec<i32> = Vec::new();
        let line = line?;
        let numbers_string = line.split(" ").collect::<Vec<&str>>();
        for number_string in numbers_string {
            let number: i32 = number_string.parse().unwrap();
            numbers.push(number);
        }
        if is_safe(numbers.clone()) {
            safes += 1;
        }
    }

    let result = safes.to_string();
    
    Ok(result)
}

fn is_safe(numbers: Vec<i32>) -> bool {
    let is_increasing = numbers[0] < numbers[1];
    
    for i in 0..numbers.len() - 1 {
        let diff = numbers[i + 1] - numbers[i];
        if diff.abs() < 1 || diff.abs() > 3 || (is_increasing && diff < 0) || (!is_increasing && diff > 0) {
            return false;
        }
    }

    true
}
