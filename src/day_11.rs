use std::collections::VecDeque;

#[derive(Clone)]
enum Val {
    Old,
    Val(u64),
}

#[derive(Clone)]
enum Op {
    Mul(Val),
    Add(Val),
}

#[derive(Clone)]
struct Monkey {
    items: VecDeque<u64>,
    op: Op,
    test: u64,
    if_true: usize,
    if_false: usize,
    inspected: u64,
}

fn main() {
    let input = include_str!("day_11_input.txt").trim();

    let initial_state = input
        .split("\n\n")
        .map(|monkey_data| {
            let mut data_iter = monkey_data.lines().skip(1);
            let items = data_iter.next().unwrap()[18..]
                .split(", ")
                .map(|num| num.parse().unwrap())
                .collect::<VecDeque<_>>();
            let (op, v) = data_iter.next().unwrap()[23..].split_once(' ').unwrap();
            let v = match v {
                "old" => Val::Old,
                _ => Val::Val(v.parse().unwrap()),
            };
            let op = match op {
                "*" => Op::Mul(v),
                "+" => Op::Add(v),
                _ => unreachable!(),
            };
            let test = data_iter.next().unwrap()[21..].parse().unwrap();
            let if_true = data_iter.next().unwrap()[29..].parse().unwrap();
            let if_false = data_iter.next().unwrap()[30..].parse().unwrap();
            Monkey {
                items,
                op,
                test,
                if_true,
                if_false,
                inspected: 0,
            }
        })
        .collect::<Vec<_>>();

    let mut monkeys = initial_state.clone();
    for _round in 0..20 {
        for idx in 0..monkeys.len() {
            while let Some(worry_level) = monkeys[idx].items.pop_front() {
                monkeys[idx].inspected += 1;
                let worry_level = match monkeys[idx].op {
                    Op::Mul(Val::Val(v)) => worry_level * v,
                    Op::Mul(Val::Old) => worry_level * worry_level,
                    Op::Add(Val::Val(v)) => worry_level + v,
                    Op::Add(Val::Old) => worry_level + worry_level,
                } / 3;
                let throw_idx = if worry_level % monkeys[idx].test == 0 {
                    monkeys[idx].if_true
                } else {
                    monkeys[idx].if_false
                };
                monkeys[throw_idx].items.push_back(worry_level);
            }
        }
    }

    let mut inspection_counts = monkeys.iter().map(|m| m.inspected).collect::<Vec<_>>();
    inspection_counts.sort_unstable();
    let part1: u64 = inspection_counts.into_iter().rev().take(2).product();
    println!("part1: {part1}");

    let mut monkeys = initial_state;
    let mut items = vec![];
    for monkey in &mut monkeys {
        let mut monkey_items = VecDeque::new();
        while let Some(worry_level) = monkey.items.pop_front() {
            monkey_items.push_back(items.len() as u64);
            items.push(worry_level);
        }
        monkey.items = monkey_items;
    }
    let mut items_per_monkey = vec![items; monkeys.len()];
    for (monkey, items) in monkeys.iter().zip(items_per_monkey.iter_mut()) {
        items.iter_mut().for_each(|i| *i %= monkey.test);
    }
    for _round in 0..10000 {
        for idx in 0..monkeys.len() {
            while let Some(worry_level_idx) = monkeys[idx].items.pop_front() {
                monkeys[idx].inspected += 1;
                for midx in 0..monkeys.len() {
                    let worry_level = items_per_monkey[midx][worry_level_idx as usize];
                    let worry_level = match monkeys[idx].op {
                        Op::Mul(Val::Val(v)) => worry_level * v,
                        Op::Mul(Val::Old) => worry_level * worry_level,
                        Op::Add(Val::Val(v)) => worry_level + v,
                        Op::Add(Val::Old) => worry_level + worry_level,
                    } % monkeys[midx].test;
                    items_per_monkey[midx][worry_level_idx as usize] = worry_level;
                }
                let worry_level = items_per_monkey[idx][worry_level_idx as usize];
                let throw_idx = if worry_level == 0 {
                    monkeys[idx].if_true
                } else {
                    monkeys[idx].if_false
                };
                monkeys[throw_idx].items.push_back(worry_level_idx);
            }
        }
    }

    let mut inspection_counts = monkeys.iter().map(|m| m.inspected).collect::<Vec<_>>();
    inspection_counts.sort_unstable();
    let part2: u64 = inspection_counts.into_iter().rev().take(2).product();
    println!("part2: {part2}");
}
