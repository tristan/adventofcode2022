enum Op {
    Noop,
    Addx(i64),
}

fn main() {
    let input = include_str!("day_10_input.txt").trim();

    let mut instructions = input.lines().map(|line| {
        if line == "noop" {
            Op::Noop
        } else {
            let (_, v) = line.split_once(' ').unwrap();
            Op::Addx(v.parse().unwrap())
        }
    });
    let mut x = 1;
    let mut op = None;
    let mut part1 = 0;
    let mut crt = Vec::with_capacity(6);
    let mut row = String::with_capacity(40);
    for cycle in 1.. {
        if (cycle - 20) % 40 == 0 {
            let strength = cycle * x;
            if cycle <= 220 {
                part1 += strength;
            }
        }
        if x == cycle % 40 || x + 1 == (cycle % 40) || x + 2 == (cycle % 40) {
            row.push('#');
        } else {
            row.push('.');
        }
        if row.len() == 40 {
            crt.push(row);
            row = String::with_capacity(40);
        }
        match op {
            None => match instructions.next() {
                Some(Op::Noop) => {}
                next_op @ Some(Op::Addx(_)) => {
                    op = next_op;
                }
                None => {
                    break;
                }
            },
            Some(Op::Addx(v)) => {
                x += v;
                op = None;
            }
            _ => unreachable!(),
        }
    }
    println!("part1: {part1}");
    println!("part2:");
    for row in crt {
        println!("{row}");
    }
}
