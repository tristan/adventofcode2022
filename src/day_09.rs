use std::collections::HashSet;

fn main() {
    let input = include_str!("day_09_input.txt").trim();
//     let input = r#"R 5
// U 8
// L 8
// D 3
// R 17
// D 10
// L 25
// U 20"#;
// //     let input = r#"R 4
// // U 4
// // L 3
// // D 1
// // R 4
// // D 1
// // L 5
// // R 2"#;

    let mut visited_1 = HashSet::new();
    let mut visited_2 = HashSet::new();
    visited_1.insert((0, 0));
    visited_2.insert((0, 0));
    input.lines().fold(((0i64, 0i64), vec![(0i64, 0i64); 9]), |((mut hx, mut hy), mut tail), line| {
        let (dir, dist) = line.split_once(' ').unwrap();
        let dist = dist.parse::<i64>().unwrap();
        let (dx, dy) = match dir {
            "L" => (-1, 0),
            "R" => (1, 0),
            "U" => (0, -1),
            "D" => (0, 1),
            _ => unreachable!()
        };
        for _ in 0..dist {
            hx += dx;
            hy += dy;
            let mut nx = hx;
            let mut ny = hy;
            for (idx, (tx, ty)) in tail.iter_mut().enumerate() {
                let dx = nx - *tx;
                let dy = ny - *ty;
                if dx.abs() > 1 || dy.abs() > 1 {
                    //print!("{idx}: {tx}+{dx} {ty}+{dy} => ");
                    //let is_diagonal = dx * dy != 0;
                    *tx += dx.signum();
                    *ty += dy.signum();
                    nx = *tx;
                    ny = *ty;
                    if idx == 0 {
                        visited_1.insert((*tx, *ty));
                    } else if idx == 8 {
                        visited_2.insert((*tx, *ty));
                    }
                } else {
                    // no more knots need to move
                    break;
                }
            }
            // println!("H: {hx},{hy}");
            // for (idx, (tx, ty)) in tail.iter().enumerate() {
            //     println!("{}: {tx},{ty}", idx + 1);
            // }
        }
        ((hx, hy), tail)
    });
    println!("part1: {}", visited_1.len());
    println!("part2: {}", visited_2.len());
}
