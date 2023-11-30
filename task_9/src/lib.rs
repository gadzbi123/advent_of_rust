fn is_valley_for_point(can_read: bool, current: u8, next_to: u8) -> bool {
    if can_read && current >= next_to {
        return false;
    }
    true
}

pub fn task_9_1() {
    let lines = read_file_cell("./data_files/file9.txt");
    let mut valleys: Vec<u8> = Vec::new();
    for (i, r) in lines.iter().enumerate() {
        let ROW = r.borrow();
        for (j, c) in r.borrow().char_indices() {
            let current = c as u8;
            if i > 0 {
                let top = lines[i - 1].borrow().as_bytes()[j];
                if current >= top {
                    continue;
                }
            }
            if i < lines.len() - 1 {
                let bottom = lines[i + 1].borrow().as_bytes()[j];
                if current >= bottom {
                    continue;
                }
            }
            if j > 0 {
                let left = ROW.as_bytes()[j - 1];
                if current >= left {
                    continue;
                }
            }
            if j < ROW.len() - 1 {
                let right = ROW.as_bytes()[j + 1];
                if current >= right {
                    continue;
                }
            }
            valleys.push(current - 48);
        }
    }
    // dbg!(&valleys);
    let sum: u32 = valleys.iter().fold(0, |acc: u32, x| *x as u32 + 1 + acc);
    dbg!(sum);
}
