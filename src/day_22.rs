#[cfg(debug_assertions)]
use std::io::{Read, Write};

const GRID_WIDTH: usize = 150;
const GRID_HEIGHT: usize = 200;

#[derive(Clone, Copy, Debug)]
enum Move {
    Forward(usize),
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(debug_assertions)]
fn print_grid(grid: &Vec<Vec<Option<i32>>>, x: usize, y: usize, dir: Direction) {
    for (gy, row) in grid.iter().enumerate() {
        if (gy < y && y - gy > 10) || (gy > y && gy - y > 10) {
            continue;
        }
        for (gx, c) in row.iter().enumerate() {
            if gx == x && gy == y {
                match dir {
                    Direction::Up => print!("^"),
                    Direction::Down => print!("v"),
                    Direction::Left => print!("<"),
                    Direction::Right => print!(">"),
                }
            } else {
                match c {
                    Some(1) => print!("#"),
                    Some(0) => print!("."),
                    _ => print!(" "),
                }
            }
        }
        println!();
    }
    println!();
    wait_for_key_press();
}

#[cfg(debug_assertions)]
pub fn wait_for_key_press() {
    let mut stdout = std::io::stdout();
    stdout.write_all(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    std::io::stdin().read_exact(&mut [0]).unwrap();
    stdout.write_all(b"\n").unwrap();
}

fn main() {
    let input = include_str!("day_22_input.txt").trim_end();
    //let input = include_str!("day_22_test_input.txt").trim_end();
    let mut grid = Vec::with_capacity(GRID_HEIGHT);
    let mut directions = Vec::new();
    let mut parse_mode = 0;
    for line in input.lines() {
        if parse_mode == 0 && line.is_empty() {
            assert_eq!(grid.len(), GRID_HEIGHT);
            parse_mode = 1;
        } else if parse_mode == 0 {
            let mut row = line
                .chars()
                .map(|c| match c {
                    ' ' => None,
                    '.' => Some(0),
                    '#' => Some(1),
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>();
            if row.len() < GRID_WIDTH {
                row.extend(vec![None; GRID_WIDTH - row.len()]);
            }
            grid.push(row);
        } else {
            let last = line.chars().fold(0, |p, c| match c {
                'L' => {
                    directions.push(Move::Forward(p));
                    directions.push(Move::Left);
                    0
                }
                'R' => {
                    directions.push(Move::Forward(p));
                    directions.push(Move::Right);
                    0
                }
                _ => p * 10 + c.to_digit(10).unwrap() as usize,
            });
            if last > 0 {
                directions.push(Move::Forward(last));
            }
        }
    }

    let (mut x, mut y) = (grid[0].iter().position(|&v| v == Some(0)).unwrap(), 0);
    let mut dir = Direction::Right;
    //print_grid(&grid, x, y, dir);
    for &instruction in &directions {
        //println!("{:?}", instruction);
        match instruction {
            Move::Left => {
                dir = match dir {
                    Direction::Up => Direction::Left,
                    Direction::Right => Direction::Up,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Down,
                }
            }
            Move::Right => {
                dir = match dir {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                }
            }
            Move::Forward(steps) => match dir {
                Direction::Left => {
                    let Some(nx) = (0..x).rev().chain((x..GRID_WIDTH).rev()).cycle()
                        .filter(|&x| grid[y][x].is_some())
                        .take_while(|&x| grid[y][x] == Some(0))
                        .take(steps)
                        .last() else { continue };
                    x = nx;
                }
                Direction::Right => {
                    let Some(nx) = ((x + 1)..GRID_WIDTH).chain(0..=x).cycle()
                        .filter(|&x| grid[y][x].is_some())
                        .take_while(|&x| grid[y][x] == Some(0))
                        .take(steps)
                        .last() else {
                            continue
                        };
                    x = nx;
                }
                Direction::Up => {
                    let Some(ny) = (0..y).rev().chain((y..GRID_HEIGHT).rev()).cycle()
                        .filter(|&y| grid[y][x].is_some())
                        .take_while(|&y| grid[y][x] == Some(0))
                        .take(steps)
                        .last() else {
                            continue
                        };
                    y = ny;
                }
                Direction::Down => {
                    let Some(ny) = ((y + 1)..GRID_HEIGHT).chain(0..=y).cycle()
                        .filter(|&y| grid[y][x].is_some())
                        .take_while(|&y| grid[y][x] == Some(0))
                        .take(steps)
                        .last() else { continue };
                    y = ny;
                }
            },
        }
        //print_grid(&grid, x, y, dir);
    }

    let part1 = 1000 * (y + 1)
        + 4 * (x + 1)
        + match dir {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        };
    println!("part1: {part1}");

    let (mut x, mut y) = (grid[0].iter().position(|&v| v == Some(0)).unwrap(), 0);
    let mut dir = Direction::Right;
    //print_grid(&grid, x, y, dir);
    for &instruction in &directions {
        match instruction {
            Move::Left => {
                dir = match dir {
                    Direction::Up => Direction::Left,
                    Direction::Right => Direction::Up,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Down,
                };
                #[cfg(debug_assertions)]
                {
                    print!("\x1B[2J\x1B[1;1H");
                    println!("{:?}\n", instruction);
                    print_grid(&grid, x, y, dir);
                }
            }
            Move::Right => {
                dir = match dir {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                };
                #[cfg(debug_assertions)]
                {
                    print!("\x1B[2J\x1B[1;1H");
                    println!("{:?}\n", instruction);
                    print_grid(&grid, x, y, dir);
                }
            }
            Move::Forward(mut steps) => {
                while steps > 0 {
                    let (nx, ny, nd) = match dir {
                        Direction::Left => {
                            if (x % 50) == 0 {
                                // which grid to move to based on y
                                if y < 50 {
                                    if x < 100 {
                                        (0, 149 - y, Direction::Right)
                                    } else {
                                        (x - 1, y, dir)
                                    }
                                } else if y >= 50 && y < 100 {
                                    (y - 50, 100, Direction::Down)
                                } else if y >= 100 && y < 150 {
                                    if x < 50 {
                                        (50, 49 - (y - 100), Direction::Right)
                                    } else {
                                        (x - 1, y, dir)
                                    }
                                } else if y >= 150 {
                                    (y - 100, 0, Direction::Down)
                                } else {
                                    unreachable!()
                                }
                            } else {
                                (x - 1, y, dir)
                            }
                        }
                        Direction::Right => {
                            if (x % 50) == 49 {
                                if y < 50 {
                                    if x == 149 {
                                        (99, 149 - y, Direction::Left)
                                    } else {
                                        (x + 1, y, dir)
                                    }
                                } else if y >= 50 && y < 100 {
                                    (100 + (y - 50), 49, Direction::Up)
                                } else if y >= 100 && y < 150 {
                                    if x == 99 {
                                        (149, 49 - (y - 100), Direction::Left)
                                    } else {
                                        (x + 1, y, dir)
                                    }
                                } else if y >= 150 {
                                    (y - 100, 149, Direction::Up)
                                } else {
                                    unreachable!()
                                }
                            } else {
                                (x + 1, y, dir)
                            }
                        }
                        Direction::Up => {
                            if (y % 50) == 0 {
                                if x < 50 {
                                    if y < 150 {
                                        (50, 50 + x, Direction::Right)
                                    } else {
                                        (x, y - 1, dir)
                                    }
                                } else if x >= 50 && x < 100 {
                                    if y < 50 {
                                        (0, 150 + (x - 50), Direction::Right)
                                    } else {
                                        (x, y - 1, dir)
                                    }
                                } else if x >= 100 && x < 150 {
                                    (x - 100, 199, Direction::Up)
                                } else {
                                    unreachable!()
                                }
                            } else {
                                (x, y - 1, dir)
                            }
                        }
                        Direction::Down => {
                            if (y % 50) == 49 {
                                if x < 50 {
                                    if y == 199 {
                                        (100 + x, 0, Direction::Down)
                                    } else {
                                        (x, y + 1, dir)
                                    }
                                } else if x >= 50 && x < 100 {
                                    if y == 149 {
                                        (49, 150 + (x - 50), Direction::Left)
                                    } else {
                                        (x, y + 1, dir)
                                    }
                                } else if x >= 100 && x < 150 {
                                    (99, 50 + (x - 100), Direction::Left)
                                } else {
                                    unreachable!()
                                }
                            } else {
                                (x, y + 1, dir)
                            }
                        }
                    };
                    if matches!(grid[ny][nx], Some(0)) {
                        #[cfg(debug_assertions)]
                        {
                            print!("\x1B[2J\x1B[1;1H");
                            println!("{:?} ({})", instruction, steps);
                            println!("{x}->{nx} {y}->{ny} {:?}->{:?}", dir, nd);
                            print_grid(&grid, nx, ny, nd);
                        }
                        x = nx;
                        y = ny;
                        dir = nd;
                        steps -= 1;
                    } else {
                        #[cfg(debug_assertions)]
                        {
                            print!("\x1B[2J\x1B[1;1H");
                            println!("{:?} ({})", instruction, steps);
                            println!("{x}->{nx} {y}->{ny} {:?}->{:?} BLOCKED", dir, nd);
                            print_grid(&grid, x, y, dir);
                        }
                        break;
                    }
                }
            }
        }
    }

    let part2 = 1000 * (y + 1)
        + 4 * (x + 1)
        + match dir {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        };
    println!("part2: {part2}");
}
