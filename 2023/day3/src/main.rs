use std::{char, cmp, io::stdin};

use regex::Regex;

fn main() {
    let mut result1 = 0;
    let schematic: Vec<String> = stdin().lines().map(|l| l.unwrap()).collect();
    for (i, line) in schematic.iter().enumerate() {
        Regex::new(r"\d+").unwrap().find_iter(line).for_each(|m| {
            let mut is_part = false;
            for j in m.start()..m.start() + m.len() {
                if check_surrounding(&schematic, (i, j)) {
                    is_part = true;
                    break;
                }
            }
            if is_part {
                // println!("Found part {}", m.as_str());
                result1 += u32::from_str_radix(m.as_str(), 10).unwrap();
            }
        })
    }

    let mut result2 = 0;
    for (i, line) in schematic.iter().enumerate() {
        for sm in Regex::new(r"\*").unwrap().find_iter(line) {
            let j = sm.start();

            let mut adj_nums = vec![];
            for k in i.saturating_sub(1)..cmp::min(i + 2, schematic.len()) {
                adj_nums.extend(
                    Regex::new(r"\d+")
                        .unwrap()
                        .find_iter(&schematic[k])
                        .filter(|m| {
                            return !(m.start() > j + 1 || m.end() < j);
                        })
                        .map(|m| m.as_str()),
                )
            }
            println!("* at ({}, {}), adj_nums: {:?}", i, j, adj_nums);
            if adj_nums.len() == 2 {
                result2 += u32::from_str_radix(adj_nums[0], 10).unwrap()
                    * u32::from_str_radix(adj_nums[1], 10).unwrap();
            }
        }
    }

    println!("Result 1: {}, Result 2: {}", result1, result2);
}

fn check_surrounding(schematic: &Vec<String>, (x, y): (usize, usize)) -> bool {
    for i in x.saturating_sub(1)..x + 2 {
        for j in y.saturating_sub(1)..y + 2 {
            if i >= schematic.len() || j >= schematic[i].len() {
                continue;
            }

            let c = schematic[i].chars().nth(j).unwrap();
            if !(c == '.' || c.is_numeric()) {
                return true;
            }
        }
    }
    return false;
}
