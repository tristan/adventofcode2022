use std::collections::{VecDeque, HashSet};

fn main() {
    let input = include_str!("day_12_input.txt").trim();

    let mut part1_start = None;
    let mut end = None;
    let mut starts = VecDeque::new();
    let input = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars().enumerate().map(|(x, c)| {
                match c {
                    'S' => {
                        part1_start = Some((x, y));
                        starts.push_back((x, y));
                        b'a'
                    }
                    'E' => {
                        end = Some((x, y));
                        b'z'
                    }
                    'a' => {
                        starts.push_back((x, y));
                        b'a'
                    },
                    _ => {
                        c as u8
                    }
                }
            }).collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let part1_start = part1_start.unwrap();
    let end = end.unwrap();

    let mut part1 = None;
    let mut part2 = None;

    while let Some(start) = starts.pop_front() {
        let mut q = VecDeque::new();
        let mut explored = HashSet::new();
        q.push_back((start, 0));
        explored.insert(start);

        while let Some((next, steps)) = q.pop_front() {
            if next == end {
                if start == part1_start {
                    part1 = Some(steps);
                }
                match part2 {
                    None => {
                        part2 = Some(steps);
                    }
                    Some(prev) if steps < prev => {
                        part2 = Some(steps);
                    }
                    _ => {}
                }
                break;
            }
            let (x, y) = next;
            let current = input[y][x];
            // up
            if y > 0 {
                let next = input[y - 1][x];
                if next <= current + 1 && explored.insert((x, y - 1)) {
                    q.push_back(((x, y - 1), steps + 1));
                }
            }
            // down
            if y < input.len() - 1 {
                let next = input[y + 1][x];
                if next <= current + 1 && explored.insert((x, y + 1)) {
                    q.push_back(((x, y + 1), steps + 1));
                }
            }
            // left
            if x > 0 {
                let next = input[y][x - 1];
                if next <= current + 1 && explored.insert((x - 1, y)) {
                    q.push_back(((x - 1, y), steps + 1));
                }
            }
            // right
            if x < input[y].len() - 1 {
                let next = input[y][x + 1];
                if next <= current + 1 && explored.insert((x + 1, y)) {
                    q.push_back(((x + 1, y), steps + 1));
                }
            }
        }
    }
    println!("part1: {}", part1.unwrap());
    println!("part2: {}", part2.unwrap());
}
