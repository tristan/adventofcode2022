use std::collections::{HashSet, HashMap};

fn main() {
    let input = include_str!("day_08_input.txt").trim()
        .lines()
        .map(|line| {
            line.chars().map(|c| c as u8 - b'0').collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<u8>>>();

    let col_len = input.len();
    let row_len = input[0].len();
    let mut seen = HashSet::new();
    let mut scores = HashMap::new();

    for x in 0..row_len {
        for y in 0..col_len {
            if seen.contains(&(x, y)) {
                continue;
            }
            if x == 0 || y == 0 || x == row_len - 1 || y == col_len - 1 {
                seen.insert((x, y));
            } else {
                let mut score = 1;
                for (_d, dx, dy) in [('u', 0i64, -1i64), ('l', -1, 0), ('d', 0, 1), ('r', 1, 0)] {
                    let mut tx = x;
                    let mut ty = y;
                    let h = input[ty][tx];
                    let mut visible = 0;
                    loop {
                        if tx > 0 && tx < row_len - 1 && ty > 0 && ty < col_len - 1 {
                            visible += 1;
                            tx = (tx as i64 + dx) as usize;
                            ty = (ty as i64 + dy) as usize;
                            if input[ty][tx] >= h {
                                break;
                            }
                        } else {
                            seen.insert((x, y));
                            break;
                        }
                    }
                    if visible > 0 {
                        score *= visible;
                    }
                }
                scores.insert((x, y), score);
            }
        }
    }

    println!("part1: {}", seen.len());
    let part2 = scores.values().max().unwrap();
    println!("part2: {}", part2);
}
