use std::io;

fn main() {
    let mut result = 0;
    for raw_line in io::stdin().lines() {
        let line = raw_line.unwrap();
        let original = line.clone();
        let mut first_num: Option<(usize, i32)> = Option::None;
        let mut last_num: Option<(usize, i32)> = Option::None;
        for (token, num) in [
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
        ] {
            if let Some(idx) = line.find(token) {
                if first_num.is_none() || idx < first_num.unwrap().0 {
                    first_num = Some((idx, num));
                }
            }

            let rev_line = line.chars().rev().collect::<String>();
            let rev_token = token.chars().rev().collect::<String>();
            if let Some(idx) = rev_line.find(&rev_token) {
                if last_num.is_none() || idx < last_num.unwrap().0 {
                    last_num = Some((idx, num));
                }
            }
        }
        match (first_num, last_num) {
            (Some((_, first_num)), Some((_, last_num))) => {
                println!("{}->{}", original, first_num * 10 + last_num);
                result += first_num * 10 + last_num;
            }
            _ => {
                panic!("Found line without first & last numbers: {}", line)
            }
        }
    }

    println!("Result: {}", result);
}
