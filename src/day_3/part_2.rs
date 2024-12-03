const INPUT: &str = include_str!("../../files/day_3.txt");

#[allow(dead_code)]
pub fn run() -> String {
    let mut total_sum: i32 = 0;
    let mut is_doing = true;

    for line in INPUT.lines() {
        let mut chars = line.chars().peekable();
        while let Some(c) = chars.next() {
            if c == 'd' {
                if chars.peek() == Some(&'o') {
                    chars.next();
                    
                    if check_pattern(&mut chars, &['(', ')']) {
                        is_doing = true;
                    }
                    else if check_pattern(&mut chars, &['n', '\'', 't', '(', ')']) {
                        is_doing = false;
                    }
                }
            }

            if !is_doing {
                continue;
            }

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

fn check_pattern(chars: &mut std::iter::Peekable<std::str::Chars>, pattern: &[char]) -> bool {
    for &expected in pattern {
        if chars.peek() != Some(&expected) {
            return false;
        }
        chars.next();
    }
    true
}
