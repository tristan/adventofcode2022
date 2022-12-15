use std::{collections::{HashMap, HashSet}, cmp::Ordering};

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

    let ry = 2000000;
    let mut ranges = sensors.iter().filter_map(|(&(sx, sy), &(bx, by))| {
        let dist = distance((sx, sy), (bx, by));
        let rx1 = sx - (dist - (ry - sy).abs());
        let rx2 = sx + (dist - (ry - sy).abs());
        if rx1 <= rx2 {
            Some(rx1..rx2)
        } else {
            None
        }
    }).collect::<Vec<_>>();
    ranges.sort_unstable_by(|r1, r2| {
        match r1.start.cmp(&r2.start) {
            Ordering::Equal => r1.end.cmp(&r2.end),
            e => e
        }
    });
    let first = ranges.remove(0);
    let mut merged = Vec::new();
    let last = ranges.into_iter().fold(first, |prev, next| {
        // if the two can be merged
        if prev.end >= next.start {
            if next.end > prev.end {
                prev.start..next.end
            } else {
                prev
            }
        } else {
            merged.push(prev);
            next
        }
    });
    merged.push(last);
    let mut part1 = 0;
    for r in merged {
        part1 += (r.end + 1) - r.start;
        for &(bx, by) in &beacons {
            if by != ry {
                continue;
            }
            if bx >= r.start && bx <= r.end {
                part1 -= 1;
            }
        }
    }
    println!("part1: {part1}");
}
