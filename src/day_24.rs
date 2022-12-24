use std::{
    cmp::Ordering,
    collections::{HashSet, VecDeque},
};

#[derive(Clone, Copy)]
enum Blizzard {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy)]
enum Entity {
    Blizzard(Blizzard),
    Wall,
    Empty,
}

fn main() {
    let input = include_str!("day_24_input.txt")
        .trim_end()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '^' => Entity::Blizzard(Blizzard::North),
                    '>' => Entity::Blizzard(Blizzard::East),
                    'v' => Entity::Blizzard(Blizzard::South),
                    '<' => Entity::Blizzard(Blizzard::West),
                    '#' => Entity::Wall,
                    '.' => Entity::Empty,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let start = (1, 0);
    let goal = (input[0].len() - 2, input.len() - 1);

    let part1 = solve(&input, 0, start, goal);
    println!("part1: {part1}");
    let and_back = solve(&input, part1 - 1, goal, start);
    let part2 = solve(&input, and_back - 1, start, goal);
    println!("part2: {part2}");
}

fn add(lhs: usize, rhs: i32) -> usize {
    match rhs.cmp(&0) {
        Ordering::Equal => lhs,
        Ordering::Less => lhs - rhs.unsigned_abs() as usize,
        Ordering::Greater => lhs + rhs as usize,
    }
}

pub fn gcd(mut n: usize, mut m: usize) -> usize {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}

fn adjust(dir: Blizzard, start_pos: usize, limit: usize, time: usize) -> usize {
    let start_pos = start_pos as isize - 1;
    let limit = limit as isize;
    let time = time as isize;
    match dir {
        Blizzard::North | Blizzard::West => ((start_pos - time).rem_euclid(limit) + 1) as usize,
        Blizzard::South | Blizzard::East => ((start_pos + time).rem_euclid(limit) + 1) as usize,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_adjust() {
        assert_eq!(adjust(Blizzard::North, 1, 4, 1), 4);
        assert_eq!(adjust(Blizzard::North, 1, 4, 2), 3);
        assert_eq!(adjust(Blizzard::North, 1, 4, 3), 2);
        assert_eq!(adjust(Blizzard::North, 1, 4, 4), 1);
        assert_eq!(adjust(Blizzard::North, 1, 4, 5), 4);
        assert_eq!(adjust(Blizzard::North, 1, 4, 6), 3);
    }
}

fn check_if_safe(grid: &Vec<Vec<Entity>>, x: usize, y: usize, time: usize) -> bool {
    let height = grid.len() - 2;
    let width = grid[0].len() - 2;
    for by in 1..(grid.len() - 1) {
        if let Entity::Blizzard(bliz @ Blizzard::North | bliz @ Blizzard::South) = grid[by][x] {
            let nby = adjust(bliz, by, height, time + 1);
            if nby == y {
                return false;
            }
        }
    }
    for bx in 1..(grid[0].len() - 1) {
        if let Entity::Blizzard(bliz @ Blizzard::East | bliz @ Blizzard::West) = grid[y][bx] {
            let nbx = adjust(bliz, bx, width, time + 1);
            if nbx == x {
                return false;
            }
        }
    }
    true
}

fn solve(
    grid: &Vec<Vec<Entity>>,
    time: usize,
    start: (usize, usize),
    end: (usize, usize),
) -> usize {
    let mut q: VecDeque<(usize, (usize, usize))> = VecDeque::new();
    let tmod = (grid[0].len() - 2) * gcd(grid.len() - 2, grid[0].len() - 2);
    q.push_back((time, start));
    let mut seen = HashSet::new();
    seen.insert((time, start));
    while let Some((time, (x, y))) = q.pop_front() {
        for (dx, dy) in [(0, 0), (0, -1), (1, 0), (0, 1), (-1, 0)] {
            // only process south movement on entrance to reduce the checks needed elsewhere
            if (y == 0 && ![(0, 0), (0, 1)].contains(&(dx, dy)))
                || (y == grid.len() - 1 && ![(0, 0), (0, -1)].contains(&(dx, dy)))
            {
                continue;
            }
            let (nx, ny) = (add(x, dx), add(y, dy));
            // check end condition
            if (nx, ny) == end {
                return time + 1 + 1; // + extra 1 for index by 1 result
            }
            // make sure it's not a wall or off the grid and if it's safe to move to
            if ny < grid.len()
                && nx < grid[ny].len()
                && !matches!(grid[ny][nx], Entity::Wall)
                && check_if_safe(grid, nx, ny, time + 1)
                && seen.insert(((time + 1) % tmod, (nx, ny)))
            {
                //println!("{time}: {x},{y} -> {nx},{ny}");
                q.push_back((time + 1, (nx, ny)));
            }
        }
    }
    panic!("didn't find a result!")
}
