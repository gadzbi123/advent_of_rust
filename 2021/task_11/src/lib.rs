use std::collections::HashSet;
use std::fs::read;

pub fn task_11() {
    let mut state = read("./data_files/file11.txt")
        .expect("No file found")
        .split(|&x| x == b'\n')
        .map(|row| {
            row.iter()
                .map(|&c| (c as char).to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    let part1 = (0..100).fold(0, |acc, _| acc + step(&mut state));
    dbg!(part1);
    let mut state = read("./data_files/file11.txt")
        .expect("No file found")
        .split(|&x| x == b'\n')
        .map(|row| {
            row.iter()
                .map(|&c| (c as char).to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    let part2 = (1..).find(|_| step(&mut state) == 100).unwrap();
    dbg!(part2);
}
fn step(state: &mut Vec<Vec<u32>>) -> usize {
    let mut flashed = HashSet::new();
    let width = state[0].len();
    let height = state.len();
    for x in 0..height {
        for y in 0..width {
            state[x][y] += 1;
            if state[x][y] > 9 {
                flashed.insert((x as i32, y as i32));
            }
        }
    }
    for (x, y) in flashed.clone().iter() {
        flash(state, &mut flashed, *x, *y);
    }
    for (x, y) in flashed.iter() {
        state[*x as usize][*y as usize] = 0;
    }
    flashed.len()
}

fn flash(state: &mut Vec<Vec<u32>>, flashed: &mut HashSet<(i32, i32)>, x: i32, y: i32) {
    let width = state[0].len() as i32;
    let height = state.len() as i32;
    for new_x in [x - 1, x, x + 1] {
        for new_y in [y - 1, y, y + 1] {
            if new_x < 0 || new_y < 0 || new_x >= height || new_y >= width {
                continue;
            }
            if x == new_x && y == new_y {
                continue;
            }
            if flashed.contains(&(new_x, new_y)) {
                continue;
            }
            state[new_x as usize][new_y as usize] += 1;
            if state[new_x as usize][new_y as usize] > 9 {
                flashed.insert((new_x, new_y));
                flash(state, flashed, new_x, new_y);
            }
        }
    }
}
