use read_file;

#[derive(Debug, Clone, Copy)]
struct BingoPoint {
    value: u8,
    checked: bool,
}
impl BingoPoint {
    fn new(value: u8) -> BingoPoint {
        BingoPoint {
            value,
            checked: false,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Board {
    sum: i32,
    board: [[BingoPoint; 5]; 5],
}

impl Board {
    fn new() -> Board {
        Board {
            sum: 0,
            board: [[BingoPoint::new(0); 5]; 5],
        }
    }
    fn clear(&mut self) {
        self.sum = 0;
        self.board = [[BingoPoint::new(0); 5]; 5];
    }
    fn update(&mut self, index: usize, row: Vec<BingoPoint>) {
        let mut iter = row.iter();
        for x in self.board[index].as_mut() {
            if let Some(&value) = iter.next() {
                x.value = value.value;
                self.sum += value.value as i32;
            }
        }
    }
    fn check(&mut self, value: u8) {
        for row in self.board.as_mut() {
            let result = row.iter_mut().find(|&&mut x| x.value == value);
            if let Some(x) = result {
                x.checked = true;
                self.sum -= value as i32;
            }
        }
    }
    fn is_winner(&self) -> Option<i32> {
        let mut result = None;
        self.board.iter().for_each(|row| {
            if row.iter().all(|value| value.checked) {
                result = Some(self.sum);
            }
        });
        let transposed: Vec<Vec<BingoPoint>> = (0..5)
            .map(|col| (0..5).map(|row| self.board[row][col]).collect())
            .collect();
        transposed.iter().for_each(|row| {
            if row.iter().all(|value| value.checked) {
                result = Some(self.sum);
            }
        });
        // self.display();
        result
    }

    fn display(&self) {
        println!("board:");
        for row in self.board {
            for x in row {
                let res = if x.checked {
                    "x".to_string()
                } else {
                    x.value.to_string()
                };
                print!("{} ", res)
            }
            println!();
        }
    }
}

pub fn task_4() {
    let lines = read_file::read_file("data_files/file4.txt");
    let picks = lines
        .first()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u8>().expect("to be u8"))
        .collect::<Vec<u8>>();
    let mut boards: Vec<Board> = vec![];
    let mut curr_board: Board = Board::new();
    let mut index = 0;
    for line in lines.iter().skip(2).collect::<Vec<&String>>() {
        let row_str = &line.split_ascii_whitespace().collect::<Vec<&str>>()[..];
        match row_str {
            [] => {
                boards.push(curr_board);
                index = 0;
                curr_board.clear();
            }
            [a, b, c, d, e] => {
                let row = vec![a, b, c, d, e]
                    .iter()
                    .map(|x| BingoPoint::new(x.parse::<u8>().expect("to be number")))
                    .collect::<Vec<BingoPoint>>();
                curr_board.update(index, row);
                index += 1;
            }
            _ => (),
        }
    }
    task_4_1(&mut boards.clone(), &picks);
    task_4_2(boards, picks);
}

fn task_4_1(boards: &mut [Board], picks: &Vec<u8>) {
    let mut winning_sum = None;
    for pick in picks {
        for board in boards.iter_mut() {
            board.check(*pick);
            if let Some(sum) = board.is_winner() {
                winning_sum = Some(i32::max(sum, winning_sum.unwrap_or(0)));
            }
            // board.display();
        }
        if winning_sum.is_some() {
            println!("result_4_1={:?}", winning_sum.unwrap() * *pick as i32);
            break;
        }
    }
}

fn task_4_2(mut boards: Vec<Board>, picks: Vec<u8>) {
    let mut winning_sum_2 = None;
    for pick in picks {
        let mut poppers = vec![];
        for (i, board) in boards.iter_mut().enumerate() {
            board.check(pick);
            if let Some(sum) = board.is_winner() {
                winning_sum_2 = Some(sum * pick as i32);
                poppers.push(i);
            }
        }
        poppers.reverse();
        for p in poppers {
            boards.remove(p);
        }
    }
    println!("result_4_2={:?}", winning_sum_2.unwrap());
}

#[cfg(test)]
mod bingo_and_board_tests {
    use std::result;

    use crate::{BingoPoint, Board};

    #[test]
    fn bingo_point_new() {
        let result = BingoPoint::new(5);
        assert_eq!(result.value, 5);
        assert!(!result.checked);
    }

    #[test]
    fn board_size() {
        let result = Board::new();
        let (cols, rows) = (result.board.len(), result.board[0].len());
        assert_eq!(cols, 5);
        assert_eq!(rows, 5);
    }
}
