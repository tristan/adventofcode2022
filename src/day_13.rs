use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Number(u8),
    List(Vec<Packet>),
}

fn compare_lists(left: &[Packet], right: &[Packet]) -> Option<bool> {
    use Packet::*;
    for (lv, rv) in left.iter().zip(right.iter()) {
        if let Some(res) = match (lv, rv) {
            (Number(ln), Number(rn)) => match ln.cmp(rn) {
                Ordering::Greater => Some(false),
                Ordering::Less => Some(true),
                Ordering::Equal => None,
            },
            (List(ll), List(rl)) => compare_lists(ll, rl),
            (List(ll), Number(rn)) => compare_lists(ll, &[Number(*rn)]),
            (Number(ln), List(rl)) => compare_lists(&[Number(*ln)], rl),
        } {
            return Some(res);
        }
    }
    match left.len().cmp(&right.len()) {
        Ordering::Greater => Some(false),
        Ordering::Less => Some(true),
        Ordering::Equal => None,
    }
}

fn main() {
    let input = include_str!("day_13_input.txt").trim();

    let pairs = input
        .split("\n\n")
        .map(|pair| {
            let mut pair_iter = pair.split('\n').map(|line| {
                let mut stack: Vec<Vec<Packet>> = Vec::new();
                let mut current: Vec<Packet> = Vec::new();
                let mut curval = None;
                for c in line[1..].chars() {
                    match c {
                        '[' => {
                            stack.push(current);
                            current = Vec::new();
                        }
                        ']' => {
                            if let Some(val) = curval {
                                current.push(Packet::Number(val));
                                curval = None;
                            }
                            if let Some(mut prev) = stack.pop() {
                                prev.push(Packet::List(current));
                                current = prev;
                            } else {
                                break;
                            }
                        }
                        ',' => {
                            if let Some(val) = curval {
                                current.push(Packet::Number(val));
                                curval = None;
                            }
                        }
                        _ => {
                            let v = c as u8 - b'0';
                            match curval.as_mut() {
                                None => {
                                    curval = Some(v);
                                }
                                Some(prev) => {
                                    *prev = (*prev * 10) + v;
                                }
                            }
                        }
                    }
                }
                current
            });
            let left = pair_iter.next().unwrap();
            let right = pair_iter.next().unwrap();
            (left, right)
        })
        .collect::<Vec<_>>();

    let part1 = pairs
        .iter()
        .enumerate()
        .filter_map(|(idx, (left, right))| {
            if compare_lists(left, right).unwrap() {
                Some(idx + 1)
            } else {
                None
            }
        })
        .sum::<usize>();
    println!("part1: {part1}");

    let mut items = Vec::with_capacity(pairs.len() * 2);
    for (left, right) in pairs {
        items.push(left);
        items.push(right);
    }
    let target2 = vec![Packet::List(vec![Packet::Number(2)])];
    let target6 = vec![Packet::List(vec![Packet::Number(6)])];
    items.push(target2.clone());
    items.push(target6.clone());

    items.sort_unstable_by(|left, right| match compare_lists(left, right) {
        Some(true) => Ordering::Less,
        Some(false) => Ordering::Greater,
        None => Ordering::Equal,
    });

    let mut idx2 = None;
    let mut idx6 = None;
    for (idx, item) in items.into_iter().enumerate() {
        if item == target2 {
            idx2 = Some(idx + 1);
            if idx6.is_some() {
                break;
            }
        } else if item == target6 {
            idx6 = Some(idx + 1);
            if idx2.is_some() {
                break;
            }
        }
    }
    let part2 = idx2.unwrap() * idx6.unwrap();
    println!("part2: {part2}");
}
