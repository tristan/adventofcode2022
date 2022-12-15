use std::collections::{HashMap, HashSet};

fn distance((x1, y1): (i64, i64), (x2, y2): (i64, i64)) -> i64 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn main() {
    let input = include_str!("day_15_input.txt").trim();

    let mut sensors = HashMap::new();
    let mut beacons = HashSet::new();
    for line in input.lines() {
        let mut iter = line.split(' ').skip(2);
        let sx = iter
            .next()
            .unwrap()
            .strip_prefix("x=")
            .unwrap()
            .strip_suffix(',')
            .unwrap()
            .parse::<i64>()
            .unwrap();
        let sy = iter
            .next()
            .unwrap()
            .strip_prefix("y=")
            .unwrap()
            .strip_suffix(':')
            .unwrap()
            .parse::<i64>()
            .unwrap();
        let mut iter = iter.skip(4);
        let bx = iter
            .next()
            .unwrap()
            .strip_prefix("x=")
            .unwrap()
            .strip_suffix(',')
            .unwrap()
            .parse::<i64>()
            .unwrap();
        let by = iter
            .next()
            .unwrap()
            .strip_prefix("y=")
            .unwrap()
            .parse::<i64>()
            .unwrap();
        sensors.insert((sx, sy), (bx, by));
        beacons.insert((bx, by));
    }

    let mut cantbe = HashSet::new();

    for (&(sx, sy), &(bx, by)) in sensors.iter() {
        let dist = distance((sx, sy), (bx, by));
        let mut diff = 0;
        loop {
            let left = (sx - diff, 2000000);
            let right = (sx + diff, 2000000);
            let ldist = distance((sx, sy), left);
            if ldist <= dist {
                if !beacons.contains(&left) {
                    cantbe.insert(left);
                }
                if !beacons.contains(&right) {
                    cantbe.insert(right);
                }
                diff += 1;
            } else {
                break;
            }
        }
    }
    let part1 = cantbe.len();
    println!("part1: {part1}");
}
