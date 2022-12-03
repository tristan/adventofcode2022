fn char_to_priority(c: char) -> u64 {
    let i1 = c as u64;
    if i1 < 97 {
        i1 - 64 + 26
    } else {
        i1 - 96
    }
}

fn main() {
    let input = include_str!("day_03_input.txt").trim();
    let part1 = input.lines()
        .flat_map(|rucksack| {
            let split_at = rucksack.len() / 2;
            let (c1, c2) = rucksack.split_at(split_at);
            let mut common = vec![];
            for i in c1.chars() {
                if c2.contains(i) {
                    let priority = char_to_priority(i);
                    if !common.contains(&priority) {
                        common.push(priority);
                    }
                }
            }
            common
        })
        .sum::<u64>();
    println!("part1: {part1}");
    let (_, _, part2) = input.lines().fold((None, None, 0), |prev, rucksack| {
        match prev {
            (None, None, sum) => (Some(rucksack), None, sum),
            (Some(a), None, sum) => (Some(a), Some(rucksack), sum),
            (Some(a), Some(b), sum) => {
                for i in a.chars() {
                    if b.contains(i) && rucksack.contains(i) {
                        let priority = char_to_priority(i);
                        return (None, None, sum + priority);
                    }
                }
                unreachable!();
            },
            _ => unreachable!()
        }
    });
    println!("part2: {part2}");
}
