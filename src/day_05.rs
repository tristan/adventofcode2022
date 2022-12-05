fn main() {
//     let input = r#"    [D]
// [N] [C]
// [Z] [M] [P]
//  1   2   3

// move 1 from 2 to 1
// move 3 from 1 to 3
// move 2 from 2 to 1
    // move 1 from 1 to 2"#;
    let input = include_str!("day_05_input.txt").trim_end();

    let (board, moves) = input.split_once("\n\n").unwrap();
    let mut stacks = vec![];
    board.lines().rev().skip(1).for_each(|line| {
        (0..line.len()).step_by(4).enumerate().for_each(|(sidx, iidx)| {
            let c = &line[iidx + 1..iidx + 2];
            if c != " " {
                let stack = match stacks.get_mut(sidx) {
                    Some(v) => v,
                    None => {
                        stacks.push(vec![]);
                        stacks.get_mut(sidx).unwrap()
                    }
                };
                stack.push(c);
            }
        });
    });
    let mut p1stacks = stacks.clone();
    let mut p2stacks = stacks.clone();
    for line in moves.lines() {
        let mut iter = line.split(' ').skip(1).step_by(2).map(|p| p.parse::<usize>().unwrap());
        let amount = iter.next().unwrap();
        let from = iter.next().unwrap() - 1;
        let to = iter.next().unwrap() - 1;

        let mut new_stacks = vec![];
        for _ in 0..amount {
            if let Some(c) = p1stacks[from].pop() {
                p1stacks[to].push(c);
            }
            if let Some(c) = p2stacks[from].pop() {
                new_stacks.push(c);
            }
        }
        while let Some(c) = new_stacks.pop() {
            p2stacks[to].push(c);
        }
    }
    print!("part1: ");
    for stack in &mut p1stacks {
        if let Some(c) = stack.pop() {
            print!("{c}");
        }
    }
    println!();
    print!("part2: ");
    for stack in &mut p2stacks {
        if let Some(c) = stack.pop() {
            print!("{c}");
        }
    }
    println!();
}
