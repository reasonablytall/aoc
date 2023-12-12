use std::io::stdin;

fn main() {
    let input = stdin()
        .lines()
        .map(|l| {
            l.unwrap()
                .split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();

    let first_triangle = build_triangle(&input[0]);
    for i in 0..first_triangle.len() {
        for j in 0..first_triangle[i].len() {
            print!("{} ", first_triangle[i][j]);
        }
        println!("");
    }
    println!(
        "first extrap: {}",
        extrap(&first_triangle, 0, first_triangle[0].len())
    );

    let res1: i64 = input
        .iter()
        .map(|row| {
            let triangle = build_triangle(&row);
            extrap(&triangle, 0, triangle[0].len())
        })
        .sum();
    println!("res1: {}", res1);

    let res2: i64 = input
        .iter()
        .map(|row| {
            let triangle = build_triangle(&row);
            extrap_rev(&triangle, 0, -1)
        })
        .sum();
    println!("res2: {}", res2);
}

fn build_triangle(row: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut triangle = vec![row.clone()];
    for i in 1..row.len() {
        let mut new_row = Vec::new();
        for j in 0..row.len() - i {
            new_row.push(triangle[i - 1][j + 1] - triangle[i - 1][j]);
        }
        triangle.push(new_row.clone());
        if new_row.iter().all(|&n| n == 0) {
            break;
        }
    }
    triangle
}

fn extrap(triangle: &Vec<Vec<i64>>, row: usize, col: usize) -> i64 {
    if row < triangle.len() && col < triangle[row].len() {
        return triangle[row][col];
    }
    if row >= triangle.len() {
        return 0;
    }
    let prev = extrap(triangle, row, col - 1);
    let delta = extrap(triangle, row + 1, col - 1);
    // println!("prev: {}, delta: {}, res: {}", prev, delta, prev + delta);
    return prev + delta;
}

fn extrap_rev(triangle: &Vec<Vec<i64>>, row: i64, col: i64) -> i64 {
    if row >= 0
        && row < triangle.len() as i64
        && col >= 0
        && col < triangle[row as usize].len() as i64
    {
        return triangle[row as usize][col as usize];
    }
    if row >= triangle.len() as i64 {
        return 0;
    }
    let next = extrap_rev(triangle, row, col + 1);
    let delta = extrap_rev(triangle, row + 1, col);
    // println!("prev: {}, delta: {}, res: {}", prev, delta, prev + delta);
    return next - delta;
}
