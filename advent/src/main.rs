use read_file::read_file;
use std::{
    borrow::BorrowMut,
    cell::{Cell, RefCell},
    ops::{Deref, DerefMut},
    rc::Rc,
};
// use task_1;
// use task_2;
// use task_3;
// use task_4;
enum Direction {
    Diagonal,
    Row,
    Column,
    None,
}

fn task_4_2() {
    let lines = read_file("./data_files/file5.txt");
    let [mut max_x, mut max_y] = [0, 0];
    let mut sides: Vec<_> = Vec::new();
    for line in &lines {
        let side = line
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
            .collect::<Vec<Vec<usize>>>();
        sides.push(side)
    }
    // println!("{:?}", sides);
    // println!("{},{}", max_x, max_y);
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
            if (y1 as i32 - y2 as i32).abs() == (x1 as i32 - x2 as i32).abs() {
                case = Direction::Diagonal;
            }
            if y1 == y2 {
                case = Direction::Column
            }
            match case {
                Direction::Diagonal => {
                    if y1 > y2 {
                        std::mem::swap(&mut x1, &mut x2);
                        std::mem::swap(&mut y1, &mut y2);
                    }
                    // println!("{} {},{} {}", x1, y1, x2, y2);
                    // lb -> rt
                    if x1 + y1 == x2 + y2 {
                        if x1 > x2 {
                            for y in y1..=y2 {
                                board[y][x1] += 1;
                                x1 = x1.saturating_sub(1)
                            }
                        } else {
                            for y in y1..=y2 {
                                board[y][x1] += 1;
                                x1 += 1;
                            }
                        };
                    }
                    // lt -> rb
                    else {
                        for y in y1..=y2 {
                            board[y][x1] += 1;
                            x1 += 1;
                        }
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
    let count = board.into_iter().fold(0, |acc, row| {
        let count = row.iter().filter(|&&x| x > 1).count();
        acc + count
    });
    println!("{}", count)
}
use undefined;
fn main() {
    undefined::mutate_array_ref_cell();
    // undefined::mutate_array_rc()
}

// fn main() {
//     let sides: Vec<_> = Vec::from([[[0, 1], [2, 3]], [[4, 5], [6, 7]]]);
//     let mut side = sides.iter();
//     while let Some(points) = side.next() {
//         match &points[..] {
//             [p1, p2] => match [p1[0..2], p2[0..2]] {
//                 [[x1, y1], [x2, y2]] => {}
//                 _ => (),
//             },
//             _ => (),
//         }
//     }
// }
