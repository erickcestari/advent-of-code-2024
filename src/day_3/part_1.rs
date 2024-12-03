const INPUT: &str = include_str!("../../files/day_3.txt");

#[allow(dead_code)]
pub fn run() -> String {
    let mut total_sum: i32 = 0;

    for line in INPUT.lines() {
        let mut chars = line.chars().peekable();

        while let Some(c) = chars.next() {
            if c == 'm' && chars.next() == Some('u') && chars.next() == Some('l') && chars.next() == Some('(') {
                let mut x = String::new();
                while let Some(&digit) = chars.peek() {
                    if digit.is_ascii_digit() {
                        x.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }

                if x.len() > 3 {
                    continue;
                }

                if chars.next() != Some(',') {
                    continue;
                }

                let mut y = String::new();
                while let Some(&digit) = chars.peek() {
                    if digit.is_ascii_digit() {
                        y.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }

                if y.len() > 3 {
                    continue;
                }

                if chars.next() != Some(')') {
                    continue;
                }

                if let (Ok(x), Ok(y)) = (x.parse::<i32>(), y.parse::<i32>()) {
                    total_sum += x * y;
                }
            }
        }
    }

    total_sum.to_string()
}
