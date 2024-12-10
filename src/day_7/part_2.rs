use itertools::Itertools;

const INPUT: &str = include_str!("../../files/day_7.txt");

pub fn run() -> String {
    let mut calibration_sum = 0;

    for line in INPUT.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        let expected = parts[0].trim().parse::<u64>().unwrap();
        let nums: Vec<u64> = parts[1]
            .split_whitespace()
            .map(|n| n.parse::<u64>().unwrap())
            .collect();

        let operator_count = nums.len() - 1;
        let operator_combinations = (0..operator_count)
            .map(|_| vec!["+", "*", "||"].into_iter())
            .multi_cartesian_product();

        let mut is_valid = false;
        for operators in operator_combinations {
            if evaluate_expression(&nums, &operators) == expected {
                is_valid = true;
                break;
            }
        }

        if is_valid {
            calibration_sum += expected;
        }
    }

    calibration_sum.to_string()
}

fn evaluate_expression(nums: &[u64], operators: &[&str]) -> u64 {
    let mut result = nums[0];
    for (i, &op) in operators.iter().enumerate() {
        match op {
            "+" => result += nums[i + 1],
            "*" => result *= nums[i + 1],
            "||" => {
                let digits = num_digits(nums[i + 1]);
                result = result * 10u64.pow(digits) + nums[i + 1];
            }
            _ => panic!("Unknown operator"),
        }
    }
    result
}

fn num_digits(mut n: u64) -> u32 {
    if n == 0 {
        1
    } else {
        let mut count = 0;
        while n > 0 {
            n /= 10;
            count += 1;
        }
        count
    }
}
