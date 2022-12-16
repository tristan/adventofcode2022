use std::collections::{HashMap, VecDeque, HashSet};

#[derive(Clone)]
struct Valve {
    rate: u64,
    targets: Vec<&'static str>
}

fn distance_to_all_remaining<'a>(current: &'a str, valves: &HashMap<&'a str, Valve>, opened: &[&'a str]) -> HashMap<&'a str, (u64, u64)> {
    let mut q = VecDeque::new();
    let mut seen = HashSet::new();
    q.push_back((current, 0));
    seen.insert(current);
    let mut distances = HashMap::new();
    while let Some((name, dist)) = q.pop_front() {
        let valve = valves.get(name).unwrap();
        if dist > 0 && valve.rate > 0 && !opened.contains(&name) {
            distances.insert(name, (dist, valve.rate));
        }
        for target in &valve.targets {
            if seen.insert(target) {
                q.push_back((target, dist + 1));
            }
        }
    }
    distances
}

fn main() {
    let input = include_str!("day_16_input.txt").trim();

    let mut valves = HashMap::new();
    for line in input.lines() {
        let mut iter = line.split(' ').skip(1);
        let name = iter.next().unwrap();
        let mut iter = iter.skip(2);
        let rate = iter.next().unwrap();
        let (_, rate) = rate.split_once('=').unwrap();
        let rate = rate.strip_suffix(';').unwrap()
            .parse::<u64>().unwrap();
        let targets = iter.skip(4).map(|target| &target[..2])
            .collect::<Vec<_>>();
        valves.insert(name, Valve { rate, targets });
    }

    let target_opened = valves.values().filter(|v| v.rate > 0).count();

    let mut largest_pressure_released = 0;
    let mut queue: VecDeque<(&str, u64, u64, Vec<&str>)> = VecDeque::new();
    queue.push_back(("AA", 0, 0, Vec::new()));
    while let Some((name, time, pressure, opened)) = queue.pop_front() {
        let distances = distance_to_all_remaining(name, &valves, &opened);
        let prev_q_len = queue.len();
        for (target, (distance, rate)) in distances {
            let new_time = time + distance + 1;
            if new_time > 30 {
                continue;
            }
            let pressure = pressure + ((30 - new_time) * rate);
            let mut opened = opened.clone();
            opened.push(target);
            queue.push_back((target, new_time, pressure, opened));
        }
        if prev_q_len == queue.len() {
            // no new ones added, we can't go any further
            //println!("{} {} {} {:?}", name, time, pressure, opened);
            if pressure > largest_pressure_released {
                largest_pressure_released = pressure;
            }
        }
    }
    println!("part1: {largest_pressure_released}");

    let mut largest_pressure_released = 0;
    let mut queue: VecDeque<(&str, u64, u64, Vec<&str>)> = VecDeque::new();
    queue.push_back(("AA", "AA", 0, 0, Vec::new()));
    while let Some((name1, name2, time, pressure, opened)) = queue.pop_front() {
        let distances1 = distance_to_all_remaining(name1, &valves, &opened);
        let distances2 = distance_to_all_remaining(name2, &valves, &opened);
        let prev_q_len = queue.len();
        for (target, (distance, rate)) in distances {
            let new_time = time + distance + 1;
            if new_time > 26 {
                continue;
            }
            let pressure = pressure + ((26 - new_time) * rate);
            let mut opened = opened.clone();
            opened.push(target);
            queue.push_back((target, new_time, pressure, opened));
        }
        if prev_q_len == queue.len() {
            // no new ones added, we can't go any further
            //println!("{} {} {} {:?}", name, time, pressure, opened);
            if pressure > largest_pressure_released {
                largest_pressure_released = pressure;
            }
        }
    }
    println!("part1: {largest_pressure_released}");
}
