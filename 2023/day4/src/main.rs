use std::{
    char, cmp,
    collections::{HashMap, HashSet},
    io::stdin,
};

use regex::Regex;

fn main() {
    let mut result1 = 0;
    let mut result2 = 0;

    let mut copies: HashMap<usize, u32> = HashMap::new();
    for (i, line) in stdin().lines().map(|l| l.unwrap()).enumerate() {
        let parsed = Regex::new(r": +((?:\d+ *)*) \| +((?:\d+ *)*)")
            .unwrap()
            .captures(&line)
            .unwrap();
        let win_nums = parsed.extract::<2>().1[0]
            .split_whitespace()
            .map(|n| u32::from_str_radix(n, 10).unwrap())
            .collect::<HashSet<u32>>();
        let had_nums = parsed.extract::<2>().1[1]
            .split_whitespace()
            .map(|n| u32::from_str_radix(n, 10).unwrap())
            .collect::<HashSet<u32>>();
        println!(
            "line: {}, win_nums: {:?}, had_nums: {:?}",
            line, win_nums, had_nums
        );

        let wins = had_nums.intersection(&win_nums).count();
        result1 += 2_u32.pow(wins as u32);

        let num_this = copies.get(&i).unwrap_or(&1).clone();
        for n in 0..wins {
            copies.insert(i + n + 1, copies.get(&(i + n + 1)).unwrap_or(&1) + num_this);
        }
        result2 += num_this;
    }
    println!("result1: {}, result2: {}", result1, result2);
}
