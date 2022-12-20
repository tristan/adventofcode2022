fn main() {
    let input = include_str!("day_20_input.txt").trim();

    let input = input
        .lines()
        .enumerate()
        .map(|(i, s)| (i, s.parse::<isize>().unwrap()))
        .collect::<Vec<_>>();

    let mut result_p1 = input.clone();
    for &(orig_idx, v) in &input {
        if v == 0 {
            continue;
        }
        let cur_idx = result_p1.iter().position(|&r| r == (orig_idx, v)).unwrap();
        let mut new_idx = (cur_idx as isize + v).rem_euclid(input.len() as isize - 1) as usize;
        if new_idx == 0 {
            new_idx = input.len() - 1;
        }
        let p = result_p1.remove(cur_idx);
        result_p1.insert(new_idx, p);
    }

    let zero_idx = result_p1.iter().position(|&r| r.1 == 0).unwrap();
    let part1 = [1000, 2000, 3000].iter().map(|&i| result_p1[(zero_idx + i) % result_p1.len()].1).sum::<isize>();
    println!("part1: {part1}");

    let mut result_p2 = input.clone();
    for _ in 0..10 {
        for &(orig_idx, v) in &input {
            if v == 0 {
                continue;
            }
            let cur_idx = result_p2.iter().position(|&r| r == (orig_idx, v)).unwrap();
            let mut new_idx = (cur_idx as isize + (v * 811589153)).rem_euclid(input.len() as isize - 1) as usize;
            if new_idx == 0 {
                new_idx = input.len() - 1;
            }
            let p = result_p2.remove(cur_idx);
            result_p2.insert(new_idx, p);
        }
    }
    let zero_idx = result_p2.iter().position(|&r| r.1 == 0).unwrap();
    let part2 = [1000, 2000, 3000].iter().map(|&i| result_p2[(zero_idx + i) % result_p2.len()].1 * 811589153).sum::<isize>();
    println!("part1: {part2}");
}
