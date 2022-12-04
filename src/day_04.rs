fn main() {
    let input = include_str!("day_04_input.txt").trim();
    let (part1, part2) = input
        .split('\n')
        .fold((0, 0), |(contains, overlaps), line| {
            let (e1, e2) = line.split_once(',').unwrap();
            let (e1rs, e1re) = e1.split_once('-').unwrap();
            let (e2rs, e2re) = e2.split_once('-').unwrap();
            let e1rs = e1rs.parse::<u64>().unwrap();
            let e1re = e1re.parse::<u64>().unwrap();
            let e2rs = e2rs.parse::<u64>().unwrap();
            let e2re = e2re.parse::<u64>().unwrap();
            (
                contains
                    + ((e1rs <= e2rs && e1re >= e2re) || (e2rs <= e1rs && e2re >= e1re)) as usize,
                overlaps + (e1rs <= e2re && e2rs <= e1re) as usize,
            )
        });
    println!("part1: {part1}");
    println!("part2: {part2}");
}
