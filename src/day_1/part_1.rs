const INPUT: &str = include_str!("../../files/day_1.txt");

#[allow(dead_code)]
pub fn run() -> String {
    let mut lefts: Vec<i32> = Vec::with_capacity(1000);
    let mut rights: Vec<i32> = Vec::with_capacity(1000);

    for line in INPUT.lines() {
        let mut values = line.split_ascii_whitespace();
        lefts.push(values.next().unwrap().parse::<i32>().unwrap());
        rights.push(values.next().unwrap().parse::<i32>().unwrap());
    }

    lefts.sort_unstable();
    rights.sort_unstable();

    let mut sum_diff: i32 = 0;

    for i in 0..lefts.len() {
        let diff = lefts[i] - rights[i];
        sum_diff += diff.abs();
    }

    let result = sum_diff.to_string();

    result
}
