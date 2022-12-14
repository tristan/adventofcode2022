use std::collections::HashSet;

fn main() {
    let input = include_str!("day_14_input.txt").trim();
    let mut cave = HashSet::new();
    let mut max_y = 0;
    for line in input.lines() {
        let mut point1 = None;
        for point2 in line.split(" -> ") {
            let (x2, y2) = point2.split_once(',').unwrap();
            let x2 = x2.parse::<usize>().unwrap();
            let y2 = y2.parse::<usize>().unwrap();
            if y2 > max_y {
                max_y = y2;
            }
            if let Some((x1, y1)) = point1 {
                let x_range = if x1 > x2 {
                    x2..=x1
                } else {
                    x1..=x2
                };
                let y_range = if y1 > y2 {
                    y2..=y1
                } else {
                    y1..=y2
                };
                for x in x_range {
                    for y in y_range.clone() {
                        cave.insert((x, y));
                    }
                }
            }
            point1 = Some((x2, y2));
        }
    }
    let start_count = cave.len();
    let mut part1 = 0;
    'main: loop {
        let mut sand_pos = (500, 0);
        loop {
            let under = (sand_pos.0, sand_pos.1 + 1);
            // if there is something under the current sand position
            if under.1 == max_y + 2 {
                cave.insert(sand_pos);
                continue 'main;
            }
            else if cave.contains(&under) {
                // can it fall left?
                let left_down = (under.0 - 1, under.1);
                let right_down = (under.0 + 1, under.1);
                if !cave.contains(&left_down) {
                    sand_pos = left_down;
                } else if !cave.contains(&right_down) {
                    sand_pos = right_down;
                } else {
                    // it can't fall anywhere, so it stays
                    cave.insert(sand_pos);
                    if sand_pos == (500, 0) {
                        break 'main;
                    } else {
                        continue 'main;
                    }
                }
            } else {
                if under.1 > max_y && part1 == 0 {
                    part1 = cave.len() - start_count;
                }
                sand_pos = under;
            }
        }
    }
    println!("part1: {part1}");
    let part2 = cave.len() - start_count;
    println!("part2: {part2}");
}
