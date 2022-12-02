fn main() {
    let input = include_str!("day_02_input.txt").trim();
    let (part1, part2) = input.split('\n')
        .fold((0u64, 0u64), |scores, round| {
            let &[elf, _, you] = round.as_bytes() else { unreachable!() };
            let elf = (elf - b'A') + 1;
            let you = (you - b'X') + 1;
            (
                scores.0 + you as u64 + match (elf, you) {
                    // win conditions
                    (1, 2) | (2, 3) | (3, 1) => 6,
                    // loss conditions
                    (1, 3) | (2, 1) | (3, 2) => 0,
                    (a, b) if a == b => 3,
                    _ => unreachable!()
                },
                scores.1 + match (elf, you) {
                    // loss
                    (1, 1) => 3,
                    (2, 1) => 1,
                    (3, 1) => 2,
                    // win
                    (1, 3) => 2 + 6,
                    (2, 3) => 3 + 6,
                    (3, 3) => 1 + 6,
                    // draw
                    (_, 2) => elf as u64 + 3,
                    _ => unreachable!()
                }
            )
        });
    println!("part1: {part1}");
    println!("part2: {part2}");
}
