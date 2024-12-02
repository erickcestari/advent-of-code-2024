use std::collections::HashMap;

const INPUT: &str = include_str!("../../files/day_1.txt");

#[allow(dead_code)]
pub fn run() -> String {
    let mut lefts: Vec<i32> = Vec::with_capacity(100);
    let mut rights: HashMap<i32, i32> = HashMap::with_capacity(1000);

    for line in INPUT.lines() {
        let mut values = line.split_ascii_whitespace();
        lefts.push(values.next().unwrap().parse::<i32>().unwrap());
        
        let right = values.next().unwrap().parse::<i32>().unwrap();
        rights.insert(right, rights.get(&right).unwrap_or(&0) + 1);
    }

    let mut sum_similarity : i32 = 0;

    for i in 0..lefts.len() {
        let similarity = rights.get(&lefts[i]).unwrap_or(&0);
        sum_similarity += similarity * lefts[i];
    }

    let result = sum_similarity.to_string();

    result
}
