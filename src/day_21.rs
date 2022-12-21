use std::collections::HashMap;

#[derive(Clone, Debug)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Clone, Debug)]
enum State {
    Number(i64),
    Math(&'static str, &'static str, Op)
}

fn main() {
    let input = include_str!("day_21_input.txt")
        .trim()
        .lines()
        .map(|line| {
            let (monkey, job) = line.split_once(": ").unwrap();
            let mut iter = job.split(' ');
            let i1 = iter.next().unwrap();
            if let Some(i2) = iter.next() {
                let i3 = iter.next().unwrap();
                (monkey, State::Math(i1, i3, match i2 {
                    "+" => Op::Add,
                    "-" => Op::Sub,
                    "*" => Op::Mul,
                    "/" => Op::Div,
                    _ => unreachable!(),
                }))
            } else {
                (monkey, State::Number(i1.parse().unwrap()))
            }
        })
        .collect::<HashMap<&'static str, State>>();
    {
        let mut input = input.clone();
        let part1 = 'outer: loop {
            let mut results = HashMap::new();
            for (&monkey, state) in &input {
                if let &State::Math(m1, m2, op) = &state {
                    if let (Some(&State::Number(mn1)), Some(&State::Number(mn2))) = (input.get(m1), input.get(m2)) {
                        let mn = match op {
                            Op::Add => mn1 + mn2,
                            Op::Sub => mn1 - mn2,
                            Op::Mul => mn1 * mn2,
                            Op::Div => mn1 / mn2,
                        };
                        if monkey == "root" {
                            break 'outer mn;
                        }
                        results.insert(monkey, State::Number(mn));
                    }
                }
            }
            input.extend(results);
        };
        println!("part1: {part1}");
    }
    {
        let mut input = input;
        let Some(State::Math(lhsm, rhsm, _)) = input.remove("root") else { unreachable!() };
        input.remove("humn");
        loop {
            let mut results = HashMap::new();
            for (&monkey, state) in &input {
                if let &State::Math(m1, m2, op) = &state {
                    if let (Some(&State::Number(mn1)), Some(&State::Number(mn2))) = (input.get(m1), input.get(m2)) {
                        let mn = match op {
                            Op::Add => mn1 + mn2,
                            Op::Sub => mn1 - mn2,
                            Op::Mul => mn1 * mn2,
                            Op::Div => mn1 / mn2,
                        };
                        results.insert(monkey, State::Number(mn));
                    }
                }
            }
            if results.is_empty() {
                break;
            }
            input.extend(results);
        }
        let (n, tosolve) = match (input.get(lhsm), input.get(rhsm)) {
            (Some(&State::Number(n)), Some(tosolve)) | (Some(tosolve), Some(&State::Number(n))) => {
                (n, tosolve)
            }
            _ => unreachable!()
        };
        let part2 = solve(&input, tosolve, n).unwrap();
        println!("part2: {part2}");
    }
}

fn solve(input: &HashMap<&str, State>, state: &State, n: i64) -> Option<i64> {
    let State::Math(lhs, rhs, op) = state else { return None; };
    let (tosolve, n) =match (input.get(lhs), input.get(rhs)) {
        (Some(&State::Number(m)), tosolve) => {
            let n = match op {
                // m + rhs = n
                // rhs = n - m
                Op::Add => n - m,
                // m - rhs = n
                // m = n + rhs
                // rhs = m - n
                Op::Sub => m - n,
                // m * rhs = n
                // rhs = n / m
                Op::Mul => n / m,
                // m / rhs = n
                // m = n * rhs
                // rhs = m / n
                Op::Div => m / n,
            };
            (tosolve, n)
        }
        (tosolve, Some(&State::Number(m))) => {
            let n = match op {
                // lhs + m = n
                // lhs = n - m
                Op::Add => n - m,
                // lhs - m = n
                // lhs = n + m
                Op::Sub => n + m,
                // lhs * m = n
                // lhs = n / m
                Op::Mul => n / m,
                // lhs / m = n
                // lhs = n * m
                Op::Div => n * m,
            };
            (tosolve, n)
        }
        _ => unreachable!()
    };
    match tosolve {
        Some(tosolve) => solve(input, tosolve, n),
        None => {
            Some(n)
        }
    }
}
