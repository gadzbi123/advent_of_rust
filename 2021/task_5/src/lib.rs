use read_file::read_file;
enum Direction {
    Diagonal,
    Row,
    Column,
    None,
}

fn solve_diagonal(
    board: &mut [Vec<i32>],
    mut x1: usize,
    mut y1: usize,
    mut x2: usize,
    mut y2: usize,
) {
    // go from bottom
    if y1 > y2 {
        std::mem::swap(&mut x1, &mut x2);
        std::mem::swap(&mut y1, &mut y2);
    }
    let calc_new: fn(usize) -> usize = if x1 > x2 {
        |x1: usize| x1.saturating_sub(1)
    } else {
        |x1: usize| x1.saturating_add(1)
    };

    for element in board.iter_mut().take(y2 + 1).skip(y1) {
        element[x1] += 1;
        x1 = calc_new(x1);
    }
}

pub fn task_5() {
    task_5_part(None);
    task_5_part(Some(true));
}

pub fn task_5_part(second: Option<bool>) {
    let lines = read_file("./data_files/file5.txt");
    let [mut max_x, mut max_y] = [0, 0];
    let mut sides: Vec<_> = Vec::new();
    for line in &lines {
        let side: Vec<Vec<usize>> = line
            .split(" -> ")
            .map(|x| {
                let res = x
                    .split(',')
                    .map(|cord| cord.parse::<usize>().expect("number"));
                res.clone().enumerate().for_each(|(i, v)| {
                    if (i % 2) == 0 {
                        max_x = max_x.max(v);
                    } else {
                        max_y = max_y.max(v);
                    }
                });
                res.collect::<Vec<usize>>()
            })
            .collect();
        sides.push(side)
    }
    let mut board = vec![vec![0; max_x + 1]; max_y + 1];
    for point in sides {
        if let [p1, p2] = &point[..] {
            let [mut x1, mut y1, mut x2, mut y2] = [
                *p1.as_slice().get(0).unwrap(),
                *p1.as_slice().get(1).unwrap(),
                *p2.as_slice().get(0).unwrap(),
                *p2.as_slice().get(1).unwrap(),
            ];
            let mut case = Direction::None;
            if x1 == x2 {
                case = Direction::Row
            }
            if y1 == y2 {
                case = Direction::Column
            }
            if (y1 as i32 - y2 as i32).abs() == (x1 as i32 - x2 as i32).abs() {
                case = Direction::Diagonal;
            }
            match case {
                Direction::Diagonal => {
                    if second.unwrap_or(false) {
                        solve_diagonal(&mut board, x1, y1, x2, y2)
                    }
                }
                Direction::Row => {
                    if y1 > y2 {
                        std::mem::swap(&mut y1, &mut y2)
                    }
                    for y in y1..=y2 {
                        board[y][x1] += 1;
                    }
                }
                Direction::Column => {
                    if x1 > x2 {
                        std::mem::swap(&mut x1, &mut x2)
                    }
                    for x in x1..=x2 {
                        board[y1][x] += 1;
                    }
                }
                Direction::None => (),
            }
        }
    }
    let count = board
        .into_iter()
        .fold(0, |acc, row| acc + row.iter().filter(|&&x| x > 1).count());
    println!("{}", count)
}
