use std::io::stdin;

fn main() {
    let input = stdin().lines().map(|l| l.unwrap()).collect::<Vec<String>>();
    let times: Vec<u64> = input[0]
        .split_whitespace()
        .skip(1)
        .map(|s| u64::from_str_radix(s, 10).unwrap())
        .collect();
    let distances: Vec<u64> = input[1]
        .split_whitespace()
        .skip(1)
        .map(|s| u64::from_str_radix(s, 10).unwrap())
        .collect();

    let mut results = vec![];
    for i in 0..times.len() {
        let mut ways = 0;
        for charge in 0..times[i] {
            let distance = charge * (times[i] - charge);
            if distance > distances[i] {
                ways += 1;
            }
        }
        results.push(ways);
    }
    println!("result1: {}", results.iter().product::<u64>());

    let mut result2 = 0;
    for charge in 0..49787980 {
        let distance: u64 = charge * (49787980 - charge);
        if distance > 298118510661181 {
            result2 += 1;
        }
    }
    println!("result2: {}", result2);
}
