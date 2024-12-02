use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[allow(dead_code)]
pub fn run() -> io::Result<()> {
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
        if is_safe_with_dampener(numbers.clone()) {
            safes += 1;
        }
    }

    println!("{:?}", safes);
    Ok(())
}

fn is_safe_with_dampener(numbers: Vec<i32>) -> bool {
    fn is_safe(numbers: &[i32]) -> bool {
        let is_increasing = numbers[0] < numbers[1];
        for i in 0..numbers.len() - 1 {
            let diff = numbers[i + 1] - numbers[i];
            if diff.abs() < 1 || diff.abs() > 3 || (is_increasing && diff < 0) || (!is_increasing && diff > 0) {
                return false;
            }
        }
        true
    }

    if is_safe(&numbers) {
        return true;
    }

    for i in 0..numbers.len() {
        let mut modified_numbers = numbers.clone();
        modified_numbers.remove(i);
        if is_safe(&modified_numbers) {
            return true;
        }
    }

    false
}
