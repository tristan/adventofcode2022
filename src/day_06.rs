fn solve(input: &str, len: usize) -> usize {
   input.as_bytes().windows(len).enumerate().find_map(|(idx, window)| {
        for i in 0..window.len() {
            for j in i + 1..window.len() {
                if window[i] == window[j] {
                    return None
                }
            }
        }
        Some(idx + len)
    }).unwrap()
}

fn main() {
    let input = include_str!("day_06_input.txt").trim();

    let part1 = solve(input, 4);
    let part2 = solve(input, 14);
    println!("part1: {part1}");
    println!("part2: {part2}");
}
