use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../../files/day_5.txt");

#[allow(dead_code)]
pub fn run() -> String {
    let mut rules = vec![];
    let mut updates: Vec<Vec<&str>> = Vec::new();

    let mut parsing_rules = true;
    for line in INPUT.lines() {
        if line.is_empty() {
            parsing_rules = false;
            continue;
        }

        if parsing_rules {
            let page_order: Vec<&str> = line.split('|').collect::<Vec<&str>>();
            rules.push((page_order[0], page_order[1]));
            continue;
        }

        let pages = line.split(',').collect::<Vec<&str>>();
        updates.push(pages);
    }

    let mut graph = HashMap::new();
    for (x, y) in rules {
        graph.entry(x).or_insert_with(HashSet::new).insert(y);
    }

    let mut middle_sum = 0;

    for update in updates {
        if is_correct_order(&update, &graph) {
            let middle = update[update.len() / 2];
            middle_sum += middle.parse::<u32>().unwrap();
            continue;
        }
    }

    middle_sum.to_string()
}

fn is_correct_order(update: &[&str], graph: &HashMap<&str, HashSet<&str>>) -> bool {
    for (i, &x) in update.iter().enumerate() {
        for &y in &update[i + 1..] {
            if let Some(dependencies) = graph.get(&y) {
                if dependencies.contains(&x) {
                    return false;
                }
            }
        }
    }
    true
}
