use std::cell::{Cell, RefCell};
use std::fs::read_to_string;
pub fn read_file(file_name: &str) -> Vec<String> {
    read_to_string(file_name)
        .expect("should read a file")
        .lines()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
}

pub fn read_file_cell(file_name: &str) -> Vec<RefCell<String>> {
    read_to_string(file_name)
        .expect("should read a file")
        .lines()
        .map(|x| RefCell::new(x.to_string()))
        .collect::<Vec<RefCell<String>>>()
}
