use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../../files/day_5.txt");

#[allow(dead_code)]
pub fn run() -> String {
    let mut sum_middle: u32 = 0;
    let mut pages_order_rules: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut parsing_rules = true;

    for line in INPUT.lines() {
        if line.is_empty() {
            parsing_rules = false;
            continue;
        }

        if parsing_rules {
            let page_order: Vec<&str> = line.split('|').collect::<Vec<&str>>();
            pages_order_rules
                .entry(page_order[0])
                .and_modify(|vec| vec.push(page_order[1]))
                .or_insert(vec![page_order[1]]);
            continue;
        }

        let pages = line.split(',').collect::<Vec<&str>>();
        let mut is_valid_order = true;
        let mut pages_passed: HashSet<&str> = HashSet::new();
        for i in 0..pages.len() {
            let page = pages[i];
            pages_passed.insert(page);
            let rules = pages_order_rules.get(page);
            if rules.is_none() {
                continue;
            }

            let rules = rules.unwrap();
            for j in 0..rules.len() {
                if pages_passed.contains(rules[j]) {
                    is_valid_order = false;
                    break;
                }
            }
        }

        if is_valid_order {
            sum_middle += pages[pages.len() / 2].parse::<u32>().unwrap();
        }
    }

    sum_middle.to_string()
}
