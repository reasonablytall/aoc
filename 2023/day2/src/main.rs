use regex::Regex;
use std::io::{self, stdin};

use anyhow::Result;

#[derive(Debug)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

fn main() -> Result<()> {
    let mut result1 = 0;
    let mut result2 = 0;
    for (i, raw_line) in io::stdin().lines().enumerate() {
        let line = raw_line.unwrap();
        let max_red = color_max(&line, "red");
        let max_green = color_max(&line, "green");
        let max_blue = color_max(&line, "blue");
        if max_red <= 12 && max_green <= 13 && max_blue <= 14 {
            result1 += i + 1;
        }
        result2 += max_red * max_green * max_blue;
    }
    println!("Result1: {}, Result2: {}", result1, result2);
    Ok(())
}

fn color_max(line: &str, color: &str) -> u32 {
    return Regex::new(&format!("(\\d+) {}", color))
        .unwrap()
        .captures_iter(&line)
        .map(|c| u32::from_str_radix(c.extract::<1>().1[0], 10).unwrap())
        .max()
        .unwrap();
}

fn main2() -> Result<()> {
    let mut result1 = 0;
    let mut result2 = 0;
    for raw_line in io::stdin().lines() {
        let line = raw_line.unwrap();
        let game = u32::from_str_radix(
            line.split_once(":").unwrap().0.split_once(" ").unwrap().1,
            10,
        )?;
        let rounds: Vec<Round> = line
            .split_once(":")
            .unwrap()
            .1
            .split(";")
            .map(|roundstr| {
                let (mut r, mut g, mut b) = (0, 0, 0);
                roundstr
                    .split(",")
                    .for_each(|sample| match sample.trim().split_once(" ") {
                        Some((num, col)) => match col {
                            "red" => r = u32::from_str_radix(num, 10).unwrap(),
                            "green" => g = u32::from_str_radix(num, 10).unwrap(),
                            "blue" => b = u32::from_str_radix(num, 10).unwrap(),
                            _ => panic!("Invalid input: {} -> {} & {}", sample, num, col),
                        },
                        None => panic!("Invalid input"),
                    });
                return Round {
                    red: r,
                    green: g,
                    blue: b,
                };
            })
            .collect();
        println!("{}\n\t-> {}: {:?}\n", line, game, rounds);

        let mut possible = true;
        for round in &rounds {
            if round.red > 12 || round.green > 13 || round.blue > 14 {
                possible = false;
            }
        }
        if possible {
            result1 += game;
        }

        let (mut max_red, mut max_green, mut max_blue) = (0, 0, 0);
        for round in rounds {
            if round.red > max_red {
                max_red = round.red;
            }
            if round.green > max_green {
                max_green = round.green;
            }
            if round.blue > max_blue {
                max_blue = round.blue;
            }
        }
        result2 += max_red * max_green * max_blue;
    }

    println!("Result1: {}, Result2: {}", result1, result2);

    Ok(())
}
