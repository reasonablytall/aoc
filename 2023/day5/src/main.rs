use std::io::{stdin, stdout, Write};

use itertools::Itertools;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Mapping {
    source: u64,
    dest: u64,
    length: u64,
}

impl std::fmt::Debug for Mapping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "M(source: {}, dest: {}, length: {})",
            self.source, self.dest, self.length
        )
    }
}

fn main() -> anyhow::Result<()> {
    let input: Vec<String> = stdin().lines().map(Result::unwrap).collect();
    let seeds = input[0]
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| u64::from_str_radix(s, 10).unwrap())
        .collect::<Vec<u64>>();
    println!(
        "seeds: {:?}, sum: {}",
        seeds,
        seeds.iter().skip(1).step_by(2).sum::<u64>()
    );

    let mut mapping_groups = vec![];

    let mut i = 1;
    while i < input.len() {
        if input[i].is_empty() || input[i].contains("map") {
            i += 1;
            continue;
        }
        let mut mappings = vec![];
        while i < input.len() && !input[i].is_empty() {
            let (dest, source, range_len) = input[i]
                .split_whitespace()
                .map(|s| u64::from_str_radix(s, 10).unwrap())
                .collect_tuple()
                .unwrap();
            mappings.push(Mapping {
                source,
                dest,
                length: range_len,
            });
            i += 1;
        }
        mappings.sort();
        mapping_groups.push(mappings);
    }

    let result1 = seeds
        .iter()
        .map(|s| {
            return transform(&mapping_groups, *s);
        })
        .min()
        .unwrap();
    println!("results1: {:?}", result1);

    let mut result2 = u64::MAX;
    let mut iters = 0;
    for i in (0..seeds.len()).step_by(2) {
        println!(
            "i: {}, seeds[i]: {}, seeds[i+1]: {}",
            i,
            seeds[i],
            seeds[i + 1]
        );
        for s in seeds[i]..(seeds[i] + seeds[i + 1]) {
            let res = transform(&mapping_groups, s);
            if res < result2 {
                result2 = res;
            }
            iters += 1;
            if iters % 1000000 == 0 {
                print!("\rbest: {}, iters: {}", result2, iters);
                stdout().flush().unwrap();
            }
        }
    }

    Ok(())
}

fn transform(mapping_groups: &Vec<Vec<Mapping>>, seed: u64) -> u64 {
    let mut res = seed;
    for mappings in mapping_groups {
        for mapping in mappings {
            if mapping.source <= res && res < mapping.source + mapping.length {
                res = mapping.dest + res - mapping.source;
                break;
            }
        }
    }
    return res;
}
