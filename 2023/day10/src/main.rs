use std::{
    collections::HashSet,
    io::stdin,
    ops::{Add, Mul},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(i64, i64);
impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}
impl Mul<i64> for Point {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl Point {
    fn midpoint_with(self, other: Self) -> Self {
        Self((self.0 + other.0) / 2, (self.1 + other.1) / 2)
    }
}

fn main() {
    let input = stdin()
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let start = (|| {
        for i in 0..input.len() {
            for j in 0..input[i].len() {
                if input[i][j] == 'S' {
                    return Some(Point(j as i64, i as i64));
                }
            }
        }
        return None;
    })()
    .unwrap();

    let cycle = ['N', 'S', 'E', 'W']
        .iter()
        .map(|dir| {
            let from = start;
            let pos = start + dir_to_delta(*dir);
            let c = find_cycle(&input, pos, from);
            return match c {
                Some(c) => {
                    println!("{} => length: {}, length/2: {}", dir, c.len(), c.len() / 2);
                    Some(c)
                }
                None => {
                    println!("{} => No cycle", dir);
                    None
                }
            };
        })
        .find(Option::is_some)
        .unwrap()
        .unwrap();

    let doubled = double_cycle(&cycle);
    // println!("cycle: {:?}", cycle);
    // println!("doubled: {:?}", doubled);

    let mut res = None;
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            print!("\rpos: {}, {}", j, i);
            let start = Point(j as i64, i as i64);
            let dfs = dfs_inside(
                &doubled,
                (
                    Point(0, 0),
                    Point(input[0].len() as i64 * 2, input.len() as i64 * 2),
                ),
                start * 2,
            );
            match dfs {
                Some(dfs) => {
                    // println!(
                    //     "{:?} => length: {}, evens length: {:?}",
                    //     start,
                    //     dfs.len(),
                    //     dfs.iter().filter(|p| p.0 % 2 == 0 && p.1 % 2 == 0).count(),
                    // );
                    if dfs.len() > 0 {
                        res = Some(dfs);
                    }
                }
                None => {} //println!("{:?} => No dfs", start),
            }
        }
    }
    println!(
        "res length: {:?}, evens length: {:?}",
        res.as_ref().unwrap().len(),
        res.unwrap()
            .iter()
            .filter(|p| p.0 % 2 == 0 && p.1 % 2 == 0)
            .count()
    );
}

fn dfs_inside(cycle: &Vec<Point>, bounds: (Point, Point), start: Point) -> Option<HashSet<Point>> {
    let mut seen = HashSet::new();
    let cycle_set = cycle.iter().cloned().collect::<HashSet<Point>>();
    let mut todo = vec![start];
    while let Some(pos) = todo.pop() {
        if seen.contains(&pos) || cycle_set.contains(&pos) {
            continue;
        }
        if pos.0 < bounds.0 .0 || pos.0 > bounds.1 .0 || pos.1 < bounds.0 .1 || pos.1 > bounds.1 .1
        {
            return None;
        }
        seen.insert(pos);
        todo.push(Point(pos.0 + 1, pos.1));
        todo.push(Point(pos.0 - 1, pos.1));
        todo.push(Point(pos.0, pos.1 + 1));
        todo.push(Point(pos.0, pos.1 - 1));
    }
    Some(seen)
}

fn double_cycle(cycle: &Vec<Point>) -> Vec<Point> {
    let doubled = cycle.iter().map(|p| *p * 2).collect::<Vec<Point>>();

    let mut res = vec![];
    for i in 0..(doubled.len() - 1) {
        res.push(doubled[i]);
        res.push(doubled[i].midpoint_with(doubled[i + 1]));
    }
    res.push(doubled[doubled.len() - 1]);
    res.push(doubled[doubled.len() - 1].midpoint_with(doubled[0]));
    res
}

fn find_cycle(map: &Vec<Vec<char>>, mut pos: Point, mut from: Point) -> Option<Vec<Point>> {
    let mut res = vec![from];
    while pos != res[0] {
        let next = match next_pos(&map, pos, from) {
            Some(p) => p,
            None => return None,
        };
        from = pos;
        pos = next;
        res.push(from);
    }
    Some(res)
}

fn next_pos(map: &Vec<Vec<char>>, pos: Point, from: Point) -> Option<Point> {
    if pos.0 < 0 || pos.1 < 0 {
        return None;
    }
    let from_dir = delt_to_dir(Point(from.0 - pos.0, from.1 - pos.1));
    let pipe = map[pos.1 as usize][pos.0 as usize];
    let dir = match (pipe, from_dir) {
        ('|', 'N') => Some('S'),
        ('|', 'S') => Some('N'),
        ('-', 'E') => Some('W'),
        ('-', 'W') => Some('E'),
        ('L', 'N') => Some('E'),
        ('L', 'E') => Some('N'),
        ('J', 'N') => Some('W'),
        ('J', 'W') => Some('N'),
        ('7', 'W') => Some('S'),
        ('7', 'S') => Some('W'),
        ('F', 'E') => Some('S'),
        ('F', 'S') => Some('E'),
        _ => None,
    };
    // println!(
    //     "pos: {:?}, from: {:?}, from_dir: {:?}, pipe: {:?}, dir: {:?}",
    //     pos, from, from_dir, pipe, dir
    // );
    let delt = dir_to_delta(dir?);
    Some(Point(pos.0 + delt.0, pos.1 + delt.1))
}

fn delt_to_dir(delt: Point) -> char {
    match delt {
        Point(0, -1) => 'N',
        Point(0, 1) => 'S',
        Point(1, 0) => 'E',
        Point(-1, 0) => 'W',
        _ => panic!("Invalid delta"),
    }
}

fn dir_to_delta(dir: char) -> Point {
    match dir {
        'N' => Point(0, -1),
        'S' => Point(0, 1),
        'E' => Point(1, 0),
        'W' => Point(-1, 0),
        _ => panic!("Invalid direction"),
    }
}
