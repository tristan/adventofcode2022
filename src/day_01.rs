fn main() {
    let input = include_str!("day_01_input.txt").trim();
    let mut all = input
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .map(|item| item.parse::<u64>().unwrap())
                .sum::<u64>()
        })
        .collect::<Vec<_>>();
    all.sort_unstable_by(|a, b| { b.partial_cmp(a).unwrap() });
    println!("part1: {}", all.first().unwrap());
    println!("part1: {}", all.into_iter().take(3).sum::<u64>());
}
