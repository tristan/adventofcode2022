use std::collections::HashMap;

fn main() {
    let input = include_str!("day_07_input.txt").trim();

    let mut paths = HashMap::<Vec<&str>, usize>::new();
    let start_path = Vec::new();
    paths.insert(start_path.clone(), 0);
    input.lines().fold(start_path, |path: Vec<&str>, line| {
        match &line[..4] {
            "$ cd" => {
                let mut path = path.clone();
                match &line[5..] {
                    "/" => {
                        path = vec![];
                    }
                    ".." => {
                        path.pop();
                    }
                    other => {
                        path.push(other);
                        paths.entry(path.clone()).or_insert(0);
                    }
                }
                path
            }
            "$ ls" | "dir " => {
                path
            }
            _ => {
                let (size, _name) = line.split_once(' ').unwrap();
                let size = size.parse::<usize>().unwrap();
                {
                    let mut path = path.clone();
                    loop {
                        *paths.get_mut(&path).unwrap() += size;
                        if !path.is_empty() {
                            path.pop();
                        } else {
                            break
                        }
                    }
                }
                path
            }
        }
    });

    let part1 = paths.values().filter(|&&v| v <= 100000).sum::<usize>();
    println!("part1: {part1}");
    let free_space = 70000000 - paths.get(&vec![]).unwrap();
    let required_space = 30000000 - free_space;
    let part2 = paths.values().filter(|&&v| v >= required_space).min().unwrap();
    println!("part2: {part2}");
}
