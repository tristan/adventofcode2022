use std::{collections::{HashMap, HashSet}, cmp::Ordering};

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn rotate(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::East,
            Direction::East => Direction::North,
        }
    }

    fn iter(&self) -> DirectionIter {
        DirectionIter(*self, 4)
    }

    fn deltas(&self) -> [(i64, i64); 3] {
        match self {
            Direction::North => [(-1, -1), (0, -1), (1, -1)],
            Direction::South => [(-1, 1), (0, 1), (1, 1)],
            Direction::West => [(-1, -1), (-1, 0), (-1, 1)],
            Direction::East => [(1, -1), (1, 0), (1, 1)],
        }
    }
}

struct DirectionIter(Direction, u8);

impl Iterator for DirectionIter {
    type Item = Direction;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.1 > 0 {
            self.1 -= 1;
            let res = self.0;
            self.0 = self.0.rotate();
            Some(res)
        } else {
            None
        }
    }
}

fn count_empty(grid: &HashSet<(i64, i64)>) -> usize {
    let ((min_x, min_y), (max_x, max_y)) = grid.iter().fold(
        ((i64::MAX, i64::MAX), (i64::MIN, i64::MIN)),
        |((min_x, min_y), (max_x, max_y)), &(x, y)| {
            ((min_x.min(x), min_y.min(y)), (max_x.max(x), max_y.max(y)))
        },
    );
    let mut count_empty = 0;
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if grid.contains(&(x, y)) {
                #[cfg(debug_assertions)]
                print!("#");
            } else {
                #[cfg(debug_assertions)]
                print!(".");
                count_empty += 1;
            }
        }
        #[cfg(debug_assertions)]
        println!();
    }
    #[cfg(debug_assertions)]
    println!();
    count_empty
}

fn main() {
    let input = include_str!("day_23_input.txt").trim_end();
    let grid = input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .filter_map(|(x, c)| {
                    if c == '#' {
                        Some((x as i64, y as i64))
                    } else {
                        None
                    }
                })
                .collect::<Vec<(i64, i64)>>()
        })
        .collect::<HashSet<(i64, i64)>>();

    let mut start_direction = Direction::North;
    let mut cgrid = grid.clone();
    for round in 1.. {
        // check moves
        let mut moves: HashMap<(i64, i64), Vec<(i64, i64)>> = HashMap::with_capacity(grid.len());
        for &(x, y) in cgrid.iter() {
            // check if the elf should move at all
            if ![(-1, -1), (0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0)].into_iter().any(|(dx, dy)| {
                cgrid.get(&(x + dx, y + dy)).is_some()
            }) {
                // don't move
                moves.entry((x, y)).or_default().push((x, y));
            } else {
                // try propose a move
                let mut found = false;
                for dir in start_direction.iter() {
                    if !dir
                        .deltas()
                        .iter()
                        .any(|&(dx, dy)| cgrid.get(&(x + dx, y + dy)).is_some())
                    {
                        let (dx, dy) = dir.deltas()[1];
                        let e = moves.entry((x + dx, y + dy)).or_default();
                        e.push((x, y));
                        found = true;
                        break;
                    }
                }
                if !found {
                    // no possible moves
                    moves.entry((x, y)).or_default().push((x, y));
                }
            }
        }
        // make moves
        let pgrid = cgrid.clone();
        cgrid.clear();
        for ((x, y), v) in moves.into_iter() {
            match v.len().cmp(&1) {
                Ordering::Equal => {
                    cgrid.insert((x, y));
                }
                Ordering::Greater => {
                    for v in v {
                        cgrid.insert(v);
                    }
                }
                _ => panic!()
            }
        }
        // rotate instructions
        start_direction = start_direction.rotate();
        // check conditions
        if round == 10 {
            let part1 = count_empty(&cgrid);
            println!("part1: {part1}");
        } else if round > 10 && pgrid == cgrid {
            println!("part2: {round}");
            break;
        }
    }
}
