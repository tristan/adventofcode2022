use std::collections::{HashSet, VecDeque};

fn main() {
    let input = include_str!("day_18_input.txt").trim();
    let cubes = input.lines().map(|line| {
        let mut iter = line.split(',').map(|s| s.parse::<i8>().unwrap());
        (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
    }).collect::<HashSet<(i8, i8, i8)>>();

    let deltas = [
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ];
    let part1 = cubes.iter().map(|&(x, y, z)| {
        6 - deltas.iter().filter(|&(dx, dy, dz)| {
            cubes.contains(&(x + dx, y + dy, z + dz))
        }).count()
    }).sum::<usize>();
    println!("part1: {part1}");

    let mut q = VecDeque::new();
    q.push_back((-1, -1, -1));
    let mut reachable = HashSet::new();
    reachable.insert((-1, -1, -1));
    while let Some((x, y, z)) = q.pop_front() {
        for (dx, dy, dz) in deltas {
            let next = (x + dx, y + dy, z + dz);
            if next.0 < -1 || next.0 > 21 || next.1 < -1 || next.1 > 21 || next.2 < -1 || next.2 > 21 {
                continue;
            }
            if !cubes.contains(&next) && reachable.insert(next) {
                q.push_back(next);
            }
        }
    }
    let part2 = cubes.iter().map(|&(x, y, z)| {
        6 - deltas.iter().filter(|&(dx, dy, dz)| {
            !reachable.contains(&(x + dx, y + dy, z + dz))
        }).count()
    }).sum::<usize>();
    println!("part2: {part2}");
}
