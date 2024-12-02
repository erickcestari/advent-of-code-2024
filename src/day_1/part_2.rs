use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

#[allow(dead_code)]
pub fn run() -> Result<String, Error> {
    let file = File::open("./files/day_1.txt")?;

    let reader = BufReader::new(file);

    let mut lefts: Vec<i32> = Vec::new();
    let mut rights: HashMap<i32, i32> = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let numbers = line.split(" ").collect::<Vec<&str>>();
        let left: i32 = numbers[0].parse().unwrap();
        let right: i32 = numbers[3].parse().unwrap();
        lefts.push(left);
        rights.insert(right, rights.get(&right).unwrap_or(&0) + 1);
    }

    let mut sum_similarity : i32 = 0;

    for i in 0..lefts.len() {
        let similarity = rights.get(&lefts[i]).unwrap_or(&0);
        sum_similarity += similarity * lefts[i];
    }
    
    let result = sum_similarity.to_string();

    Ok(result)
}
