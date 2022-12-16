use std::{collections::{HashMap, VecDeque, HashSet}, cmp::Ordering};

#[derive(Clone)]
struct Valve {
    rate: u16,
    targets: Vec<&'static str>,
    idx: usize,
}

fn distance_to_all_remaining<'a>(current: &'a str, valves: &HashMap<&'a str, Valve>, opened: &HashSet<&'a str>) -> HashMap<&'a str, (u16, u16, Vec<&'a str>)> {
    let mut q = VecDeque::new();
    let mut seen = HashSet::new();
    q.push_back((current, 0, vec![]));
    seen.insert(current);
    let mut distances = HashMap::new();
    let valve = valves.get(current).unwrap();
    if valve.rate > 0 && !opened.contains(&current) {
        distances.insert(current, (0, valve.rate, vec![]));
    }
    while let Some((name, dist, path)) = q.pop_front() {
        let valve = valves.get(name).unwrap();
        if dist > 0 && valve.rate > 0 && !opened.contains(&name) {
            distances.insert(name, (dist, valve.rate, path.clone()));
        }
        for &target in &valve.targets {
            if seen.insert(target) {
                let mut path = path.clone();
                path.push(target);
                q.push_back((target, dist + 1, path));
            }
        }
    }
    distances
}

fn main() {
    let input = include_str!("day_16_input.txt").trim();

    let mut valves = HashMap::new();
    for (idx, line) in input.lines().enumerate() {
        let mut iter = line.split(' ').skip(1);
        let name = iter.next().unwrap();
        let mut iter = iter.skip(2);
        let rate = iter.next().unwrap();
        let (_, rate) = rate.split_once('=').unwrap();
        let rate = rate.strip_suffix(';').unwrap()
            .parse::<u16>().unwrap();
        let targets = iter.skip(4).map(|target| &target[..2])
            .collect::<Vec<_>>();
        valves.insert(name, Valve { rate, targets, idx });
    }
    let mut largest_pressure_released = 0;
    let mut queue: VecDeque<(&str, u16, u16, HashSet<&str>)> = VecDeque::new();
    queue.push_back(("AA", 0, 0, HashSet::new()));
    while let Some((name, time, pressure, opened)) = queue.pop_front() {
        let distances = distance_to_all_remaining(name, &valves, &opened);
        let mut can_reach_new_target = false;
        for (target, (distance, rate, _)) in distances {
            let new_time = time + distance + 1;
            if new_time > 30 {
                continue;
            }
            let pressure = pressure + ((30 - new_time) * rate);
            let mut opened = opened.clone();
            opened.insert(target);
            queue.push_back((target, new_time, pressure, opened));
            can_reach_new_target = true;
        }
        if !can_reach_new_target {
            // no new ones added, we can't go any further
            //println!("{} {} {} {:?}", name, time, pressure, opened);
            if pressure > largest_pressure_released {
                largest_pressure_released = pressure;
            }
        }
    }
    println!("part1: {largest_pressure_released}");

    let mut largest_pressure_released = 0;
    let mut queue: VecDeque<(&str, &str, u16, u16, HashSet<&str>)> = VecDeque::new();
    queue.push_back(("AA", "AA", 0, 0, HashSet::new()));
    while let Some((name1, name2, time, pressure, opened)) = queue.pop_front() {
        let distances1 = distance_to_all_remaining(name1, &valves, &opened);
        let distances2 = distance_to_all_remaining(name2, &valves, &opened);
        // DD, JJ, BB, HH, CC, EE
        // if opened.len() >= 5 && opened[0] == "DD" && opened[1] == "JJ" && opened[2] == "BB" && opened[3] == "HH" && opened[4] == "CC" {
        //     println!("{} {} {} {} {:?}", name1, name2, time, pressure, opened);
        //     println!("{:?}", distances1);
        //     println!("{:?}", distances2);
        // }

        // dbg!(&distances1);
        // dbg!(&distances2);
        // return;
        let prev_q_len = queue.len();
        for (target1, (distance1, rate1, path1)) in &distances1 {
            for (target2, (distance2, rate2, path2)) in &distances2 {
                // no need for both to go to the same target
                // ????? maybe?
                if *target1 == *target2 && (distances1.len() > 1 || distances2.len() > 1)  {
                    continue;
                }
                match distance1.cmp(distance2) {
                    Ordering::Less => { // d1 < d2
                        let added_time = distance1 + 1;
                        let new_time = time + added_time;
                        if new_time > 26 {
                            continue;
                        }
                        // find where along the path 2 will end up in `added_time` steps
                        //println!("{} < {} {:?}", distance1, distance2, path2);
                        let target2 = path2[*distance1 as usize];
                        let new_pressure = pressure + ((26 - new_time) * rate1);
                        let mut new_opened = opened.clone();
                        new_opened.insert(*target1);
                        queue.push_back((target1, target2, new_time, new_pressure, new_opened));
                    }
                    Ordering::Greater => { // d2 < d1
                        let added_time = distance2 + 1;
                        let new_time = time + added_time;
                        if new_time > 26 {
                            continue;
                        }
                        // find where along the path 2 will end up in `added_time` steps
                        let target1 = path1[*distance2 as usize];
                        let new_pressure = pressure + ((26 - new_time) * rate2);
                        let mut new_opened = opened.clone();
                        new_opened.insert(*target2);
                        queue.push_back((target1, target2, new_time, new_pressure, new_opened));
                    }
                    Ordering::Equal => {
                        // both move the same distance
                        let added_time = distance1 + 1;
                        let new_time = time + added_time;
                        if new_time > 26 {
                            continue;
                        }
                        let new_pressure = pressure + ((26 - new_time) * rate1) + ((26 - new_time) * rate2);
                        let mut new_opened = opened.clone();
                        new_opened.insert(*target1);
                        new_opened.insert(*target2);
                        queue.push_back((target1, target2, new_time, new_pressure, new_opened));
                    }
                }
            }
        };
        if prev_q_len == queue.len() {
            // no new ones added, we can't go any further
            if pressure > largest_pressure_released {
                //println!("{}+{} {} {} {:?}", name1, name2, time, pressure, opened);
                largest_pressure_released = pressure;
            }
        }
    }
    // DD, JJ, BB, HH, CC, EE
    println!("part2: {largest_pressure_released}");
}
