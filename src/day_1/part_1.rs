use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[allow(dead_code)]
pub fn run() -> io::Result<()> {
    let file = File::open("./files/day_1.txt")?;

    let reader = BufReader::new(file);

    let mut lefts: Vec<i32> = Vec::new();
    let mut rights: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let numbers = line.split(" ").collect::<Vec<&str>>();
        let left: i32 = numbers[0].parse().unwrap();
        let right: i32 = numbers[3].parse().unwrap();
        lefts.push(left);
        rights.push(right);
    }

    lefts.sort();
    rights.sort();

    let mut sum_diff: i32 = 0;

    for i in 0..lefts.len() {
        let diff = lefts[i] - rights[i];
        if diff < 0 {
            sum_diff += diff * -1;
            continue;
        }
        sum_diff += diff;
    }

    println!("{}", sum_diff);

    Ok(())
}
