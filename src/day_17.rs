use std::time::Duration;

enum Rock {
    HLine,
    Plus,
    Angle,
    VLine,
    Square,
}

impl Rock {
    fn next(&self) -> Rock {
        match self {
            Rock::HLine => Rock::Plus,
            Rock::Plus => Rock::Angle,
            Rock::Angle => Rock::VLine,
            Rock::VLine => Rock::Square,
            Rock::Square => Rock::HLine,
        }
    }

    fn generate(&self) -> Rocks {
        Rocks(match self {
            Rock::HLine => {
                vec![
                    [0, 0, 1, 1, 1, 1, 0]
                ]
            }
            Rock::Plus => {
                vec![
                    [0, 0, 0, 1, 0, 0, 0],
                    [0, 0, 1, 1, 1, 0, 0],
                    [0, 0, 0, 1, 0, 0, 0],
                ]
            }
            Rock::Angle => {
                vec![ // they are upside down, as 0 is the bottom
                    [0, 0, 1, 1, 1, 0, 0],
                    [0, 0, 0, 0, 1, 0, 0],
                    [0, 0, 0, 0, 1, 0, 0],
                ]
            }
            Rock::VLine => {
                vec![
                    [0, 0, 1, 0, 0, 0, 0],
                    [0, 0, 1, 0, 0, 0, 0],
                    [0, 0, 1, 0, 0, 0, 0],
                    [0, 0, 1, 0, 0, 0, 0],
                ]
            }
            Rock::Square => {
                vec![
                    [0, 0, 1, 1, 0, 0, 0],
                    [0, 0, 1, 1, 0, 0, 0],
                ]
            }
        })
    }
}

enum Push {
    Left,
    Right,
}

impl From<char> for Push {
    fn from(c: char) -> Self {
        match c {
            '<' => Push::Left,
            '>' => Push::Right,
            _ => unreachable!()
        }
    }
}

struct Rocks(Vec<[u8; 7]>);

impl Rocks {
    fn push(&mut self, direction: Push) {
        match direction {
            Push::Left => {
                if !self.0.iter().any(|r| r[0] == 1) {
                    for row in self.0.iter_mut() {
                        row.rotate_left(1);
                    }
                }
            }
            Push::Right => {
                if !self.0.iter().any(|r| r[6] == 1) {
                    for row in self.0.iter_mut() {
                        row.rotate_right(1);
                    }
                }
            }
        }
    }

    fn try_push(&mut self, direction: Push, cave: &[[u8; 7]]) {
        match direction {
            Push::Left => {
                if !self.0.iter().any(|r| r[0] == 1) {
                    for row in self.0.iter_mut() {
                        row.rotate_left(1);
                    }
                    // check if it now overlaps, and if so, rotate it back
                    if overlaps(&self.0, cave) {
                        for row in self.0.iter_mut() {
                            row.rotate_right(1);
                        }
                    }
                }
            }
            Push::Right => {
                if !self.0.iter().any(|r| r[6] == 1) {
                    for row in self.0.iter_mut() {
                        row.rotate_right(1);
                    }
                    // check if it now overlaps, and if so, rotate it back
                    if overlaps(&self.0, cave) {
                        for row in self.0.iter_mut() {
                            row.rotate_left(1);
                        }
                    }
                }
            }
        }
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

fn overlaps(a: &[[u8; 7]], b: &[[u8; 7]]) -> bool {
    a.iter().zip(b.iter()).any(|(&a, &b)| {
        a.iter().zip(b.iter()).any(|(&a, &b)| a == 1 && b == 1)
    })
}

fn merge(a: &mut [[u8; 7]], b: &[[u8; 7]]) {
    for (a, &b) in a.iter_mut().zip(b.iter()) {
        for (a, &b) in a.iter_mut().zip(b.iter()) {
            if b == 1 {
                *a = 1;
            }
        }
    }
}

#[allow(unused)]
fn print_board(cave: &Rocks, max_height: usize, rock: &Rocks, rock_top: usize) {
    let cave_top = cave.len();
    let rock_bottom = rock_top - (rock.len() - 1);
    print!("\x1B[2J\x1B[1;1H");
    println!("{cave_top}");
    for row in (0..=max_height).rev() {
        print!("|");
        if row >= cave_top && row > rock_top {
            // print empty space
            print!(".......");
        } else if row >= cave_top && row <= rock_top && row >= rock_bottom {
            // print only the rock
            let rockidx = (rock.len() - 1) - (rock_top - row);
            for &i in &rock.0[rockidx] {
                if i == 1 {
                    print!("@");
                } else {
                    print!(".");
                }
            }
        } else if row < rock_bottom && row >= cave_top {
            print!(".......");
        } else if (row > rock_top || row < rock_bottom) && row < cave_top {
            // print only the cave
            for &i in &cave.0[row] {
                if i == 1 {
                    print!("#");
                } else {
                    print!(".");
                }
            }
        } else {
            // print both
            let rockidx = (rock.len() - 1) - (rock_top - row);
            for (&r, &c) in rock.0[rockidx].iter().zip(cave.0[row].iter()) {
                if r == 1 {
                    print!("@");
                } else if c == 1 {
                    print!("#");
                } else {
                    print!(".");
                }
            }
        }
        println!("|");
    }
    println!("+-------+");
    std::thread::sleep(Duration::from_millis(500));
}

fn main() {
    let input = include_str!("day_17_input.txt").trim();
    //let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    let mut cave: Rocks = Rocks(vec![]);
    let mut rock_type = Rock::Square;
    let mut push_iter = input.chars().map(Push::from).cycle();
    for _ in 0..2022 {
        rock_type = rock_type.next();
        let mut rock = rock_type.generate();
        let mut rock_top = 3 + cave.len() + (rock.len() - 1);
        //let max_height = rock_top;
        //print_board(&cave, max_height, &rock, rock_top);
        // push 4 (yes 4) times to get started
        rock.push(push_iter.next().unwrap());
        //print_board(&cave, max_height, &rock, rock_top);
        rock_top -= 1;
        //print_board(&cave, max_height, &rock, rock_top);
        rock.push(push_iter.next().unwrap());
        //print_board(&cave, max_height, &rock, rock_top);
        rock_top -= 1;
        //print_board(&cave, max_height, &rock, rock_top);
        rock.push(push_iter.next().unwrap());
        //print_board(&cave, max_height, &rock, rock_top);
        rock_top -= 1;
        //print_board(&cave, max_height, &rock, rock_top);
        rock.push(push_iter.next().unwrap());
        //print_board(&cave, max_height, &rock, rock_top);
        let mut rock_bottom = rock_top - (rock.len() - 1);
        loop {
            // attempt to move down
            if rock_bottom == 0 || overlaps(&rock.0, &cave.0[rock_bottom - 1..cave.len()]) {
                // cannot move down! extend cave with rock
                // merge to cave top
                let cave_top = cave.len();
                merge(&mut cave.0[rock_bottom..cave_top], &rock.0);
                // append remaining rock
                if rock_top >= cave_top {
                    let remaining_rock_offset = rock.len() - (cave_top - rock_bottom);
                    cave.0.extend_from_slice(&rock.0[rock.len() - remaining_rock_offset..rock.len()]);
                }
                // then spawn next rock
                //return;
                break;
            }
            // move down
            rock_top -= 1;
            rock_bottom -= 1;
            //print_board(&cave, max_height, &rock, rock_top);
            rock.try_push(push_iter.next().unwrap(), &cave.0[rock_bottom..cave.len()]);
            //print_board(&cave, max_height, &rock, rock_top);
        }
    }
    let part1 = cave.len();
    println!("part1: {part1}");
}
