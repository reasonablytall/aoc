use std::{
    collections::{HashMap, HashSet},
    io::{stdin, stdout, Write},
};

use regex::Regex;

use num::integer::lcm;

fn main() {
    let mut input = stdin().lines();
    let instructions = input
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .collect::<Vec<char>>();

    let entry_regex = Regex::new(r"(.*) = \((.*), (.*)\)").unwrap();
    let map = input
        .skip(1)
        .map(Result::unwrap)
        .map(|l| {
            let (_, [loc, left, right]) = entry_regex.captures(&l).unwrap().extract();
            (loc.to_string(), (left.to_string(), right.to_string()))
        })
        .collect::<HashMap<String, (String, String)>>();

    // println!("instructions: {:?}", instructions);
    // println!("map: {:?}", map);

    // let mut loc = "AAA";
    // let mut step1 = 0;
    // while loc != "ZZZ" {
    //     loc = next_hop(&map, &loc, &instructions, step1);
    //     step1 += 1;
    // }
    // println!("result1: {}", step1);

    println!("map: {}, instructions: {}", map.len(), instructions.len());

    let ends = map
        .keys()
        .map(|k| (k, find_end(&map, &k, &instructions)))
        .collect::<HashMap<&String, String>>();

    let cycle_lengths = map
        .keys()
        .filter(|k| k.ends_with("A"))
        .map(|k| {
            println!("k: {}", k);
            let mut loc = k;
            let mut seen = HashMap::new();
            let mut step = 0;
            let mut z_counts = vec![];
            while !seen.contains_key(&(loc, step & instructions.len())) {
                if loc.ends_with("Z") {
                    z_counts.push(step);
                    seen.insert((loc, step % instructions.len()), step);
                }
                loc = next_hop(&map, &loc, &instructions, step);
                step += 1;
            }
            println!(
                "entry {}: step = {}, loc = {}, cycle_start = {}, cycle_len = {}\n",
                k,
                step,
                loc,
                seen.get(&(loc, step & instructions.len())).unwrap(),
                step - seen.get(&(loc, step & instructions.len())).unwrap()
            );
            let found_idx = seen.get(&(loc, step & instructions.len())).unwrap();
            let res = (k, (*found_idx, step - found_idx));
            res
        })
        .collect::<HashMap<&String, (usize, usize)>>();
    println!("cycle_lengths: {:?}", cycle_lengths);

    let lcm = cycle_lengths
        .iter()
        .fold(1 as usize, |acc, (_, (_, cycle_len))| lcm(acc, *cycle_len));
    println!("lcm: {}", lcm);

    // answer is ^. Ugh what a mess of trying things. Lucky that the input is
    // structured such that I didn't need to solve the LCM with offsets issue...

    let mut counters = cycle_lengths
        .values()
        .map(|(cycle_start, cycle_len)| (*cycle_start, *cycle_start, *cycle_len))
        .collect::<Vec<(usize, usize, usize)>>();

    while !counters.iter().all(|c| c.0 == counters[0].0) {
        let smallest: &mut (usize, usize, usize) = counters.iter_mut().min().unwrap();
        smallest.0 += smallest.2;
        print!("\r{}", smallest.0);
    }

    println!("counters: {:?}", counters);

    let mut locs = map
        .keys()
        .filter(|k| k.ends_with("A"))
        .collect::<Vec<&String>>();
    let mut step2 = 0;
    while locs.iter().filter(|l| !l.ends_with("Z")).count() > 0 {
        for i in 0..locs.len() {
            locs[i] = match instructions[step2 % instructions.len()] {
                'L' => &map[locs[i]].0,
                'R' => &map[locs[i]].1,
                _ => unreachable!("Invalid instruction"),
            };
        }
        step2 += 1;
        if step2 % 1000000 == 0 {
            print!("\rstep2: {}", step2);
            stdout().flush().unwrap();
        }
    }
}

fn next_hop<'a>(
    map: &'a HashMap<String, (String, String)>,
    loc: &str,
    instructions: &Vec<char>,
    step: usize,
) -> &'a String {
    match instructions[step % instructions.len()] {
        'L' => &map.get(loc).unwrap_or_else(|| panic!("{}", loc)).0,
        'R' => &map.get(loc).unwrap_or_else(|| panic!("{}", loc)).1,
        _ => unreachable!("Invalid instruction"),
    }
}

fn find_end(
    map: &HashMap<String, (String, String)>,
    loc: &str,
    instructions: &Vec<char>,
) -> String {
    let mut loc = loc;
    for i in 0..instructions.len() {
        loc = match instructions[i] {
            'L' => &map[loc].0,
            'R' => &map[loc].1,
            _ => unreachable!("Invalid instruction"),
        };
    }
    return loc.to_string();
}
